# async-rdma
Low-level RDMA data communications with Async Rust.

Basically this is just a way to mess around with RDMA networking to understand its underlying workings and see what if it's all cracked up to be. 

Essentially, I want to make a client-server pipeline that does
1. Client loads its data from disk to RAM
1. Server does RDMA Reads of data 
1. Server computes on RDMA data
1. RDMA Writes back the result to client
1. Verify results and measure performance

*And some other features I want to bolt-on as I go along and horribly scope-creep myself*


For starting, the compute will mainly be FFTs due to its "simplicity" but other compute operations can be easily rolled into the server-side logic,... I hope.

## Roadmap