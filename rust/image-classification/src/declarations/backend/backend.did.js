export const idlFactory = ({ IDL }) => {
  const BoundingBox = IDL.Record({
    'top' : IDL.Float32,
    'left' : IDL.Float32,
    'bottom' : IDL.Float32,
    'right' : IDL.Float32,
  });
  const DetectionError = IDL.Record({ 'message' : IDL.Text });
  const DetectionResult = IDL.Variant({
    'Ok' : BoundingBox,
    'Err' : DetectionError,
  });
  const Embedding = IDL.Record({ 'v0' : IDL.Vec(IDL.Float32) });
  const EmbeddingError = IDL.Record({ 'message' : IDL.Text });
  const EmbeddingResult = IDL.Variant({
    'Ok' : Embedding,
    'Err' : EmbeddingError,
  });
  return IDL.Service({
    'detect' : IDL.Func([IDL.Vec(IDL.Nat8)], [DetectionResult], []),
    'detect_query' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [DetectionResult],
        ['query'],
      ),
    'embedding' : IDL.Func([IDL.Vec(IDL.Nat8)], [EmbeddingResult], []),
  });
};
export const init = ({ IDL }) => { return []; };
