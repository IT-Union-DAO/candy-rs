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
      'Map' : IDL.Vec(IDL.Tuple(CandyShared, CandyShared)),
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
      'Class' : IDL.Vec(PropertyShared),
    })
  );
  const AddressedChunk = IDL.Tuple(IDL.Nat, IDL.Nat, CandyShared);
  const AddressedChunkArray = IDL.Vec(AddressedChunk);
  return IDL.Service({
    'countChunks' : IDL.Func([AddressedChunkArray], [IDL.Nat], []),
    'fromBuffer' : IDL.Func([], [IDL.Vec(CandyShared)], []),
    'getChunkArraySize' : IDL.Func([], [IDL.Nat], []),
    'getDataChunkFromAddressedChunkArray' : IDL.Func(
        [IDL.Nat, IDL.Nat],
        [CandyShared],
        [],
      ),
    'getDataZoneSize' : IDL.Func([], [IDL.Nat], []),
    'getFlattenChunkedArray' : IDL.Func([], [IDL.Vec(IDL.Nat8)], []),
    'getWorkspaceChunk' : IDL.Func(
        [AddressedChunkArray, IDL.Nat, IDL.Nat],
        [
          IDL.Variant({ 'eof' : IDL.Null, 'chunk' : IDL.Null }),
          AddressedChunkArray,
        ],
        [],
      ),
    'getWorkspaceChunkSize' : IDL.Func(
        [AddressedChunkArray, IDL.Nat],
        [IDL.Nat],
        [],
      ),
    'testChunkArray' : IDL.Func([], [AddressedChunkArray], []),
    'toBytesBuffer' : IDL.Func([], [IDL.Vec(IDL.Vec(IDL.Nat8))], []),
    'toWorkspaceAndBack' : IDL.Func(
        [AddressedChunkArray],
        [AddressedChunkArray],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
