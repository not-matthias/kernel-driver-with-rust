use crate::include::{ObfDereferenceObject, PsLookupProcessByProcessId};
use winapi::km::wdm::PEPROCESS;
use winapi::shared::ntdef::NT_SUCCESS;

pub struct Process {
    pub process: PEPROCESS,
}

impl Process {
    pub fn by_id(process_id: u64) -> Option<Self> {
        let mut process = core::ptr::null_mut();

        let status = unsafe { PsLookupProcessByProcessId(process_id as _, &mut process) };
        if NT_SUCCESS(status) {
            Some(Self { process })
        } else {
            None
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        if !self.process.is_null() {
            unsafe { ObfDereferenceObject(self.process as _) }
        }
    }
}
