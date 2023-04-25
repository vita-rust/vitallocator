// Copyright (c) 2018 Martin Larralde (martin.larralde@ens-paris-saclay.fr)
//
// Licensed under MIT license (the COPYING file). This file may not be
// copied, modified, or distributed except according to those terms.

//! A Rust allocator interface to the PS Vita kernel allocator.

#![no_std]
#![feature(allocator_api)]

extern crate psp2_sys;

mod utils;

use core::alloc::AllocError;
use core::alloc::Allocator;
use core::alloc::Layout;
use core::cell::Cell;
use core::cmp::max;
use core::mem::size_of;
use core::ptr::NonNull;

use psp2_sys::kernel::sysmem::sceKernelAllocMemBlock;
use psp2_sys::kernel::sysmem::sceKernelFindMemBlockByAddr;
use psp2_sys::kernel::sysmem::sceKernelFreeMemBlock;
use psp2_sys::kernel::sysmem::sceKernelGetMemBlockBase;
use psp2_sys::kernel::sysmem::sceKernelGetMemBlockInfoByAddr;
use psp2_sys::kernel::sysmem::SceKernelAllocMemBlockOpt;
use psp2_sys::kernel::sysmem::SceKernelMemBlockInfo;
use psp2_sys::kernel::sysmem::SceKernelMemBlockType::SCE_KERNEL_MEMBLOCK_TYPE_USER_RW;
use psp2_sys::kernel::sysmem::SceKernelMemoryAccessType::SCE_KERNEL_MEMORY_ACCESS_R;
use psp2_sys::types::SceUID;
use psp2_sys::void;

/// A Rust interface to the PS Vita kernel allocator.
///
/// Uses the function [`sceKernelAllocMemBlock`] to allocate blocks of memory.
/// This allocator will only create blocks of `4kB`-aligned memory. It won't perform
/// the alignement itself, so you have to make sure the `size` requested [`Layout`]
/// fits this constraint !
///
/// It is not thread safe, so you'll have to rely on an external synchronisation
/// primitive, for instance by wrapping the allocator in a [`Mutex`]. As such, this
/// allocator cannot be used directly as a global allocator.
///
/// [`sceKernelAllocMemBlock`]: https://docs.vitasdk.org/group__SceSysmemUser.html
/// [`Alloc`]: https://doc.rust-lang.org/nightly/core/alloc/trait.Alloc.html
/// [`Layout`]: https://doc.rust-lang.org/nightly/core/alloc/struct.Layout.html
/// [`Mutex`]: struct.Mutex.html
pub struct Vitallocator {
    block_count: Cell<usize>,
}

impl Default for Vitallocator {
    fn default() -> Self {
        Vitallocator::new()
    }
}

impl Vitallocator {
    /// Create a new kernel allocator.
    pub const fn new() -> Self {
        Vitallocator { block_count: Cell::new(0) }
    }
}

unsafe impl Allocator for Vitallocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        // Prepare the options to pass to SceKernelAllocMemBlock
        let mut options = SceKernelAllocMemBlockOpt {
            size: size_of::<SceKernelAllocMemBlockOpt>() as u32,
            attr: SCE_KERNEL_MEMORY_ACCESS_R as u32,
            alignment: layout.align() as u32,
            uidBaseBlock: 0,
            strBaseBlockName: ::core::ptr::null(),
            flags: 0,
            reserved: [0; 10],
        };

        // Prepare the pointer
        let mut basep: *mut void = ::core::ptr::null_mut::<u8>() as *mut _;

        // Define a new name for the block (writing the block count as hex)
        let mut name: [u8; 18] = *b"__rust_0x00000000\0";
        utils::write_hex(self.block_count.get(), &mut name[9..16]);

        let size = max(layout.size(), 4096);

        // Allocate the memory block
        let uid: SceUID = unsafe {
            sceKernelAllocMemBlock(
                (&name).as_ptr(),
                SCE_KERNEL_MEMBLOCK_TYPE_USER_RW,
                size as i32,
                &mut options as *mut _,
            )
        };
        if uid < 0 {
            return Err(AllocError);
        }

        // Imcrease the block count: to the kernel, we allocated a new block.
        // `wrapping_add` avoids a panic when the total number of allocated blocks
        // exceeds `usize::max_value()`. An undefined behaviour is still expected
        // from the kerne since some block could possibly be named the same.
        self.block_count.set(self.block_count.get().wrapping_add(1));

        // Get the adress of the allocated location
        unsafe {
            if sceKernelGetMemBlockBase(uid, &mut basep as *mut *mut void) < 0 {
                sceKernelFreeMemBlock(uid); // avoid memory leak if the block cannot be used
                return Err(AllocError);
            }
        }
        // Return the obtained non-null, opaque pointer
        let slice = unsafe {
            core::slice::from_raw_parts_mut(basep as *mut u8, size)
        };
        NonNull::new(slice as *mut _).ok_or(AllocError)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        // Get the size of the pointer memory block
        let mut info: SceKernelMemBlockInfo = ::core::mem::uninitialized();
        sceKernelGetMemBlockInfoByAddr(ptr.as_ptr() as *mut void, (&mut info) as *mut _);

        // Find the SceUID
        let uid = sceKernelFindMemBlockByAddr(ptr.as_ptr() as *mut void, info.size);

        // Free the memory block
        sceKernelFreeMemBlock(uid);
    }
}
