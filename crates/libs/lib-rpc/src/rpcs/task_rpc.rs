use crate::rpcs::prelude::*;
use lib_core::model::task::{
	Task, TaskBmc, TaskForCreate, TaskForFilter, TaskForUpdate,
};

pub fn rpc_router() -> RpcRouter {
	rpc_router!(
		// Same as RpcRouter::new().add...
		create_task,
		get_task,
		list_tasks,
		update_task,
		delete_task,
	)
}

generate_common_rpc_fns!(
	Bmc: TaskBmc,
	Entity: Task,
	ForCreate: TaskForCreate,
	ForUpdate: TaskForUpdate,
	Filter: TaskForFilter,
	Suffix: task
);
