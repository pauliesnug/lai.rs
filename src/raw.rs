#![allow(non_camel_case_types)]

pub(crate) const LAI_SUCCESS: i32 = 0;

type lai_api_error_t = i32;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct AcpiResource {
    pub ty: u8,
    pub base: u64,
    pub length: u64,
    pub address_space: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub irq_flags: u8,
}

extern "C" {
    pub(crate) fn lai_set_acpi_revision(revision: i32);
    pub(crate) fn lai_create_namespace();
    pub(crate) fn lai_enable_acpi(mode: u32) -> lai_api_error_t;
    pub(crate) fn lai_enter_sleep(sleep_state: u8) -> lai_api_error_t;
    pub(crate) fn lai_acpi_reset() -> lai_api_error_t;
    pub(crate) fn lai_pci_route_pin(
        dest: *mut AcpiResource,
        seg: u16,
        bus: u8,
        slot: u8,
        function: u8,
        pin: u8,
    ) -> lai_api_error_t;
}
