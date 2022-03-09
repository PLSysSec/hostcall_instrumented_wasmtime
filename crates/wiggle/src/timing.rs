use std::collections::HashMap;
use std::cell::RefCell;
use std::thread;
use core::arch::x86_64::{_rdtsc,__rdtscp,__cpuid_count};

// name of hostcall -> Vec<nanoseconds>
pub type ResultsType = HashMap<String, Vec<f64>>;

fn wasi_results_init() -> RefCell<ResultsType> {
    let mut h: ResultsType = HashMap::new();
    h.insert("args_get".to_owned(), Vec::new());
    h.insert("args_sizes_get".to_owned(), Vec::new());
    h.insert("proc_exit".to_owned(), Vec::new());
    h.insert("environ_sizes_get".to_owned(), Vec::new());
    h.insert("environ_get".to_owned(), Vec::new());
    h.insert("fd_prestat_get".to_owned(), Vec::new());
    h.insert("fd_write".to_owned(), Vec::new());
    h.insert("fd_read".to_owned(), Vec::new());
    h.insert("fd_close".to_owned(), Vec::new());
    h.insert("fd_seek".to_owned(), Vec::new());
    h.insert("clock_time_get".to_owned(), Vec::new());
    h.insert("clock_res_get".to_owned(), Vec::new());
    h.insert("fd_advise".to_owned(), Vec::new());
    h.insert("fd_allocate".to_owned(), Vec::new());
    h.insert("fd_datasync".to_owned(), Vec::new());
    h.insert("fd_fdstat_get".to_owned(), Vec::new());
    h.insert("fd_fdstat_set_flags".to_owned(), Vec::new());
    h.insert("fd_filestat_get".to_owned(), Vec::new());
    h.insert("fd_filestat_set_size".to_owned(), Vec::new());
    h.insert("fd_filestat_set_times".to_owned(), Vec::new());
    h.insert("fd_pread".to_owned(), Vec::new());
    h.insert("fd_prestat_dir_name".to_owned(), Vec::new());
    h.insert("fd_pwrite".to_owned(), Vec::new());
    h.insert("fd_readdir".to_owned(), Vec::new());
    h.insert("fd_renumber".to_owned(), Vec::new());
    h.insert("fd_sync".to_owned(), Vec::new());
    h.insert("fd_tell".to_owned(), Vec::new());
    h.insert("path_create_directory".to_owned(), Vec::new());
    h.insert("path_filestat_get".to_owned(), Vec::new());
    h.insert("path_filestat_set_times".to_owned(), Vec::new());
    h.insert("path_link".to_owned(), Vec::new());
    h.insert("path_open".to_owned(), Vec::new());
    h.insert("path_readlink".to_owned(), Vec::new());
    h.insert("path_remove_directory".to_owned(), Vec::new());
    h.insert("path_rename".to_owned(), Vec::new());
    h.insert("path_symlink".to_owned(), Vec::new());
    h.insert("path_unlink_file".to_owned(), Vec::new());
    h.insert("poll_oneoff".to_owned(), Vec::new());
    h.insert("proc_raise".to_owned(), Vec::new());
    h.insert("random_get".to_owned(), Vec::new());
    h.insert("sched_yield".to_owned(), Vec::new());
    h.insert("sock_recv".to_owned(), Vec::new());
    h.insert("sock_send".to_owned(), Vec::new());
    h.insert("sock_shutdown".to_owned(), Vec::new());
    h.insert("socket".to_owned(), Vec::new());
    h.insert("sock_connect".to_owned(), Vec::new());
    RefCell::new(h)
}

thread_local! {
    pub static results: RefCell<ResultsType> = wasi_results_init();
}

#[inline]
pub fn start_timer() -> u64 {
    unsafe {
        __cpuid_count(0, 0);
        _rdtsc() as u64
    }
}

#[inline]
pub fn stop_timer() -> u64 {
    unsafe {
        let mut junk: u32 = 0;
        let ans: u64 = __rdtscp(&mut junk);
        __cpuid_count(0, 0);
        ans
    }
}


pub fn push_result(name: &str, start: u64, end: u64){
    // println!("name: {:?}", name);
    results.with(|r| {
        let mut index = r.borrow_mut();
        let vec = index.get_mut(&name.to_owned()).unwrap();
        let ticks = end - start;
        vec.push(ticks as f64 / 2.1); // convert to nanoseconds using 2.1 GHZ clock (elk)
    });
}

// let _start = start_timer()
// let _end = stop_timer()
// results["this_func"].push() 
