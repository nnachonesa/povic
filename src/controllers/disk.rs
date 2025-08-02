use sysinfo::Disks;

pub fn disk() -> String {
    let mut disks_info: String = String::new();
    let disks = Disks::new_with_refreshed_list();

    for (index, disk) in disks.iter().enumerate() {
        if index > 0 {
            disks_info.push_str(" | ");
        }
        let mount: std::borrow::Cow<str> = disk.mount_point().to_string_lossy();
        let mount2: String = mount.to_string().replace(r"\", "");
        let space: u64 = disk.total_space() / 1024 / 1024 / 1024;
        disks_info.push_str(&format!("({mount2}) {space} GB"));
    }

    format!("Discos:\n{disks_info}")
}
