use log::info;
use crate::allocator::BitsAllocator;
use crate::{DisposableWorkerIdAssigner, StringResult, UidGenerator};


#[derive(Debug, Default)]
struct DefaultUidGenerator<'a> {
    time_bits: i32,
    worker_bits: i32,
    seq_bits: i32,

    epoch_str: &'a str,
    epoch_seconds: i64,

    bits_allocator: BitsAllocator,
    worker_id: i64,
    sequence: i64,
    last_second: i64,

    worker_id_assigner: DisposableWorkerIdAssigner,
}

impl <'a>UidGenerator for DefaultUidGenerator<'a> {

    fn get_uid(&mut self) -> Result<i64, String> {
        todo!()
    }

    fn parse_uid(&self, uid: i64) -> String {
        todo!()
    }
}

impl <'a>DefaultUidGenerator<'a> {
    const TIME_BITS: i32 = 28;
    const WORKER_BITS: i32 = 28;
    const SEQ_BITS: i32 = 28;

    const EPOCH: &'static str = "2022-8-17";
    const EPOCH_SECONDS: i64 = 1660665600;
}

impl <'a>DefaultUidGenerator<'a> {
    pub fn init(&mut self) -> StringResult<()> {
        self.bits_allocator = BitsAllocator::from(self.time_bits, self.worker_bits, self.seq_bits).unwrap();

        self.worker_id = self.worker_id_assigner.assign_worker_id();

        if self.worker_id > self.bits_allocator.max_worker_id() {
            return StringResult::Err(format!("worker id: {}, exceeds the max: {}", self.worker_id, self.bits_allocator.max_worker_id()));
        }

        info!("Initialized bits(1, {}, {}, {}) for workerID:{}", self.time_bits , self.worker_bits, self.seq_bits, self.worker_bits);
        Result::Ok(())
    }
}