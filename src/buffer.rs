use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicI8, AtomicUsize, Ordering};
use std::sync::Mutex;
use crate::{bit_count, StringResult};


pub trait RejectedPutBufferHandler<const T: usize> {
    fn reject_put_buffer(&self, ring_buffer: &RingBuffer<T>, uid: i64);
}

#[derive(Copy, Clone)]
pub struct DefaultRejectedPutBufferHandler;

impl DefaultRejectedPutBufferHandler {
    pub fn new() -> Self {
        Self {}
    }
}
impl<const T: usize> RejectedPutBufferHandler<T> for DefaultRejectedPutBufferHandler {

    fn reject_put_buffer(&self, ring_buffer: &RingBuffer<T>, uid: i64) {
        println!("reject_put_buffer")
    }
}

unsafe impl Send for DefaultRejectedPutBufferHandler {}
unsafe impl Sync for DefaultRejectedPutBufferHandler {}

pub trait RejectedTakeBufferHandler<const T: usize> {
    fn reject_take_buffer(&self, ring_buffer: &RingBuffer<T>);
}
#[derive(Copy, Clone)]
pub struct DefaultRejectedTakeBufferHandler<const buffer_size: usize>;

impl <const buffer_size: usize>DefaultRejectedTakeBufferHandler<buffer_size> {
    pub fn new() -> Self where Self: RejectedTakeBufferHandler<buffer_size> + 'static {
        Self {}
    }
}

impl<const T: usize> RejectedTakeBufferHandler<T> for DefaultRejectedTakeBufferHandler<T> {
    fn reject_take_buffer(&self, ring_buffer: &RingBuffer<T>) {
        println!("reject_take_buffer")
    }
}
pub struct RingBuffer<const T: usize> {
    buffer_size: usize,
    index_mask: i64,
    slots: [i64; T],
    flags: Vec<AtomicI8>,
    tail: AtomicI64,
    cursor: AtomicI64,
    padding_threshold: usize,
    rejected_put_handler: Box<dyn 'static + RejectedPutBufferHandler<T> >,
    rejected_take_handler: Box<dyn 'static + RejectedTakeBufferHandler<T>>,
    put_lock: Mutex<i8>,
}

unsafe impl <const T: usize>Send for RingBuffer<T>{}
unsafe impl <const T: usize>Sync for RingBuffer<T>{}

impl<'a,const buffer_size: usize> RingBuffer<buffer_size> {
    const START_POINT: i64 = -1;
    const CAN_PUT_FLAG: i8 = 0;
    const CAN_TAKE_FLAG: i8 = 1;

    pub fn from(padding_factor: usize) -> StringResult<Self> {
        assert_eq!(bit_count(buffer_size), 1, "RingBuffer size must be positive");
        // assert_eq!( , );
        assert!(padding_factor > 0 && padding_factor < 100, "RingBuffer size must be positive");

        let mut buffer = RingBuffer {
            buffer_size,
            index_mask: (buffer_size - 1) as i64,
            slots: [0; buffer_size],
            flags: Vec::default(),
            tail: AtomicI64::new(-1),
            cursor: AtomicI64::new(-1),
            padding_threshold: buffer_size * padding_factor / 100,
            rejected_put_handler: Box::new(DefaultRejectedPutBufferHandler::new()),
            rejected_take_handler: Box::new(DefaultRejectedTakeBufferHandler::new()),
            put_lock: Mutex::new(0),
        };
        buffer.flags = {
            let mut flags = Vec::with_capacity(buffer_size);
            for _i in 0..buffer_size {
                flags.push(AtomicI8::new(Self::CAN_PUT_FLAG))
            }
            flags
        };

        StringResult::Ok(buffer)
    }

    pub fn put(&mut self, uid: i64) -> bool {
        self.put_lock.lock();

        let current_tail = self.tail.load(Ordering::Relaxed);

        let current_cursor = self.cursor.load(Ordering::Relaxed);

        let distance = current_tail - if current_cursor == Self::START_POINT { 0 } else { current_cursor };

        if distance == buffer_size as i64 - 1 {
            self.rejected_put_handler.reject_put_buffer(self, uid);
            return false;
        }

        let next_tail_index = self.cal_slot_index(current_tail + 1);

        let tail = next_tail_index as usize;
        if self.flags[tail].load(Ordering::Relaxed) != Self::CAN_PUT_FLAG {
            self.rejected_put_handler.reject_put_buffer(self, uid);
            return false;
        }

        self.slots[tail] = uid;
        self.flags[tail].store(Self::CAN_TAKE_FLAG, Ordering::Relaxed);
        self.tail.store(current_tail + 1, Ordering::Relaxed);

        return true;
    }


    pub fn take(&self) -> Option<i64> {
        let current_cursor = self.cursor.load(Ordering::Relaxed);
        let mut next_cursor= 0;
        let mut prev = self.cursor.load(Ordering::Relaxed);
        loop {
            let next = if prev == self.tail.load(Ordering::Relaxed) { prev } else { prev + 1 };
            let result = self.cursor.compare_exchange(prev, next, Ordering::Relaxed, Ordering::Relaxed);
            match result {
                Ok(ok_prev) => {
                    next_cursor =  if ok_prev == self.tail.load(Ordering::Relaxed) { ok_prev } else { ok_prev + 1 };
                    break
                }
                Err(err_prev) => {
                    prev = err_prev;
                }
            }
        }

        assert!(next_cursor >= current_cursor, "Curosr can't move back");
        let current_tail = self.tail.load(Ordering::Relaxed);
        if current_tail - next_cursor < (self.padding_threshold as i64) {
            // self.buffer_padding_executor.async_padding();
        }

        if next_cursor == current_cursor {
            self.rejected_take_handler.reject_take_buffer(self);
            println!("Rejected take buffer. {}", self);
            return None;
        }

        let next_cursor_index = self.cal_slot_index(next_cursor) as usize;

        assert_eq!(self.flags[next_cursor_index].load(Ordering::Relaxed) , Self::CAN_TAKE_FLAG, "Curosr not in can take status");

        let uid = self.slots[next_cursor_index];
        self.flags[next_cursor_index].store(Self::CAN_PUT_FLAG, Ordering::Relaxed);
        Some(uid)
    }

    fn cal_slot_index(&self, sequence: i64) -> i64 {
        sequence & self.index_mask
    }


    pub fn init_buffer(&mut self) -> StringResult<()>{
        Ok(())
    }
    pub fn set_rejected_put_handler(&mut self, rejected_put_handler: Box< dyn 'static + RejectedPutBufferHandler<buffer_size>>) {
        self.rejected_put_handler = rejected_put_handler;
    }
    pub fn set_rejected_take_handler(&mut self, rejected_take_handler: Box< dyn 'static + RejectedTakeBufferHandler<buffer_size>>) {
        self.rejected_take_handler = rejected_take_handler;
    }
}

impl <const T: usize>Display for  RingBuffer<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",format!("RingBuffer [bufferSize={}, tail={}, cursor={}, paddingThreshold={}",
                          self.buffer_size, self.tail.load(Ordering::Relaxed), self.cursor.load(Ordering::Relaxed), self.padding_threshold))
    }
}