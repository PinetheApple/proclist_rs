use std::{
    io::{self, Write},
    mem::size_of_val,
    process,
};
use windows_sys::Win32::{
    Foundation::{CloseHandle, GetLastError, HANDLE, INVALID_HANDLE_VALUE},
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32,
            TH32CS_SNAPPROCESS,
        },
        ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS},
        Threading::{GetPriorityClass, OpenProcess, PROCESS_ALL_ACCESS},
    },
};

fn get_process_list() {
    let mut pe32 = PROCESSENTRY32 {
        dwSize: 0,
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
    pe32.dwSize = size_of_val(&pe32) as u32;

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

        display_headers();

        loop {
            let h_process = OpenProcess(PROCESS_ALL_ACCESS, 0, pe32.th32ProcessID);
            if h_process != 0 {
                let priority_class = GetPriorityClass(h_process);

                if GetProcessMemoryInfo(
                    h_process,
                    &mut pm_counters,
                    size_of_val(&pm_counters) as u32,
                ) == 0
                {
                    println!("Failed to get information regarding process memory for process with pid {} | Error: {}", pe32.th32ProcessID, GetLastError());
                }

                display_info(&pe32, priority_class, &pm_counters);
                CloseHandle(h_process);
            }

            if Process32Next(h_snapshot, &mut pe32) == 0 {
                break;
            }
        }
    }
}

fn display_info(pe32: &PROCESSENTRY32, priority_class: u32, pmc: &PROCESS_MEMORY_COUNTERS) {
    print!(
        "{: <60} {: <6} {: <14} {: <13} {: <16} {: <17} {: <15}",
        String::from_utf8(pe32.szExeFile[..].to_vec()).unwrap(),
        pe32.th32ProcessID,
        pe32.cntThreads,
        pe32.th32ParentProcessID,
        pe32.pcPriClassBase,
        priority_class,
        pmc.PageFaultCount
    );
    io::stdout().flush().unwrap();
    display_value(pmc.WorkingSetSize, 11);
    display_value(pmc.PeakWorkingSetSize, 15);
    display_value(pmc.PagefileUsage, 14);
    display_value(pmc.PeakPagefileUsage, 20);
    println!();
}

fn display_value(value_in_bytes: usize, width: u8) {
    if value_in_bytes > 1048576 {
        print!("{: <1$}MB", value_in_bytes / 1048576, width as usize);
    } else {
        print!("{: <1$}KB", value_in_bytes / 1024, width as usize);
    }
    io::stdout().flush().unwrap();
}

fn display_headers() {
    println!(
        "{: <60} {: <6} {: <14} {: <13} {: <16} {: <17} {: <15} {: <13} {: <17} {: <16} {: <23}",
        "Process Name",
        "PID",
        "Thread Count",
        "Parent PID",
        "Base Priority",
        "Priority Class",
        "Page Faults",
        "Mem Usage",
        "Peak Mem Usage",
        "Pagefile Usage",
        "Peak Pagefile Usage"
    );
    println!("=================================================================================================================================================================================================================")
}

fn main() {
    get_process_list();
}
