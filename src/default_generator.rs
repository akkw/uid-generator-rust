use std::fmt::Error;
use std::sync::Mutex;
use chrono::{Date, DateTime, Local, TimeZone};
use log::info;
use crate::allocator::BitsAllocator;
use crate::{DisposableWorkerIdAssigner, StringResult, UidGenerator};
use crate::metadata_storage::Config;


#[allow(unused_variables, dead_code)]
pub struct InteriorDefaultUidGenerator {
    time_bits: i32,
    worker_bits: i32,
    seq_bits: i32,

    epoch_str: DateTime<Local>,
    epoch_seconds: i64,

    bits_allocator: BitsAllocator,
    worker_id: i64,
    sequence: i64,
    last_second: i64,

    worker_id_assigner: DisposableWorkerIdAssigner,
}

#[allow(unused_variables, dead_code)]
pub struct UidGeneratorBuild {
    time_bits: Option<i32>,
    worker_bits: Option<i32>,
    seq_bits: Option<i32>,
    epoch_str: Option<DateTime<Local>>,
    epoch_seconds: Option<i64>,
    config: Config,
}

impl UidGeneratorBuild {
    pub fn form(time_bits: Option<i32>, worker_bits: Option<i32>,
                seq_bits: Option<i32>, epoch_str: Option<DateTime<Local>>,
                epoch_seconds: Option<i64>, config: Config, ) -> UidGeneratorBuild {
        UidGeneratorBuild {
            time_bits,
            worker_bits,
            seq_bits,
            epoch_str,
            epoch_seconds,
            config,
        }
    }
}

#[allow(unused_variables, dead_code)]
impl InteriorDefaultUidGenerator {
    const TIME_BITS: i32 = 28;
    const WORKER_BITS: i32 = 28;
    const SEQ_BITS: i32 = 28;

    const EPOCH_Y: i32 = 2022;
    const EPOCH_M: u32 = 8;
    const EPOCH_D: u32 = 17;
    const EPOCH_SECONDS: i64 = 1660665600;
}

impl From<UidGeneratorBuild> for InteriorDefaultUidGenerator {
    fn from(build: UidGeneratorBuild) -> Self {
        let mut generator = InteriorDefaultUidGenerator {
            time_bits: InteriorDefaultUidGenerator::TIME_BITS,
            worker_bits: InteriorDefaultUidGenerator::WORKER_BITS,
            seq_bits: InteriorDefaultUidGenerator::SEQ_BITS,
            epoch_str: Local.ymd(InteriorDefaultUidGenerator::EPOCH_Y, InteriorDefaultUidGenerator::EPOCH_M,
                                 InteriorDefaultUidGenerator::EPOCH_D).and_hms(0, 0, 0),
            epoch_seconds: InteriorDefaultUidGenerator::EPOCH_SECONDS,
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
impl InteriorDefaultUidGenerator {
     fn init(&mut self) -> StringResult<()> {
        self.bits_allocator = BitsAllocator::from(self.time_bits, self.worker_bits, self.seq_bits).unwrap();
        let worker_id_assigner = DisposableWorkerIdAssigner::default();

        self.worker_id = worker_id_assigner.assign_worker_id();

        if self.worker_id > self.bits_allocator.max_worker_id() {
            return StringResult::Err(format!("worker id: {}, exceeds the max: {}", self.worker_id, self.bits_allocator.max_worker_id()));
        }

        info!("Initialized bits(1, {}, {}, {}) for workerID:{}", self.time_bits , self.worker_bits, self.seq_bits, self.worker_bits);
        Result::Ok(())
    }

    fn current_second(&self) -> StringResult<i64> {
        let current_second = Local::now().timestamp();

        if current_second - self.epoch_seconds > self.bits_allocator.max_delta_seconds() {
            return StringResult::Err(format!("Timestamp bits is exhausted. Refusing UID generate. Now: {}", current_second));
        }

        StringResult::Ok(current_second)
    }

    fn next_second(&self, last_timestamp: i64) -> StringResult<i64> {
        let mut timestamp = self.current_second()?;

        while timestamp <= last_timestamp {
            timestamp = self.current_second()?;
        }

        return StringResult::Ok(timestamp);
    }

    fn next_id(&mut self) -> StringResult<i64> {
        let mut current_second = self.current_second()?;

        // 防止时钟回拨
        if current_second < self.last_second {
            let refused_seconds = self.last_second - current_second;
            return StringResult::Err(format!("Clock moved backwards. Refusing for {} seconds", refused_seconds));
        }

        // 同一秒生成多个uid
        if current_second == self.last_second {
            self.sequence = (self.sequence + 1) & self.bits_allocator.max_sequence();
            if self.sequence == 0 {
                current_second = self.next_second(self.last_second)?;
            }
        } else {
            self.sequence = 0;
        }

        self.last_second = current_second;

        StringResult::Ok(self.bits_allocator.allocate(current_second - self.epoch_seconds, self.worker_id, self.sequence))
    }
}


pub struct DefaultUidGenerator {
    uid_generator: Mutex<InteriorDefaultUidGenerator>,
}

impl From<UidGeneratorBuild> for DefaultUidGenerator {
    fn from(build: UidGeneratorBuild) -> Self {
        let generator = DefaultUidGenerator {
            uid_generator: Mutex::new(InteriorDefaultUidGenerator::from(build)),
        };
        generator.uid_generator.lock().unwrap().init();
        generator
    }
}

impl UidGenerator for DefaultUidGenerator {
    fn get_uid(&mut self) -> StringResult<i64> {
        let mut guard = self.uid_generator.lock().unwrap();
        guard.next_id()
    }

    fn parse_uid(&self, uid: i64) -> String {
        todo!()
    }
}


