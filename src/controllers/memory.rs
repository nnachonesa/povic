use sysinfo::System;

pub fn memory() -> String {
    let mut sys: System = System::new_all();
    sys.refresh_all();

    format!(
        "Memoria: {} MB / {} MB",
        sys.used_memory() / 1024 / 1024,
        sys.total_memory() / 1024 / 1024
    )
}