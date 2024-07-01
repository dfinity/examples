export const idlFactory = ({ IDL }) => {
  const Embedding = IDL.Record({ 'v0' : IDL.Vec(IDL.Float32) });
  const EmbeddingError = IDL.Record({ 'message' : IDL.Text });
  const EmbeddingResult = IDL.Variant({
    'Ok' : Embedding,
    'Err' : EmbeddingError,
  });
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
  const Recognize = IDL.Record({ 'label' : IDL.Text, 'score' : IDL.Float32 });
  const RecognizeError = IDL.Record({ 'message' : IDL.Text });
  const RecognizeResult = IDL.Variant({
    'Ok' : Recognize,
    'Err' : RecognizeError,
  });
  return IDL.Service({
    'add' : IDL.Func([IDL.Text, IDL.Vec(IDL.Nat8)], [EmbeddingResult], []),
    'detect' : IDL.Func([IDL.Vec(IDL.Nat8)], [DetectionResult], []),
    'detect_query' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [DetectionResult],
        ['query'],
      ),
    'embedding' : IDL.Func([IDL.Vec(IDL.Nat8)], [EmbeddingResult], []),
    'recognize' : IDL.Func([IDL.Vec(IDL.Nat8)], [RecognizeResult], []),
  });
};
export const init = ({ IDL }) => { return []; };
