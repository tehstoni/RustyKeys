# CMSTP UAC Bypass Implementation in Rust

This project is a Rust implementation of a UAC (User Account Control) bypass technique that was originally researched by Oddvar Moe and implemented in C#/.NET by zc00l. The technique leverages the Windows Connection Manager Profile Installer (CMSTP.exe) to elevate privileges.

## Overview

The implementation uses Windows' CMSTP.exe binary to bypass UAC restrictions through a specially crafted .inf file. This allows a medium integrity process belonging to a local administrator to spawn a new high integrity process with full privileges.

## Technical Implementation

### Key Components

#### INF File Generation
- Creates a temporary .inf file in `C:\windows\temp`
- Uses UUID v4 for unique filename generation
- Injects the specified command into the INF template
- Maintains the same INF structure as the original implementation

#### Process Management
- Executes CMSTP.exe with the `/au` flag
- Handles process spawning and monitoring
- Implements proper error handling for binary existence checks

#### Window Management
- Uses the Windows API through `winapi` crate
- Implements window finding and interaction
- Handles UAC prompt automation
- Simulates keyboard input when needed

### Safety Improvements

- Memory safety through Rust's ownership system
- No unsafe DLL reflection (unlike the C# version)
- Proper error handling and Result types
- Safe string handling with proper Unicode support
- Controlled process management

## Usage

Default usage (spawns elevated cmd.exe):
```bash
.\RustyKeys.exe
```

Execute specific command with elevation:
```bash
.\RustyKeys.exe "path_to_executable"
```

## Building on Windows

```cmd
cargo build --release
```

## Building on Linux
```cmd
cargo build --release --target x86_64-pc-windows-msvc  # For 64-bit
cargo build --release --target i686-pc-windows-msvc    # For 32-bit
```

## Technical Details

### Core Functions

#### generate_inf_file(command: &str) -> String
- Generates the INF file with the command to be executed
- Returns the path to the generated file
- Uses UUID for unique filename generation

#### execute_cmstp(inf_file: &str)
- Handles the execution of CMSTP.exe
- Manages process creation and monitoring
- Implements privilege elevation logic

#### interact_with_window(process_name: &str) -> bool
- Manages window interaction
- Handles UAC prompt automation
- Implements keyboard simulation when needed

## Credits

This implementation is based on research and work by several security researchers:

- Original Research: [Oddvar Moe](https://oddvar.moe/2017/08/15/research-on-cmstp-exe/)
- Original C#/.NET Implementation and Article: zc00l
  - Article: [How to bypass UAC in newer Windows versions](https://0x00-0x00.github.io/research/2018/10/31/How-to-bypass-UAC-in-newer-Windows-versions.html)
- PowerShell Script: Tyler Applebaum
  - Script: [UACBypassCMSTP.ps1](https://gist.githubusercontent.com/tylerapplebaum/ae8cb38ed8314518d95b2e32a6f0d3f1/raw/3127ba7453a6f6d294cd422386cae1a5a2791d71/UACBypassCMSTP.ps1)

Special thanks to zc00l for the comprehensive article explaining the technique and providing the original C#/.NET implementation that served as the basis for this Rust version.

## Legal Disclaimer

This code is provided for educational purposes only. Users are responsible for ensuring compliance with applicable laws and regulations. The authors are not responsible for misuse of this software.
