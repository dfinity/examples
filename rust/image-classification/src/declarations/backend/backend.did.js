export const idlFactory = ({ IDL }) => {
  const Classification = IDL.Record({
    'label' : IDL.Text,
    'score' : IDL.Float32,
  });
  const ClassificationError = IDL.Record({ 'message' : IDL.Text });
  const ClassificationResult = IDL.Variant({
    'Ok' : IDL.Vec(Classification),
    'Err' : ClassificationError,
  });
  return IDL.Service({
    'classify' : IDL.Func([IDL.Vec(IDL.Nat8)], [ClassificationResult], []),
  });
};
export const init = ({ IDL }) => { return []; };
