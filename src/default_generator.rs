use chrono::{Date, Local, TimeZone};
use log::info;
use crate::allocator::BitsAllocator;
use crate::{DisposableWorkerIdAssigner, StringResult, UidGenerator};
use crate::metadata_storage::Config;


#[allow(unused_variables, dead_code)]
struct DefaultUidGenerator {
    time_bits: i32,
    worker_bits: i32,
    seq_bits: i32,

    epoch_str: Date<Local>,
    epoch_seconds: i64,

    bits_allocator: BitsAllocator,
    worker_id: i64,
    sequence: i64,
    last_second: i64,

    worker_id_assigner: DisposableWorkerIdAssigner,
}
#[allow(unused_variables)]
impl UidGenerator for DefaultUidGenerator {
    fn get_uid(&mut self) -> Result<i64, String> {
        todo!()
    }

    fn parse_uid(&self, uid: i64) -> String {
        todo!()
    }
}

#[allow(unused_variables, dead_code)]
struct UidGeneratorBuild {
    time_bits: Option<i32>,
    worker_bits: Option<i32>,
    seq_bits: Option<i32>,
    epoch_str: Option<Date<Local>>,
    epoch_seconds: Option<i64>,
    config: Config,
}

#[allow(unused_variables, dead_code)]
impl DefaultUidGenerator {
    const TIME_BITS: i32 = 28;
    const WORKER_BITS: i32 = 28;
    const SEQ_BITS: i32 = 28;

    const EPOCH_Y: i32 = 2022;
    const EPOCH_M: u32 = 8;
    const EPOCH_D: u32 = 17;
    const EPOCH_SECONDS: i64 = 1660665600;
}

impl From<UidGeneratorBuild> for DefaultUidGenerator {
    fn from(build: UidGeneratorBuild) -> Self {
        let mut generator = DefaultUidGenerator {
            time_bits: DefaultUidGenerator::TIME_BITS,
            worker_bits: DefaultUidGenerator::WORKER_BITS,
            seq_bits: DefaultUidGenerator::SEQ_BITS,
            epoch_str: Local.ymd(DefaultUidGenerator::EPOCH_Y, DefaultUidGenerator::EPOCH_M, DefaultUidGenerator::EPOCH_D),
            epoch_seconds: DefaultUidGenerator::EPOCH_SECONDS,
            bits_allocator: Default::default(),
            worker_id: 0,
            sequence: 0,
            last_second: 0,
            worker_id_assigner: Default::default(),
        };
        if let Some(time_bits) = build.time_bits {
            generator.time_bits = time_bits
        }
        if let Some(worker_bits) = build.worker_bits {
            generator.worker_bits = worker_bits
        }
        if let Some(seq_bits) = build.seq_bits {
            generator.seq_bits = seq_bits
        }
        if let Some(date) = build.epoch_str {
            generator.epoch_str = date
        }
        if let Some(seconds) = build.epoch_seconds {
            generator.epoch_seconds = seconds;
        }
        generator
    }
}
#[allow(unused_variables, dead_code)]
impl DefaultUidGenerator {
    pub fn init(&mut self) -> StringResult<()> {
        self.bits_allocator = BitsAllocator::from(self.time_bits, self.worker_bits, self.seq_bits).unwrap();
        let worker_id_assigner = DisposableWorkerIdAssigner::default();

        self.worker_id = worker_id_assigner.assign_worker_id();

        if self.worker_id > self.bits_allocator.max_worker_id() {
            return StringResult::Err(format!("worker id: {}, exceeds the max: {}", self.worker_id, self.bits_allocator.max_worker_id()));
        }

        info!("Initialized bits(1, {}, {}, {}) for workerID:{}", self.time_bits , self.worker_bits, self.seq_bits, self.worker_bits);
        Result::Ok(())
    }
}