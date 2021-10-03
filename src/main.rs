//! Capture CLI definition and call appropriate actions

#![warn(missing_docs)]

use env_logger::Env;
use log::{info};
use pyo3::prelude::*;

#[pyo3_asyncio::tokio::main]
async fn main() -> PyResult<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Calling start");


    let fut0 = Python::with_gil(|py| {
        let asyncio = py.import("asyncio")?;
        // convert asyncio.sleep into a Rust Future
        pyo3_asyncio::tokio::into_future(asyncio.call_method1("sleep", (1.into_py(py),))?)
    })?;

    let fut1 = Python::with_gil(|py| {
        let startme = py.import("mypy.startme")?;
        pyo3_asyncio::tokio::into_future(startme.call_method0("as_start")?)
    })?;

    info!("Have requested python function");


    fut0.await?;
    fut1.await?;

    info!("Completed all");

    Ok(())
}



// fn main() {
//     //! Capture CLI definition and call appropriate actions

//     env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

//     info!("Calling start");

// }

