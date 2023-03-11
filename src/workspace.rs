use crate::types::Bytes;
use crate::value::CandyValue;
use candid::CandidType;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::ops::Add;
pub type DataChunk = CandyValue;
pub type DataZone = Vec<DataChunk>;
pub type Workspace = Vec<DataZone>;

pub type AddressedChunk = (u128, u128, CandyValue);
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
    fn get_data_chunk(&self, data_zone: u128, data_chunk: u128) -> CandyValue;
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
    fn get_data_zone_size(&self) -> u128 {
        self.iter()
            .map(|chunk| chunk.get_value_size())
            .fold(0, |acc, x| acc.add(x))
    }

    fn to_bytes_buffer(self) -> Vec<Vec<u8>> {
        self.into_iter()
            .map(|chunk| chunk.to_blob())
            .collect::<Vec<Vec<u8>>>()
    }

    fn from_buffer(bytes_buffer: Vec<Vec<u8>>) -> Self {
        bytes_buffer
            .into_iter()
            .map(|vec| CandyValue::Bytes(Bytes::frozen(vec)))
            .collect()
    }
}

impl AddressedChunkArrayTrait for AddressedChunkArray {
    fn get_addressed_chunk_array_size(&self) -> u128 {
        std::mem::size_of_val(self) as u128
    }

    fn get_data_chunk(&self, data_zone: u128, data_chunk: u128) -> CandyValue {
        for chunk in self {
            if chunk.0 == data_zone && chunk.1 == data_chunk {
                return chunk.2.clone();
            }
        }
        CandyValue::Empty
    }

    fn flatten(self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::with_capacity(self.len());
        for item in self {
            res.extend_from_slice(&item.0.to_be_bytes());
            res.extend_from_slice(&item.1.to_be_bytes());
            res.extend_from_slice(&item.2.to_blob());
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
        let mut ws: Workspace = Vec::new();
        for chunk in chunks {
            let result_size = ws.len() as u128;
            let target_zone = chunk.0 + 1;
            if target_zone > result_size {
                for _ in result_size..target_zone {
                    ws.push(Vec::new());
                }
            }
            let current_zone = ws.get_mut(chunk.0.to_usize().unwrap()).unwrap();

            if chunk.1 + 1 < current_zone.len() as u128 {
                let index = chunk.1.to_usize().unwrap();
                current_zone.insert(index, chunk.2);
            } else {
                for new_chunk_index in current_zone.len()..chunk.1.to_usize().unwrap() {
                    let new_buffer = if chunk.1.to_usize().unwrap() == new_chunk_index {
                        chunk.2.clone()
                    } else {
                        CandyValue::Empty
                    };
                    current_zone.push(new_buffer);
                }
            }
        }
        ws
    }

    fn get_workspace_chunk_size(self, max_chunk_size: u128) -> u128 {
        let mut current_chunk: u128 = 0;
        let mut handbreak: u128 = 0;
        let mut zone_tracker: u128 = 0;
        let mut chunk_tracker: u128 = 0;

        loop {
            handbreak += 1;
            if handbreak > 10_000 {
                break;
            };
            let mut found_bytes = 0_u128;
            for zone_index in zone_tracker..self.len().to_u128().unwrap() {
                for chunk_index in chunk_tracker
                    ..self
                        .get(zone_index.to_usize().unwrap())
                        .unwrap()
                        .len()
                        .to_u128()
                        .unwrap()
                {
                    let current_item = self
                        .get(zone_index.to_usize().unwrap())
                        .unwrap()
                        .get(chunk_index.to_usize().unwrap())
                        .unwrap();
                    let new_size = found_bytes + current_item.get_value_size();

                    if new_size > max_chunk_size {
                        current_chunk += 1;
                        zone_tracker = zone_index;
                        chunk_tracker = chunk_index;
                    }
                    found_bytes = new_size;
                }
            }
        }
        current_chunk += 1;
        current_chunk
    }

    fn get_workspace_chunk(
        self,
        chunk_id: u128,
        max_chunk_size: u128,
    ) -> (ChunkingType, AddressedChunkArray) {
        let mut current_chunk: u128 = 0;
        let mut handbreak = 0;
        let mut zone_tracker = 0;
        let mut chunk_tracker = 0;

        let mut result_buffer: AddressedChunkArray = Vec::new();

        loop {
            handbreak += 1;
            if handbreak > 10_000 {
                break;
            }
            let mut found_bytes = 0;

            for zone_index in zone_tracker..self.len() {
                for chunk_index in chunk_tracker..self.get(zone_index).unwrap().len() {
                    let current_item = self.get(zone_index).unwrap().get(chunk_index).unwrap();
                    let new_size = found_bytes + current_item.get_value_size();
                    if new_size > max_chunk_size {
                        if (chunk_index as u128) == chunk_id {
                            return (ChunkingType::Chunk, result_buffer);
                        }
                        current_chunk += 1;
                        zone_tracker = zone_index;
                        chunk_tracker = chunk_index;
                    }
                    if (chunk_index as u128) == chunk_id {
                        result_buffer.push((
                            zone_index as u128,
                            chunk_index as u128,
                            current_item.clone(),
                        ))
                    }
                    found_bytes = new_size;
                }
            }
        }

        (ChunkingType::Eof, result_buffer)
    }
}
