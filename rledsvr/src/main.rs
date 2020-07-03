extern crate pretty_env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

mod taskmgr;
mod tasks;

use std::env;
use taskmgr::start_task_manager;

mod filters {
    use core::TaskMessage;
    use warp::Filter;

    pub fn all() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        execute()
    }

    pub fn execute() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("execute")
            .and(warp::post())
            .and(json_body())
            .and_then(crate::handlers::execute)
    }

    fn json_body() -> impl Filter<Extract = (TaskMessage,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers {
    use crate::taskmgr::send_message;
    use core::TaskMessage;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn execute(payload: TaskMessage) -> Result<impl warp::Reply, Infallible> {
        log::info!("Executing: {:?}", payload);
        send_message(payload).unwrap();
        Ok(StatusCode::OK)
    }
}

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=execute=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    info!("Starting task manager and warp server...");

    start_task_manager();

    let api = filters::all();
    let routes = api; // .with(warp::log("execute"));
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await
}
