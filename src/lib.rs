#![no_std]

mod helper;
mod host;
mod raw;

use alloc::boxed::Box;
pub use host::*;
pub use raw::*;

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
pub enum Error {
    None,
    OutOfMemory,
    TypeMismatch,
    NoSuchNode,
    OutOfBounds,
    ExecutionFailure,
    IllegalArguments,
    UnexpectedResult,
    EndReached,
    NotSupported,
}

extern crate alloc;

/// Initializes the ACPI revision.
#[inline]
pub fn set_acpi_revision(revision: i32) {
    unsafe { raw::lai_set_acpi_revision(revision) }
}

/// Creates the ACPI namespace.
#[inline]
pub fn create_namespace() {
    unsafe { raw::lai_create_namespace() }
}

/// Enables ACPI SCI.
pub fn enable_acpi(mode: u32) {
    unsafe { assert_eq!(raw::lai_enable_acpi(mode), raw::LAI_SUCCESS) }
}

/// Enters a sleep state.
pub fn enter_sleep(sleep_state: u8) {
    unsafe { assert_eq!(raw::lai_enter_sleep(sleep_state), raw::LAI_SUCCESS) }
}

pub fn reset() {
    unsafe { assert_eq!(raw::lai_acpi_reset(), raw::LAI_SUCCESS) }
}

pub fn pci_route_pin(
    seg: u16,
    bus: u8,
    slot: u8,
    function: u8,
    pin: u8,
) -> Result<Box<AcpiResource>, Error> {
    let mut dest = Box::new(AcpiResource::default());
    unsafe {
        let result = raw::lai_pci_route_pin(
            &mut *dest as *mut AcpiResource,
            seg,
            bus,
            slot,
            function,
            pin,
        );

        if result != LAI_SUCCESS {
            let err: Error = core::mem::transmute(result);
            Err(err)
        } else {
            Ok(dest)
        }
    }
}
