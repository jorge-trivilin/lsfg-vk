use procfs::{process, ProcResult};

/// Common emulator process names that should be shown even if Vulkan isn't loaded yet
const KNOWN_EMULATORS: &[&str] = &[
    "pcsx2-qt",
    "PCSX2",
    "pcsx2",
    "dolphin-emu",
    "dolphin-emu-qt",
    "dolphin-emu-wx",
    "retroarch",
    "rpcs3",
    "duckstation-qt",
    "duckstation",
];

pub fn find_vulkan_processes() -> ProcResult<Vec<(String, String)>> {
    let mut processes = Vec::new();
    let apps = process::all_processes()?;
    for app in apps {
        let Ok(prc) = app else { continue; };

        let Ok(maps) = proc_maps::get_process_maps(prc.pid()) else {
            continue;
        };

        // check if vulkan is loaded
        let has_vulkan = maps.iter()
            .filter_map(|map| map.filename())
            .map(|filename| filename.to_string_lossy().to_string())
            .any(|filename| filename.to_lowercase().contains("vulkan"));

        // find executed binary
        let mut exe = prc.exe()?.to_string_lossy().to_string();
        let exe_name = exe.split('/').last().unwrap_or(&exe);

        // check if this is a known emulator
        let is_known_emulator = KNOWN_EMULATORS.iter()
            .any(|&emulator| exe_name.to_lowercase().contains(&emulator.to_lowercase()));

        // skip if neither vulkan loaded nor known emulator
        if !has_vulkan && !is_known_emulator {
            continue;
        }

        // replace binary with exe for wine apps
        if exe.contains("wine") || exe.contains("proton") {
            let result = maps.iter()
                .filter_map(|map| map.filename())
                .map(|filename| filename.to_string_lossy().to_string())
                .find(|filename| filename.ends_with(".exe"));

            if let Some(exe_name) = result {
                exe = exe_name;
            }
        }

        // split off last part of the path
        exe = exe.split('/').last().unwrap_or(&exe).to_string();

        // format process information
        let pid = prc.pid();
        let process_info = format!("PID {}: {}", pid, exe);
        processes.push((process_info, exe));
    }

    Ok(processes)
}
