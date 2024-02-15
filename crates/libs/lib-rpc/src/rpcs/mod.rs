use crate::router::RpcRouter;

mod task_rpc;

mod macro_utils;
mod prelude;

pub fn all_rpc_router() -> RpcRouter {
	RpcRouter::new().extend(task_rpc::rpc_router())
}
