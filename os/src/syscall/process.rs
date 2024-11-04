//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,
        get_task_info, alllocate_memory, free_memory, current_user_token
    },
    timer::get_time_us,
    mm::translated_byte_buffer,
};

use core::{mem::size_of, slice};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

/// This part is from team member
fn get_address_for_user(from: usize, to: usize, size: usize) {
    let pg_token = current_user_token();
    let mut to_buf = translated_byte_buffer(pg_token, to as *const u8, size);
    let from_slice = unsafe { slice::from_raw_parts(from as *const u8, size) };
    let mut count = 0;
    for buf_slice in to_buf.iter_mut() {
        let target_len = buf_slice.len();
        buf_slice.copy_from_slice(&from_slice[count..count + target_len]);
        count += target_len
    }
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let ts = TimeVal {
        sec: get_time_us() / 1_000_000,
        usec: get_time_us() % 1_000_000,
    };
    get_address_for_user(
        (&ts) as *const TimeVal as usize,
        _ts as usize,
        size_of::<TimeVal>(),
    );
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let mut ti = TaskInfo {
        status: TaskStatus::Running,
        syscall_times: [0; MAX_SYSCALL_NUM],
        time: 0,
    };
    get_task_info(&mut ti);
    get_address_for_user(
        (&ti) as *const TaskInfo as usize,
        _ti as usize,
        size_of::<TaskInfo>(),
    );
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap");
    alllocate_memory(_start, _len, _port) 
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap");
    free_memory(_start, _len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
