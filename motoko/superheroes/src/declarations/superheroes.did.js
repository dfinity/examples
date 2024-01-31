export const idlFactory = ({ IDL }) => {
  const List = IDL.Rec();
  List.fill(IDL.Opt(IDL.Tuple(IDL.Text, List)));
  const Superhero = IDL.Record({ 'superpowers' : List, 'name' : IDL.Text });
  const SuperheroId = IDL.Nat32;
  return IDL.Service({
    'create' : IDL.Func([Superhero], [SuperheroId], []),
    'delete' : IDL.Func([SuperheroId], [IDL.Bool], []),
    'read' : IDL.Func([SuperheroId], [IDL.Opt(Superhero)], ['query']),
    'update' : IDL.Func([SuperheroId, Superhero], [IDL.Bool], []),
  });
};
export const init = ({ IDL }) => { return []; };
