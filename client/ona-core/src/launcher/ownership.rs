use std::process::Child;

#[derive(Debug)]
pub struct ProcessOwnership {
    #[cfg(windows)]
    job: Option<RawJobHandle>,
}

#[cfg(windows)]
#[derive(Debug)]
struct RawJobHandle(isize);

#[cfg(windows)]
unsafe impl Send for RawJobHandle {}

#[cfg(windows)]
unsafe impl Sync for RawJobHandle {}

impl ProcessOwnership {
    pub fn attach(child: &Child) -> Self {
        #[cfg(windows)]
        {
            Self {
                job: windows_attach_to_kill_on_close_job(child)
                    .ok()
                    .map(|handle| RawJobHandle(handle.0 as isize)),
            }
        }

        #[cfg(not(windows))]
        {
            let _ = child;
            Self {}
        }
    }
}

impl Drop for ProcessOwnership {
    fn drop(&mut self) {
        #[cfg(windows)]
        if let Some(job) = self.job.take() {
            unsafe {
                let _ = windows::Win32::Foundation::CloseHandle(
                    windows::Win32::Foundation::HANDLE(job.0 as *mut core::ffi::c_void),
                );
            }
        }
    }
}

#[cfg(windows)]
fn windows_attach_to_kill_on_close_job(
    child: &Child,
) -> windows::core::Result<windows::Win32::Foundation::HANDLE> {
    use std::os::windows::io::AsRawHandle;
    use windows::Win32::{
        Foundation::HANDLE,
        System::JobObjects::{
            AssignProcessToJobObject, CreateJobObjectW, JobObjectExtendedLimitInformation,
            SetInformationJobObject, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
            JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
        },
    };

    let job = unsafe { CreateJobObjectW(None, None)? };
    let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
    info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;

    unsafe {
        SetInformationJobObject(
            job,
            JobObjectExtendedLimitInformation,
            &mut info as *mut _ as *const _,
            std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
        )?;

        let process = HANDLE(child.as_raw_handle());
        AssignProcessToJobObject(job, process)?;
    }

    Ok(job)
}
