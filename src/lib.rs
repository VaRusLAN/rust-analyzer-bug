use redis_module::{redis_module, Context, RedisString, Status};

redis_module! {
    name: "Test",
    version: 1.0,
    allocator: (redis_module::alloc::RedisAlloc, redis_module::alloc::RedisAlloc),
    data_types: [],
    init: module_init,
    deinit: module_deinit,
    commands: []
}

fn module_init(_: &Context, args: &Vec<RedisString>) -> Status {
    Status::Ok
}

fn module_deinit(_: &Context) -> Status {
    Status::Err
}
