use std::{mem::size_of_val, process};
use tabled::{builder::Builder, settings::Style};
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
    let mut builder = Builder::default();

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

        generate_headers(&mut builder);

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

                add_info(&pe32, priority_class, &pm_counters, &mut builder);
                CloseHandle(h_process);
            }

            if Process32Next(h_snapshot, &mut pe32) == 0 {
                break;
            }
        }

        CloseHandle(h_snapshot);
    }

    let output = builder.build().with(Style::psql()).to_string();
    println!("{}", output);
}

fn add_info(
    pe32: &PROCESSENTRY32,
    priority_class: u32,
    pmc: &PROCESS_MEMORY_COUNTERS,
    builder: &mut Builder,
) {
    builder.push_record(vec![
        String::from_utf8(pe32.szExeFile[..].to_vec()).unwrap(),
        pe32.th32ProcessID.to_string(),
        pe32.cntThreads.to_string(),
        pe32.th32ParentProcessID.to_string(),
        pe32.pcPriClassBase.to_string(),
        priority_class.to_string(),
        pmc.PageFaultCount.to_string(),
        get_value_and_unit(pmc.WorkingSetSize),
        get_value_and_unit(pmc.PeakWorkingSetSize),
        get_value_and_unit(pmc.PagefileUsage),
        get_value_and_unit(pmc.PeakPagefileUsage),
    ])
}

fn get_value_and_unit(value_in_bytes: usize) -> String {
    if value_in_bytes > 1048576 {
        (value_in_bytes / 1048576).to_string() + "MB"
    } else {
        (value_in_bytes / 1024).to_string() + "KB"
    }
}

fn generate_headers(builder: &mut Builder) {
    builder.push_record(vec![
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
        "Peak Pagefile Usage",
    ]);
}

fn main() {
    get_process_list();
}
