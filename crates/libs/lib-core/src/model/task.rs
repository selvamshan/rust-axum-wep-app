use crate::ctx::Ctx;
use crate::generate_common_bmc_fns;
use crate::model::base::{self, DbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::ModelManager;
use crate::model::Result;
use lib_utils::time::Rfc3339;
use modql::field::Fields;
use modql::filter::{FilterNodes, OpValsString};
use modql::filter::{ListOptions, OpValsInt64, OpValsValue};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::types::time::OffsetDateTime;

use sqlx::FromRow;

// region:  --- Rask Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Task {
	pub id: i64,
	pub title: String,

	// -- Timestamps
	//    (creator and last modified user_id/time)
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Fields, Deserialize)]
pub struct TaskForCreate {
	pub title: String,
}

#[derive(Fields, Deserialize)]
pub struct TaskForUpdate {
	pub title: Option<String>,
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct TaskForFilter {
	pub id: Option<OpValsInt64>,
	pub title: Option<OpValsString>,

	pub cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub ctime: Option<OpValsValue>,
	pub mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub mtime: Option<OpValsValue>,
}

// endregion: -- Task Types

// region: --- TaskBmc
pub struct TaskBmc;

impl DbBmc for TaskBmc {
	const TABLE: &'static str = "task";
}

// This will generate the `impl AgentBmc {...}` with the default CRUD functions.
generate_common_bmc_fns!(
	Bmc: TaskBmc,
	Entity: Task,
	ForCreate: TaskForCreate,
	ForUpdate: TaskForUpdate,
	Filter: TaskForFilter,
);

// endregion: --- TaskBmc

// // region -- Tests
// #[cfg(test)]
// mod tests {
//     #![allow(unused)]
//     use super::*;
//     use anyhow::Result;
//     use serial_test::serial;
//     use crate::_dev_utils;
//     use crate::model::Error;

//     #[serial]
//     #[tokio::test]
//     async fn test_create_ok() -> Result<()> {
//         let mm = _dev_utils::init_test().await;
//         let ctx = Ctx::root_ctx();
//         let fx_title = "test_create_ok title";

//         // -- Exec
//         let task_c = TaskForCreate {
//             title: fx_title.to_string(),
//         };
//         let id = TaskBmc::create(&ctx, &mm, task_c).await?;

//         // -- Check
//         let task = TaskBmc::get(&ctx, &mm, id).await?;
//         println!("->> {}", task.id);
//         assert_eq!(task.title, fx_title);

//         // -- Clean
//         TaskBmc::delete(&ctx, &mm, id).await?;

//         Ok(())
//     }

//    #[serial]
//    #[tokio::test]
//    async fn test_get_err_not_found() -> Result<()> {
//        // -- Setup & Fixutes
//        let mm = _dev_utils::init_test().await;
//        let ctx = Ctx::root_ctx();
//        let fx_id = 100;

//        // -- Exec
//        let res = TaskBmc::get(&ctx, &mm, fx_id).await;

//        // -- Check
//        assert!(
//         matches!(
//             res,
//             Err(Error::EntityNotFound { entity: "task", id: 100 })
//         ),
//         "EntityNotFound not matching",
//        );

//        Ok(())

//    }

//    #[serial]
//     #[tokio::test]
//     async fn test_list_ok() -> Result<()> {
//          // -- Setup & Fixutes
//        let mm = _dev_utils::init_test().await;
//        let ctx = Ctx::root_ctx();
//        let fx_titles = &["test_list_ok-task 01", "test_list_ok-task 02"];
//        _dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;

//        // -- Exec
// 	   let tasks = TaskBmc::list(&ctx, &mm).await?;

// 	   // -- Check
// 	   let tasks: Vec<Task> = tasks
// 	   .into_iter()
// 	   .filter(|t| t.title.starts_with("test_list_ok"))
// 	   .collect();

// 		assert_eq!(tasks.len(), 2, "number of seeded tasks.");

// 		//  --- Clean
// 		for task in tasks.iter() {
// 			TaskBmc::delete(&ctx, &mm, task.id).await?;
// 		}

//        Ok(())
//     }

//     #[serial]
//     #[tokio::test]
//     async fn test_update_ok() -> Result<()> {
//         let mm = _dev_utils::init_test().await;
//         let ctx = Ctx::root_ctx();
//         let fx_title = "test_update_ok - task 01";
//         let fx_title_new = "test_update_ok - task 01 - new";
//         let fx_task = _dev_utils::seed_tasks(&ctx, &mm, &[fx_title])
//         .await?
//         .remove(0);

//          // -- Exec
//         TaskBmc::update(
//             &ctx,
//             &mm,
//             fx_task.id,
//             TaskForUpdate {
//                 title: Some(fx_title_new.to_string()),
//             },
//             ).await?;

//         // -- Check
//         let task = TaskBmc::get(&ctx, &mm, fx_task.id).await?;
//         assert_eq!(task.title, fx_title_new);

//         Ok(())
//     }

//    #[serial]
//    #[tokio::test]
//    async fn test_delete_err_not_found() -> Result<()> {
//        // -- Setup & Fixutes
//        let mm = _dev_utils::init_test().await;
//        let ctx = Ctx::root_ctx();
//        let fx_id = 100;

//        // -- Exec
//        let res = TaskBmc::delete(&ctx, &mm, fx_id).await;

//        // -- Check
//        assert!(
//         matches!(
//             res,
//             Err(Error::EntityNotFound { entity: "task", id: 100 })
//         ),
//         "EntityNotFound not matching",
//        );

//        Ok(())

//    }

// }

// endregion -- Tests
