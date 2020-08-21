use winapi::{km::wdm::PEPROCESS, shared::ntdef::NTSTATUS};

pub type PVOID = *mut core::ffi::c_void;
pub type HANDLE = PVOID;

extern "system" {
    pub fn MmIsAddressValid(virtual_address: PVOID) -> bool;

    pub fn PsLookupProcessByProcessId(process_id: HANDLE, process: *mut PEPROCESS) -> NTSTATUS;

    pub fn ObfDereferenceObject(object: PVOID);
}
