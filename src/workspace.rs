use candid::{CandidType, Encode};
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};

use crate::value::{CandyShared, ToCandyValue};

/*
   DataChunk is a type alias of CandyShared that represents a single chunk of data.
*/
pub type DataChunk = CandyShared;
/*
   DataZone is a type alias of Vec<DataChunk> that represents a contiguous sequence of data.
*/
pub type DataZone = Vec<DataChunk>;
/*
   Workspace is a type alias of Vec<DataZone> that represents a workspace containing multiple contiguous sequences of data.
*/
pub type Workspace = Vec<DataZone>;

/*
   A tuple consisting of an address, a length, and a CandyShared instance representing a single chunk of data
*/
pub type AddressedChunk = (u128, u128, CandyShared);
/*
   A vector of AddressedChunk instances representing a contiguous sequence of chunks of data
*/
pub type AddressedChunkArray = Vec<AddressedChunk>;

/*
Enumeration that specifies the type of chunk retrieved from workspace, which can be either Eof (for end-of-file) or Chunk (partial)
*/
#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum ChunkingType {
    #[serde(rename = "eof")]
    Eof,
    #[serde(rename = "chunk")]
    Chunk,
}

/// A trait for operations on `AddressedChunkArray`, which is a vector of `AddressedChunk` instances representing a contiguous sequence of chunks of data.
pub trait AddressedChunkArrayTrait {
    /// Get the total size of the `AddressedChunkArray`.
    fn get_addressed_chunk_array_size(&self) -> u128;

    /// Get the data chunk from the specified `DataZone` and `DataChunk`.
    ///
    /// # Arguments
    ///
    /// * `data_zone` - The index of the `DataZone` containing the desired data chunk.
    /// * `data_chunk` - The index of the desired data chunk within the specified `DataZone`.
    ///
    /// # Returns
    ///
    /// The `CandyShared` instance representing the desired data chunk, or a `CandyShared::Option(None)` instance if the data chunk is not found.
    fn get_data_chunk(&self, data_zone: u128, data_chunk: u128) -> CandyShared;

    /// Flatten the `AddressedChunkArray` into a byte vector.
    fn flatten(self) -> Vec<u8>;
}

pub trait DataZoneTrait {
    fn get_data_zone_size(&self) -> u128;
    fn to_bytes_buffer(self) -> Vec<Vec<u8>>;
    fn from_buffer(bytes_buffer: Vec<Vec<u8>>) -> Self;
}

/// The `WorkspaceTrait` trait provides methods for working with a workspace, which is a vector of `DataZone` instances.
pub trait WorkspaceTrait {
    // Counts the total number of addressed chunks in the workspace.
    ///
    /// # Returns
    ///
    /// * A `u128` representing the count of addressed chunks.
    fn count_addressed_chunks(&self) -> u128;
    /// Converts the workspace into an `AddressedChunkArray`.
    ///
    /// # Returns
    ///
    /// * An `AddressedChunkArray` representing the workspace.
    fn to_addressed_chunk_array(self) -> AddressedChunkArray;
    /// Creates a workspace from an `AddressedChunkArray`.
    ///
    /// # Arguments
    ///
    /// * `chunks` - An `AddressedChunkArray` representing the workspace.
    ///
    /// # Returns
    ///
    /// * A `Workspace` instance.
    fn from_addressed_chunks(chunks: AddressedChunkArray) -> Self;
    /// Gets the number of chunks a workspace will be split into given a max chunk size (bytes).
    ///
    /// # Arguments
    ///
    /// * `max_chunk_size` - A `u128` representing the maximum chunk size in bytes.
    ///
    /// # Returns
    ///
    /// * A `u128` representing the number of chunks.
    fn get_workspace_chunk_size(self, max_chunk_size: u128) -> u128;

    /// Gets the chunk at a given index in the workspace.
    ///
    /// # Arguments
    ///
    /// * `chunk_id` - A `u128` representing the index of the chunk to retrieve.
    /// * `max_chunk_size` - A `u128` representing the maximum chunk size in bytes.
    ///
    /// # Returns
    ///
    /// * A tuple containing a `ChunkingType` indicating whether the chunk is a regular chunk or the end-of-file chunk, and an `AddressedChunkArray` representing the chunk.
    fn get_workspace_chunk(
        self,
        chunk_id: u128,
        max_chunk_size: u128,
    ) -> (ChunkingType, AddressedChunkArray);
}

impl DataZoneTrait for DataZone {
    ///  Get DataZone size of CandyShared values
    /// ```
    /// use ic_candy::value::CandyShared;
    /// use ic_candy::workspace::DataZone;
    /// use crate::ic_candy::workspace::DataZoneTrait;
    /// use crate::ic_candy::value::ToCandyValue;
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

    /// Convert the DataZone to a vector of byte vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_candy::value::{CandyShared, ToCandyValue};
    /// use ic_candy::workspace::DataZone;
    /// use crate::ic_candy::workspace::DataZoneTrait;
    ///
    /// let dz: DataZone = vec![42_u128.to_candy()];
    /// let buffer = dz.to_bytes_buffer();
    /// assert_eq!(buffer, vec![vec![42]]);
    /// ```
    fn to_bytes_buffer(self) -> Vec<Vec<u8>> {
        self.into_iter()
            .map(DataChunk::to_blob)
            .collect::<Vec<Vec<u8>>>()
    }

    /// Convert the DataZone from a vector of byte vectors.
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
