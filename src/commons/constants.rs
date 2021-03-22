pub const CHAIN_VERSION: u32 = 2;

pub const ZONE_DIFFICULTY: u32 = 26;
pub const BLOCK_DIFFICULTY: u32 = 24;
pub const LOCKER_DIFFICULTY: u32 = 18;
pub const KEYSTORE_DIFFICULTY: usize = 23;

pub const LOCKER_BLOCK_START: u64 = 12;
pub const LOCKER_BLOCK_LOCKERS: u64 = 6;
pub const LOCKER_BLOCK_SIGNS: u64 = 4;
pub const LOCKER_BLOCK_TIME: i64 = 300;
pub const LOCKER_BLOCK_INTERVAL: u64 = 50;

pub const FULL_BLOCKS_INTERVAL: i64 = 86400; // One day in seconds

pub const ZONE_MAX_LENGTH: usize = 10;
pub const MAX_RECONNECTS: u32 = 5;