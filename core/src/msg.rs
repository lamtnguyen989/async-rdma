

/// Location of a registered memory region that peer may access remotely
#[derive(Clone, Copy, Debug)]
pub struct MemoryRegionInfo 
{
    pub address: u64,
    pub remote_key: u32,
    pub n_slots: u32,
    pub slot_bytes: usize
}

/// Timings struct to measure pipeline performance
#[derive(Clone, Copy, Debug, Default)]
pub struct Timings 
{
    
}

#[derive(Clone, Copy, Debug)]
pub enum Message 
{

}