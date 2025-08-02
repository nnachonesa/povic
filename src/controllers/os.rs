use sysinfo::System;

pub fn os() -> String {
    let info: os_info::Info = os_info::get();
    let binding: Option<&str> = info.edition();
    let osname: String = binding.unwrap_or("Unknown").to_string();

    let binding: Option<&str> = info.architecture();
    let architecture: &str = binding.expect("wtf");
    let binding: Option<String> = System::host_name();
    let hostname: &str = binding.as_deref().expect("wtf");

    format!(
        "OS: {} {}\nKernel: {}\nHost: {}",
        osname,
        architecture,
        info.version().to_string().trim_matches('"'),
        hostname,
        // windows::fetch_latest_ps_version()
    )
}
