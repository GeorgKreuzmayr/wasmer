use crate::syscalls::types::*;
use tracing::debug;
use wasmer::WasmRef;

pub fn platform_clock_res_get(
    clock_id: __wasi_clockid_t,
    resolution: WasmRef<__wasi_timestamp_t>,
) -> Result<i64, __wasi_errno_t> {
    let resolution_val = match clock_id {
        // resolution of monotonic clock at 10ms, from:
        // https://docs.microsoft.com/en-us/windows/desktop/api/sysinfoapi/nf-sysinfoapi-gettickcount64
        __WASI_CLOCK_MONOTONIC => 10_000_000,
        // TODO: verify or compute this
        __WASI_CLOCK_REALTIME => 1,
        __WASI_CLOCK_PROCESS_CPUTIME_ID => {
            return Err(__WASI_EINVAL);
        }
        __WASI_CLOCK_THREAD_CPUTIME_ID => {
            return Err(__WASI_EINVAL);
        }
        _ => return Err(__WASI_EINVAL),
    };
    Ok(resolution_val)
}

pub fn platform_clock_time_get(
    clock_id: __wasi_clockid_t,
    precision: __wasi_timestamp_t,
) -> Result<i64, __wasi_errno_t> {
    let nanos = match clock_id {
        __WASI_CLOCK_MONOTONIC => {
            let tick_ms = unsafe { winapi::um::sysinfoapi::GetTickCount64() };
            tick_ms * 1_000_000
        }
        __WASI_CLOCK_REALTIME => {
            let duration = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| {
                    debug!("Error in wasi::platform_clock_time_get: {:?}", e);
                    __WASI_EIO
                })?;
            duration.as_nanos() as u64
        }
        __WASI_CLOCK_PROCESS_CPUTIME_ID => {
            unimplemented!("wasi::platform_clock_time_get(__WASI_CLOCK_PROCESS_CPUTIME_ID, ..)")
        }
        __WASI_CLOCK_THREAD_CPUTIME_ID => {
            unimplemented!("wasi::platform_clock_time_get(__WASI_CLOCK_THREAD_CPUTIME_ID, ..)")
        }
        _ => return Err(__WASI_EINVAL),
    };
    Ok(nanos as i64)
}
