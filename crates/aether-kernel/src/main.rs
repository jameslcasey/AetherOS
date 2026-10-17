#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

/// Unforgeable kernel capability stored in per-process C-Space
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Capability {
    pub object_id: u64,
    pub cap_type: CapType,
    pub rights: u32,
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapType {
    None = 0,
    MemoryFrame = 1,
    IpcEndpoint = 2,
    DeviceMmio = 3,
    AetherFsObject = 4,
    AcceleratorTpcSlice = 5,
    KvCachePool = 6,
    AgentToolEndpoint = 7,
}

#[repr(C, align(64))]
#[derive(Clone, Copy)]
pub struct IpcDescriptor {
    pub opcode: u16,
    pub flags: u16,
    pub cap_transfer_slot: u32,
    pub target_endpoint_cap: u64,
    pub shared_frame_cap: u64,
    pub payload_offset: u32,
    pub payload_len: u32,
    pub inline_data: [u64; 3],
}

#[repr(C, align(64))]
#[derive(Clone, Copy)]
pub struct LithosComputeDescriptor {
    pub tpc_slice_cap: u64,
    pub shader_object_cap: u64,
    pub assigned_tpc_mask: u64,
    pub total_grid_blocks: u32,
    pub atom_quantum_blocks: u16,
    pub qos_power_profile: u16,
    pub tensor_io_cap: u64,
}

#[repr(C, align(64))]
#[derive(Clone, Copy)]
pub struct AiosAgentContext {
    pub agent_id: u64,
    pub kv_cache_cap: u64,
    pub logits_state_cap: u64,
    pub slo_deadline_us: u64,
    pub sequence_token_offset: u32,
    pub remaining_tool_quota: u16,
    pub state_flags: u16,
}

#[repr(C, align(4096))]
pub struct SharedIpcRing<const N: usize> {
    pub head: AtomicU32,
    pub tail: AtomicU32,
    pub ring_mask: u32,
    pub flags: AtomicU32,
    pub _pad: [u8; 48],
    pub descriptors: [IpcDescriptor; N],
}

impl<const N: usize> SharedIpcRing<N> {
    pub fn try_push(&self, desc: IpcDescriptor) -> Result<(), ()> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);
        if tail.wrapping_sub(head) >= N as u32 {
            return Err(());
        }
        let idx = (tail & self.ring_mask) as usize;
        unsafe {
            let slot = &self.descriptors[idx] as *const IpcDescriptor as *mut IpcDescriptor;
            slot.write_volatile(desc);
        }
        self.tail.store(tail.wrapping_add(1), Ordering::Release);
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    aether_hal::early_init();

    let _cap = Capability {
        object_id: 1,
        cap_type: CapType::KvCachePool,
        rights: 1,
    };
    let _ctx = AiosAgentContext {
        agent_id: 42,
        kv_cache_cap: 100,
        logits_state_cap: 101,
        slo_deadline_us: 1000,
        sequence_token_offset: 0,
        remaining_tool_quota: 10,
        state_flags: 0,
    };

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
