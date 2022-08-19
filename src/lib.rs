pub mod buffer;
pub mod default_generator;
pub mod cached_generator;
pub mod metadata_storage;
pub mod allocator;



type StringResult<T> = Result<T, String>;


pub trait UidGenerator {

    fn get_uid(&mut self) -> StringResult<i64>;

    fn parse_uid(&self, uid: i64) -> String;

}

#[derive(Debug, Default)]
pub struct DisposableWorkerIdAssigner {
}

impl DisposableWorkerIdAssigner {
    pub fn assign_worker_id(&self) -> i64{
        1
    }
}


pub fn bit_count(mut i: usize) -> usize{
    i = i - ((i >> 1) & 0x55555555);
    i = (i & 0x33333333) + ((i >> 2) & 0x33333333);
    i = (i + (i >> 4)) & 0x0f0f0f0f;
    i = i + (i >> 8);
    i = i + (i >> 16);
    return i & 0x3f;
}