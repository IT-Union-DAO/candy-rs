use crate::value::CandyValue;

pub type DataChunk = CandyValue;
pub type DataZone = Vec<DataChunk>;
pub type Workspace = Vec<DataZone>;

pub type AddressedChunk = (u128, u128, CandyValue);
pub type AddressedChunkArray = Vec<AddressedChunk>;

pub trait AddressedChunkArrayTrait {
    fn get_addressed_chunk_array_size(self) -> u128;
    fn get_data_chunk(self, data_zone: u128, data_chunk: u128) -> CandyValue;
    fn flatten(self) -> Vec<u8>;
}

pub trait DataZoneTrait {
    fn get_data_zone_size(self) -> u128;
    fn to_bytes_buffer(self) -> Vec<Vec<u8>>;
    fn from(bytes_buffer: Vec<Vec<u8>>) -> Self;
    fn init(self) -> DataZone;
}

pub trait WorkspaceTrait {
    fn to_addressed_chunk_array(self) -> AddressedChunkArray;
    fn from_addressed_chunks(chunks: AddressedChunkArray) -> Self;
    fn get_workspace_chunk_size(self, max_chunk_size: u128) -> u128;
    fn get_workspace_chunk(self, chunk_id: u128, max_chunk_size: u128) -> Vec<DataZone>;
}

impl DataZoneTrait for DataZone {
    fn get_data_zone_size(self) -> u128 {
        todo!()
    }

    fn to_bytes_buffer(self) -> Vec<Vec<u8>> {
        todo!()
    }

    fn from(bytes_buffer: Vec<Vec<u8>>) -> Self {
        todo!()
    }

    fn init(self) -> DataZone {
        todo!()
    }
}

impl AddressedChunkArrayTrait for AddressedChunkArray {
    fn get_addressed_chunk_array_size(self) -> u128 {
        todo!()
    }

    fn get_data_chunk(self, data_zone: u128, data_chunk: u128) -> CandyValue {
        todo!()
    }

    fn flatten(self) -> Vec<u8> {
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
