pub mod buffer;
pub mod default_generator;
pub mod dao;
pub mod allocator;



type StringResult<T> = Result<T, String>;


pub trait UidGenerator {

    fn get_uid(&mut self) -> Result<i64, String>;

    fn parse_uid(&self, uid: i64) -> String;

}

#[derive(Debug, Default)]
pub struct DisposableWorkerIdAssigner {

}

impl DisposableWorkerIdAssigner {
    pub fn assign_worker_id(&self) -> i64{
        0
    }
}