//! Provides functions to interact with memory.

use core::{fmt, ptr, mem};

use crate::sys::*;
use crate::utils::{self, Result};

///Convenient wrapper over [MEMORY_BASIC_INFORMATION](https://msdn.microsoft.com/en-us/library/windows/desktop/aa366775(v=vs.85).aspx)
pub struct Info(pub MEMORY_BASIC_INFORMATION);

impl Info {
    #[inline]
    ///Returns region's base address
    ///
    ///Memory can be read from this address up to `<base> + <region size>`
    ///
    ///You can get this pointer by accessing inner structure `self.0.BaseAddress`
    pub fn base_addr(&self) -> usize {
        self.0.BaseAddress as usize
    }

    #[inline]
    ///Returns Allocation base.
    ///
    ///You can get this pointer by accessing inner structure `self.0.AllocationBase`
    pub fn alloc_base(&self) -> usize {
        self.0.AllocationBase as usize
    }

    #[inline]
    ///Returns memory size.
    pub fn size(&self) -> SIZE_T {
        self.0.RegionSize
    }

    #[inline]
    ///Returns whether memory is committed or not.
    ///
    ///Basically it is in use currently
    pub fn is_commit(&self) -> bool {
        self.0.State == MEM_COMMIT
    }

    #[inline]
    ///Returns whether memory is free or not.
    pub fn is_free(&self) -> bool {
        self.0.State == MEM_FREE
    }

    #[inline]
    ///Returns whether memory is reserved or not.
    ///
    ///This space is not backed by actual physical storage.
    pub fn is_reserved(&self) -> bool {
        self.0.State == MEM_RESERVE
    }
}

impl fmt::Debug for Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Info {{ BaseAddress={:p}, AllocationBase={:p}, AllocationProtect={}, RegionSize={}, State={}, Protect={}, Type={} }}",
                   self.0.BaseAddress, self.0.AllocationBase, self.0.AllocationProtect, self.0.RegionSize, self.0.State, self.0.Protect, self.0.Type)
    }
}

///Iterator over memory regions
///
///Returns memory addresses
pub struct Virtual {
    handle: HANDLE,
    addr: *const u8
}

impl Virtual {
    ///Creates new instance to retrieve memory regions of provided process.
    ///
    ///It is assumed that handle is valid, if it is not then nothing can be retrieved.
    pub fn new(handle: HANDLE) -> Self {
        Virtual {
            handle,
            addr: ptr::null()
        }
    }
}

impl Iterator for Virtual {
    type Item = Info;

    fn next(&mut self) -> Option<Self::Item> {
        virtual_query_ex(self.handle, self.addr as *const c_void).ok().map(|info| {
            self.addr = unsafe { self.addr.add(info.size()) };
            info
        })
    }
}

///Retrieves information about virtual memory of specified process.
///
///Wrapper over `VirtualQueryEx`
///
///In case no information is available i.e. `VirtualQueryEx` returns 0
///function returns None.
///
///# Note:
///
///When using this function on process of different bitness it might not work correctly.
pub fn virtual_query_ex(handle: HANDLE, base: *const c_void) -> Result<Info> {
    let mut info: MEMORY_BASIC_INFORMATION = unsafe { mem::zeroed() };

    if unsafe { VirtualQueryEx(handle, base, &mut info as *mut _, mem::size_of_val(&info) as SIZE_T) } != 0 {
        Ok(Info(info))
    }
    else {
        Err(utils::get_last_error())
    }
}
