use crate::StringResult;

#[derive(Debug, Default, Clone)]
pub struct BitsAllocator {
    /**
     * Bits for [sign-> second-> workId-> sequence]
     */
    timestamp_bits: i32,
    worker_id_bits: i32,
    sequence_bits: i32,

    max_delta_seconds: i64,
    max_worker_id: i64,
    max_sequence: i64,

    timestamp_shift: i32,
    worker_id_shift: i32
}


impl BitsAllocator {
    const TOTAL_BITS :i32 = 1<< 6;
    const SIGN_BITS :i32 =1;


    pub fn from(timestamp_bits: i32, worker_id_bits: i32, sequence_bits: i32) -> StringResult<BitsAllocator> {
        let allocate_total_bits = Self::SIGN_BITS + timestamp_bits + worker_id_bits + sequence_bits;

        if allocate_total_bits != Self::TOTAL_BITS {
            return StringResult::Err("allocate not enough 64 bits".to_string())
        }

        let allocator = BitsAllocator {
            // initialize bits
            timestamp_bits,
            worker_id_bits,
            sequence_bits,
            // initialize max value
            max_delta_seconds: !(-1 << timestamp_bits),
            max_worker_id: !(-1 << worker_id_bits),
            max_sequence: !(-1 << sequence_bits),
            // initialize shift
            timestamp_shift: worker_id_bits + sequence_bits,
            worker_id_shift: sequence_bits
        };

        Result::Ok(allocator)
    }

    pub fn allocate(&self, delta_seconds: i64, worker_id: i64, sequence: i64) -> i64 {
        return (delta_seconds << self.timestamp_shift) | (worker_id << self.worker_id_shift) | sequence
    }
}


impl BitsAllocator {
    pub fn timestamp_bits(&self) -> i32 {
        self.timestamp_bits
    }
    pub fn worker_id_bits(&self) -> i32 {
        self.worker_id_bits
    }
    pub fn sequence_bits(&self) -> i32 {
        self.sequence_bits
    }
    pub fn max_delta_seconds(&self) -> i64 {
        self.max_delta_seconds
    }
    pub fn max_worker_id(&self) -> i64 {
        self.max_worker_id
    }
    pub fn max_sequence(&self) -> i64 {
        self.max_sequence
    }
}