# Error reproduction using embassy primitives from a web-worker

This tries to start and use embassy primitives from a webworker, and fails at this.

It starts from https://github.com/embassy-rs/embassy/tree/main/examples/wasm, however adds a bunch of changes to allow the usage of webworkers.
Most changes are explained here: https://github.com/trunk-rs/trunk/tree/main/examples/wasm_threads.
Many changes are probably not required to trigger the issues, they are required to have web-workers with shared memory (AKA wasm threads), but `wasm_thread` is the webworker API I'm familiar with, and requires these.
Additional changes are made to make the issue visible (e.g. setting a panic hook to show the panic in the background thread).
