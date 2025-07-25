use sysinfo::{Disks, System};

extern crate sysinfo;

pub fn disk() -> String {
    let mut sys: System = System::new_all();
    let mut disks_info: String = String::new();
    let disks: Disks = Disks::new_with_refreshed_list();
    sys.refresh_all();
    for (index, disk) in disks.iter().enumerate() {
        if index > 0 {
            disks_info.push_str(" | ");
        }
        let mount: std::borrow::Cow<str> = disk.mount_point().to_string_lossy();
        let mount2: String = mount.to_string().replace(r"\", "");
        let space: u64 = disk.total_space() / 1024 / 1024 / 1024;
        disks_info.push_str(&format!("({}) {} GB", mount2, space));
    }

    format!("Discos:\n{}", disks_info)
}
