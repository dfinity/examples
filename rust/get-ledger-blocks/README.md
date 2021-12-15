# Fetching blocks from the Ledger example

The Ledger exposes an endpoint `get_blocks_pb` that can be used to get a range of blocks encoded with
protocol buffers. This example shows how to query this endpoint and how to decode the response received
from the Ledger. The steps are the following:

1. create a request and encode it as protobuf message
2. send a `get_blocks_pb` request with the encoded request as argument
3. decode the response as protobuf message. This returns a rust list of protobuf encoded blocks
4. optionally decode each block in the response list