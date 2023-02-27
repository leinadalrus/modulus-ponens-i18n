use crate::controllers::QueryConfig;
// use crate::lib::StructuredClone; // unnecessary use case because-
// -StructuredClone is in lib.rs already.
use web_sys::Worker;

#[derive(Debug, Clone, Copy)]
pub trait StoreWorkerStructuredClone {
    /// An optional choice function to choose between QueryConfig or \ Web-sys's Worker
    fn optional_clone_structure<T>(worker_or_query: T);
}

/// WorkerStore employs a type-generic so that Worker or QueryConfig can be stored.
#[derive(Debug, Clone, Copy)]
pub struct WorkerStore<T>(T);

impl StoreWorkerStructuredClone for WorkerStore {

}