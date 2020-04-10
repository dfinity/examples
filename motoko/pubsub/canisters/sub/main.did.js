export default ({ IDL }) => {
 const Counter = IDL.Record({'topic': IDL.Text, 'value': IDL.Nat})
 return IDL.Service({'getCount': IDL.Func([], [IDL.Nat], ['query']),
  'init': IDL.Func([], [], ['oneway']),
  'updateCount': IDL.Func([Counter], [], ['oneway'])});
};
