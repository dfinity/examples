export const idlFactory = ({ IDL }) => {
  const Embedding = IDL.Record({ 'v0' : IDL.Vec(IDL.Float32) });
  const Error = IDL.Record({ 'message' : IDL.Text });
  const Addition = IDL.Variant({ 'Ok' : Embedding, 'Err' : Error });
  const BoundingBox = IDL.Record({
    'top' : IDL.Float32,
    'left' : IDL.Float32,
    'bottom' : IDL.Float32,
    'right' : IDL.Float32,
  });
  const Detection = IDL.Variant({ 'Ok' : BoundingBox, 'Err' : Error });
  const Person = IDL.Record({ 'label' : IDL.Text, 'score' : IDL.Float32 });
  const Recognition = IDL.Variant({ 'Ok' : Person, 'Err' : Error });
  return IDL.Service({
    'add' : IDL.Func([IDL.Text, IDL.Vec(IDL.Nat8)], [Addition], []),
    'detect' : IDL.Func([IDL.Vec(IDL.Nat8)], [Detection], ['query']),
    'recognize' : IDL.Func([IDL.Vec(IDL.Nat8)], [Recognition], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
