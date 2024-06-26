export const idlFactory = ({ IDL }) => {
  const CandyShared = IDL.Rec();
  const PropertyShared = IDL.Record({
    'value' : CandyShared,
    'name' : IDL.Text,
    'immutable' : IDL.Bool,
  });
  CandyShared.fill(
    IDL.Variant({
      'Int' : IDL.Int,
      'Map' : IDL.Vec(IDL.Tuple(IDL.Text, CandyShared)),
      'Nat' : IDL.Nat,
      'Set' : IDL.Vec(CandyShared),
      'Nat16' : IDL.Nat16,
      'Nat32' : IDL.Nat32,
      'Nat64' : IDL.Nat64,
      'Blob' : IDL.Vec(IDL.Nat8),
      'Bool' : IDL.Bool,
      'Int8' : IDL.Int8,
      'Ints' : IDL.Vec(IDL.Int),
      'Nat8' : IDL.Nat8,
      'Nats' : IDL.Vec(IDL.Nat),
      'Text' : IDL.Text,
      'Bytes' : IDL.Vec(IDL.Nat8),
      'Int16' : IDL.Int16,
      'Int32' : IDL.Int32,
      'Int64' : IDL.Int64,
      'Option' : IDL.Opt(CandyShared),
      'Floats' : IDL.Vec(IDL.Float64),
      'Float' : IDL.Float64,
      'Principal' : IDL.Principal,
      'Array' : IDL.Vec(CandyShared),
      'ValueMap' : IDL.Vec(IDL.Tuple(CandyShared, CandyShared)),
      'Class' : IDL.Vec(PropertyShared),
    })
  );
  const ChunkingType = IDL.Variant({ 'eof' : IDL.Null, 'chunk' : IDL.Null });
  return IDL.Service({
    '__get_candid' : IDL.Func([], [IDL.Text], ['query']),
    'candy_array_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_blob_to_blob' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'candy_blob_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_bool_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_bytes_to_blob' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'candy_bytes_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_class_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_float_to_blob' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'candy_float_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_floats_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_int16_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_int32_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_int64_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_int8_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_int_to_blob' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'candy_int_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_ints_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_nat16_to_blob' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'candy_nat16_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_nat32_to_blob' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'candy_nat32_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_nat64_to_blob' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'candy_nat64_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_nat8_to_blob' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'candy_nat8_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_nat_to_blob' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'candy_nat_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_nats_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_option_some_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_option_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_principal_to_blob' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'candy_principal_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'candy_text_to_blob' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'candy_text_to_json' : IDL.Func([], [IDL.Text], ['query']),
    'count_chunks' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Nat, IDL.Nat, CandyShared))],
        [IDL.Nat],
        ['query'],
      ),
    'from_buffer' : IDL.Func([], [IDL.Vec(CandyShared)], ['query']),
    'get_candy' : IDL.Func([IDL.Text, IDL.Text], [CandyShared], []),
    'get_chunk_array_size' : IDL.Func([], [IDL.Nat], ['query']),
    'get_data_chunk_from_addressed_chunk_array' : IDL.Func(
        [IDL.Nat, IDL.Nat],
        [CandyShared],
        ['query'],
      ),
    'get_data_zone_size' : IDL.Func([], [IDL.Nat], ['query']),
    'get_flatten_chunk_array' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'get_workspace_chunk' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Nat, IDL.Nat, CandyShared)), IDL.Nat, IDL.Nat],
        [ChunkingType, IDL.Vec(IDL.Tuple(IDL.Nat, IDL.Nat, CandyShared))],
        ['query'],
      ),
    'get_ws_chunk_size' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Nat, IDL.Nat, CandyShared)), IDL.Nat],
        [IDL.Nat],
        ['query'],
      ),
    'size_of_candy_array' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_blob' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_bool' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_bytes' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_class' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_float' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_floats' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_int' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_int16' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_int32' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_int64' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_int8' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_ints' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_map' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_nat' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_nat16' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_nat32' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_nat64' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_nat8' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_nats' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_option' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_option_none' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_principal' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_set' : IDL.Func([], [IDL.Nat], ['query']),
    'size_of_candy_text' : IDL.Func([], [IDL.Nat], ['query']),
    'test_chunk_array' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Nat, IDL.Nat, CandyShared))],
        ['query'],
      ),
    'to_bytes_buffer' : IDL.Func([], [IDL.Vec(IDL.Vec(IDL.Nat8))], ['query']),
    'to_workspace_and_back' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Nat, IDL.Nat, CandyShared))],
        [IDL.Vec(IDL.Tuple(IDL.Nat, IDL.Nat, CandyShared))],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
