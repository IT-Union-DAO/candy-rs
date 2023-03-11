export const idlFactory = ({ IDL }) => {
  const CandyValue = IDL.Rec();
  const Nats = IDL.Variant({
    'thawed' : IDL.Vec(IDL.Nat),
    'frozen' : IDL.Vec(IDL.Nat),
  });
  const Bytes = IDL.Variant({
    'thawed' : IDL.Vec(IDL.Nat8),
    'frozen' : IDL.Vec(IDL.Nat8),
  });
  const Floats = IDL.Variant({
    'thawed' : IDL.Vec(IDL.Float64),
    'frozen' : IDL.Vec(IDL.Float64),
  });
  const Array = IDL.Variant({
    'thawed' : IDL.Vec(CandyValue),
    'frozen' : IDL.Vec(CandyValue),
  });
  const Property = IDL.Record({
    'value' : CandyValue,
    'name' : IDL.Text,
    'immutable' : IDL.Bool,
  });
  CandyValue.fill(
    IDL.Variant({
      'Int' : IDL.Int,
      'Map' : IDL.Vec(IDL.Tuple(IDL.Text, CandyValue)),
      'Nat' : IDL.Nat,
      'Empty' : IDL.Null,
      'Nat16' : IDL.Nat16,
      'Nat32' : IDL.Nat32,
      'Nat64' : IDL.Nat64,
      'Blob' : IDL.Vec(IDL.Nat8),
      'Bool' : IDL.Bool,
      'Int8' : IDL.Int8,
      'Nat8' : IDL.Nat8,
      'Nats' : Nats,
      'Text' : IDL.Text,
      'Bytes' : Bytes,
      'Int16' : IDL.Int16,
      'Int32' : IDL.Int32,
      'Int64' : IDL.Int64,
      'Option' : IDL.Opt(CandyValue),
      'Floats' : Floats,
      'Float' : IDL.Float64,
      'Principal' : IDL.Principal,
      'Array' : Array,
      'Class' : IDL.Vec(Property),
    })
  );
  const ChunkingType = IDL.Variant({ 'eof' : IDL.Null, 'chunk' : IDL.Null });
  return IDL.Service({
    'get_blob' : IDL.Func([], [CandyValue], ['query']),
    'get_bool' : IDL.Func([], [CandyValue], ['query']),
    'get_type' : IDL.Func([], [ChunkingType], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
