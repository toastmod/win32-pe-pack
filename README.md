# Win32 PE Packer

This program hunts and gathers the Dynamically Linked Libraries from your System32 folder, as per a given PE file's imports, and packs them into a folder.

You can use [pe-bear](https://github.com/hasherezade/pe-bear) to find what imports are available.

# Usage

`win32-pe-pack [FILENAME]`

or just `cargo run [FILENAME]`

For example `win32-pe-packer main.exe` will create a folder named `main-packed` with a copy of `main.exe` and it's DLLs in it.

# Uh... why?

I'm using this tool to investigate compatibility between win32kmin (Xbox One/Series OS, "OneCore", Windows 10X/Mobile/IoT) and win32kfull (Windows 10/11 desktop) based Windows systems.

Having this tool allows me to fetch DLLs from my desktop OS that are missing on win32kmin systems. It's proven helpful to get some exe's running. But without win32k.sys loading win32kfull.sys, there are still alot of missing system calls. One most obvious example being for the Windows desktop shell.

This is kind of inspired by the cancelled project to bring desktop win32 apps to Windows 10X in the form on a virtualized container.
