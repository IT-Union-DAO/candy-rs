use candid::candid_method;
use candid::Principal;
use ic_cdk_macros::{query, update};

use candy::value::{CandyShared, ToCandyValue};
use candy::workspace::{
    AddressedChunkArray, AddressedChunkArrayTrait, ChunkingType, DataZone, DataZoneTrait,
    Workspace, WorkspaceTrait,
};

//AddressedChunkArray
#[query]
#[candid_method(query)]
pub fn test_chunk_array() -> AddressedChunkArray {
    let ws: Workspace = vec![
        vec![
            16_u128.to_candy(),
            Principal::from_text("aaaaa-aa").unwrap().to_candy(),
        ],
        vec![
            16_u128.to_candy(),
            Principal::from_text("aaaaa-aa").unwrap().to_candy(),
        ],
        vec![
            (-123_456_789_000_i128).to_candy(),
            "Hello, world!".to_candy(),
        ],
    ];
    ws.to_addressed_chunk_array()
}

#[query]
#[candid_method(query)]
pub fn get_chunk_array_size() -> u128 {
    let arr = test_chunk_array();
    arr.get_addressed_chunk_array_size()
}

#[query]
#[candid_method(query)]
pub fn get_flatten_chunk_array() -> Vec<u8> {
    let arr = test_chunk_array();
    arr.flatten()
}

#[query]
#[candid_method(query)]
pub fn get_data_chunk_from_addressed_chunk_array(data_zone: u128, data_chunk: u128) -> CandyShared {
    let arr = test_chunk_array();
    arr.get_data_chunk(data_zone, data_chunk)
}

//DataZone
#[query]
#[candid_method(query)]
pub fn get_data_zone_size() -> u128 {
    let dz = vec![
        16_u128.to_candy(),
        Principal::from_text("aaaaa-aa").unwrap().to_candy(),
    ] as DataZone;
    ic_cdk::println!(
        "getting data zone {:?} size {:?}",
        dz,
        dz.get_data_zone_size()
    );
    dz.get_data_zone_size()
}

#[query]
#[candid_method(query)]
pub fn to_bytes_buffer() -> Vec<Vec<u8>> {
    let dz = vec![
        16_u128.to_candy(),
        Principal::from_text("aaaaa-aa").unwrap().to_candy(),
    ];
    dz.to_bytes_buffer()
}

#[query]
#[candid_method(query)]
pub fn from_buffer() -> Vec<CandyShared> {
    let dz = vec![
        16_u128.to_candy(),
        Principal::from_text("aaaaa-aa").unwrap().to_candy(),
    ];
    let buffer = dz.to_bytes_buffer();
    DataZone::from_buffer(buffer)
}

//Workspace
#[query]
#[candid_method(query)]
pub fn count_chunks(chunks: AddressedChunkArray) -> u128 {
    let ws: Workspace = Workspace::from_addressed_chunks(chunks);
    ws.count_addressed_chunks()
}

#[query]
#[candid_method(query)]
pub fn to_workspace_and_back(chunks: AddressedChunkArray) -> AddressedChunkArray {
    let ws = Workspace::from_addressed_chunks(chunks);
    ws.to_addressed_chunk_array()
}

#[query]
#[candid_method(query)]
pub fn get_ws_chunk_size(chunks: AddressedChunkArray, max_chunk_size: u128) -> u128 {
    let ws: Workspace = Workspace::from_addressed_chunks(chunks);
    ws.get_workspace_chunk_size(max_chunk_size)
}

#[query]
#[candid_method(query)]
pub fn get_workspace_chunk(
    chunks: AddressedChunkArray,
    chunk_id: u128,
    max_chunk_size: u128,
) -> (ChunkingType, AddressedChunkArray) {
    let ws = Workspace::from_addressed_chunks(chunks);
    ws.get_workspace_chunk(chunk_id, max_chunk_size)
}
