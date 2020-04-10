export default ({ IDL }) => {
 const Counter = IDL.Record({'topic': IDL.Text, 'value': IDL.Nat})
 const Subscriber =
  IDL.Record({'topic': IDL.Text,
   'callback': IDL.Func([Counter], [], ['oneway'])})
 return IDL.Service({'publish': IDL.Func([Counter], [], ['oneway']),
  'subscribe': IDL.Func([Subscriber], [], ['oneway'])});
};
