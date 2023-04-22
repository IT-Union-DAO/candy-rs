use candid::{CandidType, Encode};
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};

use crate::value::{CandyShared, ToCandyValue};

pub type DataChunk = CandyShared;
pub type DataZone = Vec<DataChunk>;
pub type Workspace = Vec<DataZone>;

pub type AddressedChunk = (u128, u128, CandyShared);
pub type AddressedChunkArray = Vec<AddressedChunk>;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum ChunkingType {
    #[serde(rename = "eof")]
    Eof,
    #[serde(rename = "chunk")]
    Chunk,
}

pub trait AddressedChunkArrayTrait {
    fn get_addressed_chunk_array_size(&self) -> u128;
    fn get_data_chunk(&self, data_zone: u128, data_chunk: u128) -> CandyShared;
    fn flatten(self) -> Vec<u8>;
}

pub trait DataZoneTrait {
    fn get_data_zone_size(&self) -> u128;
    fn to_bytes_buffer(self) -> Vec<Vec<u8>>;
    fn from_buffer(bytes_buffer: Vec<Vec<u8>>) -> Self;
}

pub trait WorkspaceTrait {
    fn count_addressed_chunks(&self) -> u128;
    fn to_addressed_chunk_array(self) -> AddressedChunkArray;
    fn from_addressed_chunks(chunks: AddressedChunkArray) -> Self;
    fn get_workspace_chunk_size(self, max_chunk_size: u128) -> u128;
    fn get_workspace_chunk(
        self,
        chunk_id: u128,
        max_chunk_size: u128,
    ) -> (ChunkingType, AddressedChunkArray);
}

impl DataZoneTrait for DataZone {
    ///  Get DataZone size of CandyShared values
    /// ```
    /// use candy::value::CandyShared;
    /// use candy::workspace::DataZone;
    /// use crate::candy::workspace::DataZoneTrait;
    /// use crate::candy::value::ToCandyValue;
    /// use candid::Principal;
    ///
    ///
    /// let dz : DataZone = vec![
    /// 16_u128.to_candy(),
    /// Principal::from_text("aaaaa-aa").unwrap().to_candy()
    /// ];
    /// assert_eq!(dz.get_data_zone_size(), 197);
    /// ```
    fn get_data_zone_size(&self) -> u128 {
        Encode!(self).unwrap().len() as u128
    }

    fn to_bytes_buffer(self) -> Vec<Vec<u8>> {
        self.into_iter()
            .map(DataChunk::to_blob)
            .collect::<Vec<Vec<u8>>>()
    }

    fn from_buffer(bytes_buffer: Vec<Vec<u8>>) -> Self {
        bytes_buffer.into_iter().map(CandyShared::Bytes).collect()
    }
}

impl AddressedChunkArrayTrait for AddressedChunkArray {
    fn get_addressed_chunk_array_size(&self) -> u128 {
        Encode!(self).unwrap().len() as u128
    }

    fn get_data_chunk(&self, data_zone: u128, data_chunk: u128) -> CandyShared {
        for chunk in self {
            if chunk.0 == data_zone && chunk.1 == data_chunk {
                return chunk.2.clone();
            }
        }
        CandyShared::Option(None)
    }

    fn flatten(self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        for item in self {
            res.append(item.0.to_candy().to_blob().as_mut());
            res.extend_from_slice(item.1.to_candy().to_blob().as_mut());
            res.extend_from_slice(item.2.to_blob().as_mut());
        }
        res
    }
}

impl WorkspaceTrait for Workspace {
    fn count_addressed_chunks(&self) -> u128 {
        self.iter().map(|zone| zone.len() as u128).sum()
    }

    fn to_addressed_chunk_array(self) -> AddressedChunkArray {
        let mut result: AddressedChunkArray = Vec::new();
        for (zone_index, zone) in self.into_iter().enumerate() {
            for (chunk_index, chunk) in zone.into_iter().enumerate() {
                result.push((zone_index as u128, chunk_index as u128, chunk));
            }
        }
        result
    }

    fn from_addressed_chunks(chunks: AddressedChunkArray) -> Self {
        let mut ws = vec![
            Vec::new();
            chunks
                .iter()
                .map(|chunk| chunk.0 + 1)
                .max()
                .unwrap_or(0)
                .to_usize()
                .unwrap()
        ];

        for chunk in chunks {
            let current_zone = &mut ws[chunk.0 as usize];

            if (chunk.1 + 1) <= (current_zone.len() as u128) {
                current_zone.insert(chunk.1 as usize, chunk.2);
            } else {
                current_zone.resize_with((chunk.1 + 1) as usize, || CandyShared::Option(None));
                current_zone[chunk.1 as usize] = chunk.2;
            }
        }
        ws
    }

    /// Gets the number of chunks a workspace will be split into given a max chunk size
    fn get_workspace_chunk_size(self, max_chunk_size: u128) -> u128 {
        let mut current_chunk: u128 = 0;

        let mut found_bytes = 0_u128;
        for data_zone in self {
            'inner: for data_chunk in data_zone {
                let new_size = found_bytes + data_chunk.get_value_size();
                if new_size > max_chunk_size {
                    current_chunk += 1;
                    found_bytes = 0;
                    continue 'inner;
                }
                found_bytes = new_size;
            }
        }
        current_chunk + 1
    }

    fn get_workspace_chunk(
        self,
        chunk_id: u128,
        max_chunk_size: u128,
    ) -> (ChunkingType, AddressedChunkArray) {
        let mut current_chunk: u128 = 0;

        let mut result_buffer: AddressedChunkArray = Vec::new();

        let mut found_bytes = 0_u128;
        for (zone_index, data_zone) in self.iter().enumerate() {
            'inner: for (chunk_index, data_chunk) in data_zone.iter().enumerate() {
                let new_size = found_bytes + data_chunk.get_value_size();
                if new_size > max_chunk_size {
                    if current_chunk == chunk_id {
                        return (ChunkingType::Chunk, result_buffer);
                    }
                    current_chunk += 1;
                    found_bytes = 0;
                    continue 'inner;
                }
                if current_chunk == chunk_id {
                    result_buffer.push((
                        zone_index as u128,
                        chunk_index as u128,
                        data_chunk.clone(),
                    ));
                }
                found_bytes = new_size;
            }
        }
        (ChunkingType::Eof, result_buffer)
    }
}
