use crate::types::Bytes;
use crate::value::CandyValue;
use num_traits::AsPrimitive;
use std::ops::Add;

pub type DataChunk = CandyValue;
pub type DataZone = Vec<DataChunk>;
pub type Workspace = Vec<DataZone>;

pub type AddressedChunk = (u128, u128, CandyValue);
pub type AddressedChunkArray = Vec<AddressedChunk>;

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
    fn to_addressed_chunk_array(self) -> AddressedChunkArray;
    fn from_addressed_chunks(chunks: AddressedChunkArray) -> Self;
    fn get_workspace_chunk_size(self, max_chunk_size: u128) -> u128;
    fn get_workspace_chunk(self, chunk_id: u128, max_chunk_size: u128) -> Vec<DataZone>;
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
        // let res: Vec<u8> = Vec::with_capacity(self.len());
        // for item in self {
        //     for byte in  {
        //         res.add(byte);
        //     }
        // }
        // res
        todo!()
    }
}

impl WorkspaceTrait for Workspace {
    fn to_addressed_chunk_array(self) -> AddressedChunkArray {
        todo!()
    }

    fn from_addressed_chunks(chunks: AddressedChunkArray) -> Self {
        todo!()
    }

    fn get_workspace_chunk_size(self, max_chunk_size: u128) -> u128 {
        todo!()
    }

    fn get_workspace_chunk(self, chunk_id: u128, max_chunk_size: u128) -> Vec<DataZone> {
        todo!()
    }
}
