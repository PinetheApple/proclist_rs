use std::process;
use windows_sys::Win32::{
    Foundation::{GetLastError, HANDLE, INVALID_HANDLE_VALUE},
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32First, PROCESSENTRY32, TH32CS_SNAPPROCESS,
        },
        ProcessStatus::PROCESS_MEMORY_COUNTERS,
    },
};

fn get_process_list() {
    let mut pe32 = PROCESSENTRY32 {
        dwSize: 304,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0; 260],
    };
    let mut h_process: HANDLE = 0;
    let mut dw_priority_class: u32 = 0;
    let mut pm_counters = PROCESS_MEMORY_COUNTERS {
        cb: 0,
        PageFaultCount: 0,
        PeakWorkingSetSize: 0,
        WorkingSetSize: 0,
        QuotaPeakPagedPoolUsage: 0,
        QuotaPagedPoolUsage: 0,
        QuotaPeakNonPagedPoolUsage: 0,
        QuotaNonPagedPoolUsage: 0,
        PagefileUsage: 0,
        PeakPagefileUsage: 0,
    };

    unsafe {
        let h_snapshot: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if h_snapshot == INVALID_HANDLE_VALUE {
            println!(
                "Unable to create ToolHelp32 snapshot of processes \nError: {}",
                GetLastError()
            );
            process::exit(1)
        }

        if Process32First(h_snapshot, &mut pe32) == 0 {
            println!(
                "Couldn't get information about [System process] \nError: {}",
                GetLastError()
            );
            process::exit(1);
        }

        loop {}
    }
}

fn display_info() {}

fn display_value() {}

fn display_headers() {}

fn main() {
    get_process_list();
}
