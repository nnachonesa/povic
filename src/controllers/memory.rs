use sysinfo::System;

pub fn memory(sys: &mut System) -> String {
    sys.refresh_memory();

    format!(
        "Memoria: {} MB / {} MB",
        sys.used_memory() / 1024 / 1024,
        sys.total_memory() / 1024 / 1024
    )
}
