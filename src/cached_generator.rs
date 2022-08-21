use chrono::{DateTime, Local};
use job_scheduler::{Job, JobScheduler};
use log::info;
use crate::allocator::BitsAllocator;
use crate::buffer::{RejectedPutBufferHandler, RejectedTakeBufferHandler, RingBuffer};
use crate::default_generator::{InteriorDefaultUidGenerator};
use crate::{DisposableWorkerIdAssigner, StringResult, UidGenerator};

struct CacheUidGenerator<'a, const buffer_size: usize> {
    boost_power: i32,
    padding_factor: i32,
    schedule_interval: Option<i64>,
    rejected_put_buffer_handler: Box<dyn RejectedPutBufferHandler<buffer_size>>,
    rejected_take_buffer_handler: Box<dyn RejectedTakeBufferHandler<buffer_size>>,
    ring_buffer: RingBuffer<buffer_size>,
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
    job_scheduler: JobScheduler<'a>,

}




impl <'a, const buffer_size: usize>CacheUidGenerator<'a, buffer_size> {
    const DEFAULT_BOOST_POWER: i32 = 3;



    pub fn init(&mut self) -> StringResult<()>{

        self.bits_allocator = BitsAllocator::from(self.time_bits, self.worker_bits, self.seq_bits).unwrap();
        let worker_id_assigner = DisposableWorkerIdAssigner::default();

        self.worker_id = worker_id_assigner.assign_worker_id();

        if self.worker_id > self.bits_allocator.max_worker_id() {
            return StringResult::Err(format!("worker id: {}, exceeds the max: {}", self.worker_id, self.bits_allocator.max_worker_id()));
        }
        self.ring_buffer.init_buffer()?;

        info!("Initialized bits(1, {}, {}, {}) for workerID:{}", self.time_bits , self.worker_bits, self.seq_bits, self.worker_bits);
        Result::Ok(())
    }


    pub fn init_buffer(&mut self) {
        // let buffer_size = (self.bits_allocator.max_sequence() + 1) << self.boost_power;
        self.ring_buffer = RingBuffer::<buffer_size>::from(50).unwrap();
        info!("Initialized ring buffer size:{}, paddingFactor:{}", buffer_size, self.padding_factor);
        // 填充buffer
    }

    pub fn init_job_scheduler(&mut self) {
        self.job_scheduler = JobScheduler::new();
        self.job_scheduler.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
            println!("job test")
        }));
    }

    fn next_ids_for_one_second(&self, current_second: i64) -> Vec<i64> {
        let size = self.bits_allocator.max_sequence() + 1;
        let mut uids = Vec::with_capacity(size as usize);

        let first_seq_uid = self.bits_allocator.allocate(current_second - self.epoch_seconds, self.worker_id, 0);
        for offset in 0..size {
            uids.push(first_seq_uid + offset)
        }
        uids
    }

}


impl <'a, const buffer_size: usize>UidGenerator for CacheUidGenerator<'a, buffer_size> {
    fn get_uid(&mut self) -> StringResult<i64> {
        match self.ring_buffer.take() {
            None => {
                return StringResult::Err(format!("ran out of the uid"))
            }
            Some(uid) => {
                StringResult::Ok(uid)
            }
        }
    }

    fn parse_uid(&self, uid: i64) -> String {
        todo!()
    }
}