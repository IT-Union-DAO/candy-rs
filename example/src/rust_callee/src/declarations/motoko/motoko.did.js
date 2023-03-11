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
      'Nats' : IDL.Variant({
        'thawed' : IDL.Vec(IDL.Nat),
        'frozen' : IDL.Vec(IDL.Nat),
      }),
      'Text' : IDL.Text,
      'Bytes' : IDL.Variant({
        'thawed' : IDL.Vec(IDL.Nat8),
        'frozen' : IDL.Vec(IDL.Nat8),
      }),
      'Int16' : IDL.Int16,
      'Int32' : IDL.Int32,
      'Int64' : IDL.Int64,
      'Option' : IDL.Opt(CandyValue),
      'Floats' : IDL.Variant({
        'thawed' : IDL.Vec(IDL.Float64),
        'frozen' : IDL.Vec(IDL.Float64),
      }),
      'Float' : IDL.Float64,
      'Principal' : IDL.Principal,
      'Array' : IDL.Variant({
        'thawed' : IDL.Vec(CandyValue),
        'frozen' : IDL.Vec(CandyValue),
      }),
      'Class' : IDL.Vec(Property),
    })
  );
  return IDL.Service({
    'getBlob' : IDL.Func([], [CandyValue], ['query']),
    'getBool' : IDL.Func([], [CandyValue], ['query']),
    'getClass' : IDL.Func([], [CandyValue], ['query']),
    'getEmpty' : IDL.Func([], [CandyValue], ['query']),
    'getFloat' : IDL.Func([], [CandyValue], ['query']),
    'getFrozenArray' : IDL.Func([], [CandyValue], ['query']),
    'getFrozenBytes' : IDL.Func([], [CandyValue], ['query']),
    'getFrozenFloats' : IDL.Func([], [CandyValue], ['query']),
    'getFrozenNats' : IDL.Func([], [CandyValue], ['query']),
    'getFrozenText' : IDL.Func([], [CandyValue], ['query']),
    'getInt' : IDL.Func([], [CandyValue], ['query']),
    'getInt16' : IDL.Func([], [CandyValue], ['query']),
    'getInt32' : IDL.Func([], [CandyValue], ['query']),
    'getInt64' : IDL.Func([], [CandyValue], ['query']),
    'getInt8' : IDL.Func([], [CandyValue], ['query']),
    'getNat' : IDL.Func([], [CandyValue], ['query']),
    'getNat16' : IDL.Func([], [CandyValue], ['query']),
    'getNat32' : IDL.Func([], [CandyValue], ['query']),
    'getNat64' : IDL.Func([], [CandyValue], ['query']),
    'getNat8' : IDL.Func([], [CandyValue], ['query']),
    'getNullOption' : IDL.Func([], [CandyValue], ['query']),
    'getOptSome' : IDL.Func([], [CandyValue], ['query']),
    'getPrincipal' : IDL.Func([], [CandyValue], ['query']),
    'getText' : IDL.Func([], [CandyValue], ['query']),
    'getThawedArray' : IDL.Func([], [CandyValue], ['query']),
    'getThawedBytes' : IDL.Func([], [CandyValue], ['query']),
    'getThawedFloats' : IDL.Func([], [CandyValue], ['query']),
    'getThawedNats' : IDL.Func([], [CandyValue], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
