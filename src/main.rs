//! Capture CLI definition and call appropriate actions

#![warn(missing_docs)]

use env_logger::Env;
use log::{info};
use pyo3::prelude::*;
use pyo3::types::PyList;

use futures::future::join_all;

#[pyo3_asyncio::tokio::main]
async fn main() -> PyResult<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Calling start");


    // let fut0 = Python::with_gil(|py| {
    //     let startme = py.import("mypy.startme")?;
    //     // convert asyncio.sleep into a Rust Future
    //     pyo3_asyncio::tokio::into_future(startme.call_method0("as_start_loop")?)
    // })?;

    let fut1 = Python::with_gil(|py| {
        let asyncio = py.import("asyncio")?;
        // convert asyncio.sleep into a Rust Future
        pyo3_asyncio::tokio::into_future(asyncio.call_method1("sleep", (5.into_py(py),))?)
    })?;

    let mut fut_list = Python::with_gil(|py| {
        // let sys = py.import("sys").expect("");
        // PyModule::import(py, "site");
        let _helpme = py.import("site").unwrap();

        let syspath: &PyList = py.import("sys")
            .unwrap()
            .getattr("path")
            .unwrap()
            .try_into()
            .unwrap();

        println!("The syspath is {}", syspath);

        let mypy = py.import("mypy").expect("");
        let sub0 = pyo3_asyncio::tokio::into_future(mypy.call_method0("as_stop").expect("")).expect("");
        let sub1 = pyo3_asyncio::tokio::into_future(mypy.call_method0("as_start_loop").expect("")).expect("");
        // let retval = vec![sub0?, sub1?];
        // futures::future::join_all(vec![sub0, sub1])
        // Ok(vec![sub0, sub1])
        // Ok(vec![])
        vec![sub0, sub1]
    });

    // let mut  fut_list =vec![fut2];

    fut_list.push(fut1);

    info!("Have requested python function");

    join_all(fut_list).await;
    // fut0.await?;
    // fut1.await?;
    // fut2.await?;

    info!("Completed all");

    Ok(())
}



// fn main() {
//     //! Capture CLI definition and call appropriate actions

//     env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

//     info!("Calling start");

// }
