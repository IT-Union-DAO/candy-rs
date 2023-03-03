export const idlFactory = ({ IDL }) => {
  const CandyValue = IDL.Rec();
  const Property = IDL.Record({
    'value' : CandyValue,
    'name' : IDL.Text,
    'immutable' : IDL.Bool,
  });
  CandyValue.fill(
    IDL.Variant({
      'Int' : IDL.Int,
      'Nat' : IDL.Nat,
      'Empty' : IDL.Null,
      'Nat16' : IDL.Nat16,
      'Nat32' : IDL.Nat32,
      'Nat64' : IDL.Nat64,
      'Blob' : IDL.Vec(IDL.Nat8),
      'Bool' : IDL.Bool,
      'Int8' : IDL.Int8,
      'Nat8' : IDL.Nat8,
      'Nats' : IDL.Vec(IDL.Nat),
      'Text' : IDL.Text,
      'Int16' : IDL.Int16,
      'Int32' : IDL.Int32,
      'Int64' : IDL.Int64,
      'Option' : IDL.Opt(CandyValue),
      'Floats' : IDL.Vec(IDL.Float64),
      'Float' : IDL.Float64,
      'Principal' : IDL.Principal,
      'Array' : IDL.Vec(CandyValue),
      'Class' : IDL.Vec(Property),
    })
  );
  return IDL.Service({
    'get_blob' : IDL.Func([], [CandyValue], ['query']),
    'get_bool' : IDL.Func([], [CandyValue], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
