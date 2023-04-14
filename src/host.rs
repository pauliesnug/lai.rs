use core::alloc::Layout;

use alloc::sync::Arc;

use super::helper::*;

static mut LAI_HOST: Option<Arc<dyn Host>> = None;

fn get_laihost() -> Arc<dyn Host> {
    unsafe {
        LAI_HOST
            .as_ref()
            .expect("lai: host not initialized")
            .clone()
    }
}

macro_rules! marker {
    ($(fn $name:tt(&self, $($pname:tt: $ptyp:ty),*) -> $ret:ty);*;) => {
        $(fn $name(&self, $($pname: $ptyp),*) -> $ret { unimplemented!() })*
    };
}

pub trait Host {
    marker!(
        fn scan(&self, _signature: &str, _index: usize) -> *const u8;
        fn sleep(&self, _ms: u64) -> ();

        // Port I/O functions:
        fn outb(&self, _port: u16, _value: u8) -> ();
        fn outw(&self, _port: u16, _value: u16) -> ();
        fn outd(&self, _port: u16, _value: u32) -> ();

        fn inb(&self, _port: u16) -> u8;
        fn inw(&self, _port: u16) -> u16;
        fn ind(&self, _port: u16) -> u32;

        // PCI read functions:
        fn pci_readb(&self, _seg: u16, _bus: u8, _slot: u8, _fun: u8, _offset: u16) -> u8;
        fn pci_readw(&self, _seg: u16, _bus: u8, _slot: u8, _fun: u8, _offset: u16) -> u16;
        fn pci_readd(&self, _seg: u16, _bus: u8, _slot: u8, _fun: u8, _offset: u16) -> u32;

        // Maps count bytes from the given physical address and returns
        // a pointer that can be used to access the memory.
        fn map(&self, _address: usize, _count: usize) -> *mut u8;
    );

    unsafe fn alloc(&self, size: usize) -> *mut u8 {
        let layout = Layout::from_size_align_unchecked(size, 16);
        alloc::alloc::alloc_zeroed(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, size: usize) {
        let layout = Layout::from_size_align_unchecked(size, 16);
        alloc::alloc::dealloc(ptr, layout);
    }

    unsafe fn realloc(&self, ptr: *mut u8, new_size: usize, old_size: usize) -> *mut u8 {
        let layout = Layout::from_size_align_unchecked(old_size, 16);
        alloc::alloc::realloc(ptr, layout, new_size)
    }
}

pub fn init(host: Arc<dyn Host>) {
    unsafe {
        assert!(LAI_HOST.is_none());
        LAI_HOST = Some(host);
    }
}

const LAI_DEBUG_LOG: i32 = 1;
const LAI_WARN_LOG: i32 = 2;

#[no_mangle]
extern "C" fn laihost_log(level: i32, message: *const u8) {
    let message = unsafe { c_str_as_str(message) };

    match level {
        LAI_DEBUG_LOG => log::debug!("{message}"),
        LAI_WARN_LOG => log::warn!("{message}"),

        _ => unreachable!("undefined log level: (message={message}, level={level})"),
    };
}

#[no_mangle]
extern "C" fn laihost_panic(message: *const u8) -> ! {
    let message = unsafe { c_str_as_str(message) };
    panic!("{message}");
}

#[no_mangle]
unsafe extern "C" fn laihost_malloc(size: usize) -> *mut u8 {
    get_laihost().alloc(size)
}

#[no_mangle]
unsafe extern "C" fn laihost_free(ptr: *mut u8, size: usize) {
    get_laihost().dealloc(ptr, size)
}

#[no_mangle]
unsafe extern "C" fn laihost_realloc(ptr: *mut u8, new_size: usize, old_size: usize) -> *mut u8 {
    get_laihost().realloc(ptr, new_size, old_size)
}

#[no_mangle]
unsafe extern "C" fn laihost_scan(signature: *const u8, index: usize) -> *const u8 {
    let signature = c_str_as_str(signature);
    get_laihost().scan(signature, index)
}

#[no_mangle]
unsafe extern "C" fn laihost_outb(port: u16, value: u8) {
    get_laihost().outb(port, value)
}

#[no_mangle]
unsafe extern "C" fn laihost_outw(port: u16, value: u16) {
    get_laihost().outw(port, value)
}

#[no_mangle]
unsafe extern "C" fn laihost_outd(port: u16, value: u32) {
    get_laihost().outd(port, value)
}

#[no_mangle]
unsafe extern "C" fn laihost_inb(port: u16) -> u8 {
    get_laihost().inb(port)
}

#[no_mangle]
unsafe extern "C" fn laihost_inw(port: u16) -> u16 {
    get_laihost().inw(port)
}

#[no_mangle]
unsafe extern "C" fn laihost_ind(port: u16) -> u32 {
    get_laihost().ind(port)
}

#[no_mangle]
unsafe extern "C" fn laihost_sleep(ms: u64) {
    get_laihost().sleep(ms)
}

#[no_mangle]
unsafe extern "C" fn laihost_pci_readb(seg: u16, bus: u8, slot: u8, fun: u8, offset: u16) -> u8 {
    get_laihost().pci_readb(seg, bus, slot, fun, offset)
}

#[no_mangle]
unsafe extern "C" fn laihost_pci_readw(seg: u16, bus: u8, slot: u8, fun: u8, offset: u16) -> u16 {
    get_laihost().pci_readw(seg, bus, slot, fun, offset)
}

#[no_mangle]
unsafe extern "C" fn laihost_pci_readd(seg: u16, bus: u8, slot: u8, fun: u8, offset: u16) -> u32 {
    get_laihost().pci_readd(seg, bus, slot, fun, offset)
}

#[no_mangle]
unsafe extern "C" fn laihost_map(address: usize, count: usize) -> *mut u8 {
    get_laihost().map(address, count)
}
