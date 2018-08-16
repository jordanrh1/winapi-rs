// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Public header file for Windows NT Setup and Device Installer services Dll.
use ctypes::c_int;
use shared::basetsd::{DWORD_PTR, UINT_PTR, ULONG_PTR};
use shared::devpropdef::{DEVPROPKEY, DEVPROPTYPE};
use shared::guiddef::{GUID, LPGUID};
use shared::minwindef::{
    BOOL, BYTE, DWORD, FILETIME, HINSTANCE, HKEY, INT, LPARAM, LPCVOID, LPDWORD, MAX_PATH, PBOOL,
    PBYTE, PDWORD, PINT, PUINT, UINT, USHORT, WORD,
};
use shared::windef::{HDC, HICON, HWND, RECT};
use um::commctrl::HIMAGELIST;
use um::prsht::{HPROPSHEETPAGE, LPPROPSHEETHEADERA, LPPROPSHEETHEADERW};
use um::spapidef::SP_LOG_TOKEN;
use um::winnt::{
    ANYSIZE_ARRAY, APPLICATION_ERROR_MASK, CHAR, DWORDLONG, ERROR_SEVERITY_ERROR, HANDLE, LONG,
    LONGLONG, LPCSTR, LPCWSTR, PCSTR, PCWSTR, PSTR, PVOID, PWSTR, WCHAR,
};
use um::winreg::REGSAM;
pub const LINE_LEN: usize = 256;
pub const MAX_INF_STRING_LENGTH: usize = 4096;
pub const MAX_INF_SECTION_NAME_LENGTH: usize = 255;
pub const MAX_TITLE_LEN: usize = 60;
pub const MAX_INSTRUCTION_LEN: usize = 256;
pub const MAX_LABEL_LEN: usize = 30;
pub const MAX_SERVICE_NAME_LEN: usize = 256;
pub const MAX_SUBTITLE_LEN: usize = 256;
pub const SP_MAX_MACHINENAME_LENGTH: usize = MAX_PATH + 3;
pub type HINF = PVOID;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct INFCONTEXT {
    Inf: PVOID,
    CurrentInf: PVOID,
    Section: UINT,
    Line: UINT,
}}
pub type PINFCONTEXT = *mut INFCONTEXT;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_INF_INFORMATION {
    InfStyle: DWORD,
    InfCount: DWORD,
    VersionData: [BYTE; ANYSIZE_ARRAY],
}}
pub type PSP_INF_INFORMATION = *mut SP_INF_INFORMATION;
UNION!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] union SP_ALTPLATFORM_INFO_V3_u {
    [u16; 1],
    Reserved Reserved_mut: WORD,
    Flags Flags_mut: WORD,
}}
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_ALTPLATFORM_INFO_V3 {
    cbSize: DWORD,
    Platform: DWORD,
    MajorVersion: DWORD,
    MinorVersion: DWORD,
    ProcessorArchitecture: WORD,
    u: SP_ALTPLATFORM_INFO_V3_u,
    FirstValidatedMajorVersion: DWORD,
    FirstValidatedMinorVersion: DWORD,
    ProductType: BYTE,
    SuiteMask: WORD,
    BuildNumber: DWORD,
}}
pub type PSP_ALTPLATFORM_INFO_V3 = *mut SP_ALTPLATFORM_INFO_V3;
UNION!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] union SP_ALTPLATFORM_INFO_V2_u {
    [u16; 1],
    Reserved Reserved_mut: WORD,
    Flags Flags_mut: WORD,
}}
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_ALTPLATFORM_INFO_V2 {
    cbSize: DWORD,
    Platform: DWORD,
    MajorVersion: DWORD,
    MinorVersion: DWORD,
    ProcessorArchitecture: WORD,
    u: SP_ALTPLATFORM_INFO_V2_u,
    FirstValidatedMajorVersion: DWORD,
    FirstValidatedMinorVersion: DWORD,
}}
pub type PSP_ALTPLATFORM_INFO_V2 = *mut SP_ALTPLATFORM_INFO_V2;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_ALTPLATFORM_INFO_V1 {
    cbSize: DWORD,
    Platform: DWORD,
    MajorVersion: DWORD,
    MinorVersion: DWORD,
    ProcessorArchitecture: WORD,
    Reserved: WORD,
}}
pub type PSP_ALTPLATFORM_INFO_V1 = *mut SP_ALTPLATFORM_INFO_V1;
pub type SP_ALTPLATFORM_INFO = SP_ALTPLATFORM_INFO_V2;
pub type PSP_ALTPLATFORM_INFO = PSP_ALTPLATFORM_INFO_V2;
pub const SP_ALTPLATFORM_FLAGS_VERSION_RANGE: WORD = 0x0001;
pub const SP_ALTPLATFORM_FLAGS_SUITE_MASK: WORD = 0x0002;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_ORIGINAL_FILE_INFO_A {
    cbSize: DWORD,
    OriginalInfName: [CHAR; MAX_PATH],
    OriginalCatalogName: [CHAR; MAX_PATH],
}}
pub type PSP_ORIGINAL_FILE_INFO_A = *mut SP_ORIGINAL_FILE_INFO_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_ORIGINAL_FILE_INFO_W {
    cbSize: DWORD,
    OriginalInfName: [WCHAR; MAX_PATH],
    OriginalCatalogName: [WCHAR; MAX_PATH],
}}
pub type PSP_ORIGINAL_FILE_INFO_W = *mut SP_ORIGINAL_FILE_INFO_W;
pub const INF_STYLE_NONE: DWORD = 0x00000000;
pub const INF_STYLE_OLDNT: DWORD = 0x00000001;
pub const INF_STYLE_WIN4: DWORD = 0x00000002;
pub const INF_STYLE_CACHE_ENABLE: DWORD = 0x00000010;
pub const INF_STYLE_CACHE_DISABLE: DWORD = 0x00000020;
pub const INF_STYLE_CACHE_IGNORE: DWORD = 0x00000040;
pub const DIRID_ABSOLUTE: DWORD = -1i32 as u32;
pub const DIRID_ABSOLUTE_16BIT: DWORD = 0xffff;
pub const DIRID_NULL: DWORD = 0;
pub const DIRID_SRCPATH: DWORD = 1;
pub const DIRID_WINDOWS: DWORD = 10;
pub const DIRID_SYSTEM: DWORD = 11;
pub const DIRID_DRIVERS: DWORD = 12;
pub const DIRID_IOSUBSYS: DWORD = DIRID_DRIVERS;
pub const DIRID_DRIVER_STORE: DWORD = 13;
pub const DIRID_INF: DWORD = 17;
pub const DIRID_HELP: DWORD = 18;
pub const DIRID_FONTS: DWORD = 20;
pub const DIRID_VIEWERS: DWORD = 21;
pub const DIRID_COLOR: DWORD = 23;
pub const DIRID_APPS: DWORD = 24;
pub const DIRID_SHARED: DWORD = 25;
pub const DIRID_BOOT: DWORD = 30;
pub const DIRID_SYSTEM16: DWORD = 50;
pub const DIRID_SPOOL: DWORD = 51;
pub const DIRID_SPOOLDRIVERS: DWORD = 52;
pub const DIRID_USERPROFILE: DWORD = 53;
pub const DIRID_LOADER: DWORD = 54;
pub const DIRID_PRINTPROCESSOR: DWORD = 55;
pub const DIRID_DEFAULT: DWORD = DIRID_SYSTEM;
pub const DIRID_COMMON_STARTMENU: DWORD = 16406;
pub const DIRID_COMMON_PROGRAMS: DWORD = 16407;
pub const DIRID_COMMON_STARTUP: DWORD = 16408;
pub const DIRID_COMMON_DESKTOPDIRECTORY: DWORD = 16409;
pub const DIRID_COMMON_FAVORITES: DWORD = 16415;
pub const DIRID_COMMON_APPDATA: DWORD = 16419;
pub const DIRID_PROGRAM_FILES: DWORD = 16422;
pub const DIRID_SYSTEM_X86: DWORD = 16425;
pub const DIRID_PROGRAM_FILES_X86: DWORD = 16426;
pub const DIRID_PROGRAM_FILES_COMMON: DWORD = 16427;
pub const DIRID_PROGRAM_FILES_COMMONX86: DWORD = 16428;
pub const DIRID_COMMON_TEMPLATES: DWORD = 16429;
pub const DIRID_COMMON_DOCUMENTS: DWORD = 16430;
pub const DIRID_USER: DWORD = 0x8000;
FN!{stdcall PSP_FILE_CALLBACK_A(
    Context: PVOID,
    Notification: UINT,
    Param1: UINT_PTR,
    Param2: UINT_PTR,
) -> UINT}
FN!{stdcall PSP_FILE_CALLBACK_W(
    Context: PVOID,
    Notification: UINT,
    Param1: UINT_PTR,
    Param2: UINT_PTR,
) -> UINT}
pub const SPFILENOTIFY_STARTQUEUE: UINT = 0x00000001;
pub const SPFILENOTIFY_ENDQUEUE: UINT = 0x00000002;
pub const SPFILENOTIFY_STARTSUBQUEUE: UINT = 0x00000003;
pub const SPFILENOTIFY_ENDSUBQUEUE: UINT = 0x00000004;
pub const SPFILENOTIFY_STARTDELETE: UINT = 0x00000005;
pub const SPFILENOTIFY_ENDDELETE: UINT = 0x00000006;
pub const SPFILENOTIFY_DELETEERROR: UINT = 0x00000007;
pub const SPFILENOTIFY_STARTRENAME: UINT = 0x00000008;
pub const SPFILENOTIFY_ENDRENAME: UINT = 0x00000009;
pub const SPFILENOTIFY_RENAMEERROR: UINT = 0x0000000a;
pub const SPFILENOTIFY_STARTCOPY: UINT = 0x0000000b;
pub const SPFILENOTIFY_ENDCOPY: UINT = 0x0000000c;
pub const SPFILENOTIFY_COPYERROR: UINT = 0x0000000d;
pub const SPFILENOTIFY_NEEDMEDIA: UINT = 0x0000000e;
pub const SPFILENOTIFY_QUEUESCAN: UINT = 0x0000000f;
pub const SPFILENOTIFY_CABINETINFO: UINT = 0x00000010;
pub const SPFILENOTIFY_FILEINCABINET: UINT = 0x00000011;
pub const SPFILENOTIFY_NEEDNEWCABINET: UINT = 0x00000012;
pub const SPFILENOTIFY_FILEEXTRACTED: UINT = 0x00000013;
pub const SPFILENOTIFY_FILEOPDELAYED: UINT = 0x00000014;
pub const SPFILENOTIFY_STARTBACKUP: UINT = 0x00000015;
pub const SPFILENOTIFY_BACKUPERROR: UINT = 0x00000016;
pub const SPFILENOTIFY_ENDBACKUP: UINT = 0x00000017;
pub const SPFILENOTIFY_QUEUESCAN_EX: UINT = 0x00000018;
pub const SPFILENOTIFY_STARTREGISTRATION: UINT = 0x00000019;
pub const SPFILENOTIFY_ENDREGISTRATION: UINT = 0x00000020;
pub const SPFILENOTIFY_QUEUESCAN_SIGNERINFO: UINT = 0x00000040;
pub const SPFILENOTIFY_LANGMISMATCH: UINT = 0x00010000;
pub const SPFILENOTIFY_TARGETEXISTS: UINT = 0x00020000;
pub const SPFILENOTIFY_TARGETNEWER: UINT = 0x00040000;
pub const FILEOP_COPY: UINT = 0;
pub const FILEOP_RENAME: UINT = 1;
pub const FILEOP_DELETE: UINT = 2;
pub const FILEOP_BACKUP: UINT = 3;
pub const FILEOP_ABORT: UINT = 0;
pub const FILEOP_DOIT: UINT = 1;
pub const FILEOP_SKIP: UINT = 2;
pub const FILEOP_RETRY: UINT = FILEOP_DOIT;
pub const FILEOP_NEWPATH: UINT = 4;
pub const COPYFLG_WARN_IF_SKIP: UINT = 0x00000001;
pub const COPYFLG_NOSKIP: UINT = 0x00000002;
pub const COPYFLG_NOVERSIONCHECK: UINT = 0x00000004;
pub const COPYFLG_FORCE_FILE_IN_USE: UINT = 0x00000008;
pub const COPYFLG_NO_OVERWRITE: UINT = 0x00000010;
pub const COPYFLG_NO_VERSION_DIALOG: UINT = 0x00000020;
pub const COPYFLG_OVERWRITE_OLDER_ONLY: UINT = 0x00000040;
pub const COPYFLG_PROTECTED_WINDOWS_DRIVER_FILE: UINT = 0x00000100;
pub const COPYFLG_REPLACEONLY: UINT = 0x00000400;
pub const COPYFLG_NODECOMP: UINT = 0x00000800;
pub const COPYFLG_REPLACE_BOOT_FILE: UINT = 0x00001000;
pub const COPYFLG_NOPRUNE: UINT = 0x00002000;
pub const COPYFLG_IN_USE_TRY_RENAME: UINT = 0x00004000;
pub const DELFLG_IN_USE: UINT = 0x00000001;
pub const DELFLG_IN_USE1: UINT = 0x00010000;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct FILEPATHS_A {
    Target: PCSTR,
    Source: PCSTR,
    Win32Error: UINT,
    Flags: DWORD,
}}
pub type PFILEPATHS_A = *mut FILEPATHS_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct FILEPATHS_W {
    Target: PCWSTR,
    Source: PCWSTR,
    Win32Error: UINT,
    Flags: DWORD,
}}
pub type PFILEPATHS_W = *mut FILEPATHS_W;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct FILEPATHS_SIGNERINFO_A {
    Target: PCSTR,
    Source: PCSTR,
    Win32Error: UINT,
    Flags: DWORD,
    DigitalSigner: PCSTR,
    Version: PCSTR,
    CatalogFile: PCSTR,
}}
pub type PFILEPATHS_SIGNERINFO_A = *mut FILEPATHS_SIGNERINFO_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct FILEPATHS_SIGNERINFO_W {
    Target: PCWSTR,
    Source: PCWSTR,
    Win32Error: UINT,
    Flags: DWORD,
    DigitalSigner: PCWSTR,
    Version: PCWSTR,
    CatalogFile: PCWSTR,
}}
pub type PFILEPATHS_SIGNERINFO_W = *mut FILEPATHS_SIGNERINFO_W;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SOURCE_MEDIA_A {
    Reserved: PCSTR,
    Tagfile: PCSTR,
    Description: PCSTR,
    SourcePath: PCSTR,
    SourceFile: PCSTR,
    Flags: DWORD,
}}
pub type PSOURCE_MEDIA_A = *mut SOURCE_MEDIA_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SOURCE_MEDIA_W {
    Reserved: PCWSTR,
    Tagfile: PCWSTR,
    Description: PCWSTR,
    SourcePath: PCWSTR,
    SourceFile: PCWSTR,
    Flags: DWORD,
}}
pub type PSOURCE_MEDIA_W = *mut SOURCE_MEDIA_W;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct CABINET_INFO_A {
    CabinetPath: PCSTR,
    CabinetFile: PCSTR,
    DiskName: PCSTR,
    SetId: USHORT,
    CabinetNumber: USHORT,
}}
pub type PCABINET_INFO_A = *mut CABINET_INFO_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct CABINET_INFO_W {
    CabinetPath: PCWSTR,
    CabinetFile: PCWSTR,
    DiskName: PCWSTR,
    SetId: USHORT,
    CabinetNumber: USHORT,
}}
pub type PCABINET_INFO_W = *mut CABINET_INFO_W;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct FILE_IN_CABINET_INFO_A {
    NameInCabinet: PCSTR,
    FileSize: DWORD,
    Win32Error: DWORD,
    DosDate: WORD,
    DosTime: WORD,
    DosAttribs: WORD,
    FullTargetName: [CHAR; MAX_PATH],
}}
pub type PFILE_IN_CABINET_INFO_A = *mut FILE_IN_CABINET_INFO_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct FILE_IN_CABINET_INFO_W {
    NameInCabinet: PCWSTR,
    FileSize: DWORD,
    Win32Error: DWORD,
    DosDate: WORD,
    DosTime: WORD,
    DosAttribs: WORD,
    FullTargetName: [WCHAR; MAX_PATH],
}}
pub type PFILE_IN_CABINET_INFO_W = *mut FILE_IN_CABINET_INFO_W;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_REGISTER_CONTROL_STATUSA {
    cbSize: DWORD,
    FileName: PCSTR,
    Win32Error: DWORD,
    FailureCode: DWORD,
}}
pub type PSP_REGISTER_CONTROL_STATUSA = *mut SP_REGISTER_CONTROL_STATUSA;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_REGISTER_CONTROL_STATUSW {
    cbSize: DWORD,
    FileName: PCWSTR,
    Win32Error: DWORD,
    FailureCode: DWORD,
}}
pub type PSP_REGISTER_CONTROL_STATUSW = *mut SP_REGISTER_CONTROL_STATUSW;
pub const SPREG_SUCCESS: DWORD = 0x00000000;
pub const SPREG_LOADLIBRARY: DWORD = 0x00000001;
pub const SPREG_GETPROCADDR: DWORD = 0x00000002;
pub const SPREG_REGSVR: DWORD = 0x00000003;
pub const SPREG_DLLINSTALL: DWORD = 0x00000004;
pub const SPREG_TIMEOUT: DWORD = 0x00000005;
pub const SPREG_UNKNOWN: DWORD = 0xFFFFFFFF;
pub type HSPFILEQ = PVOID;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_FILE_COPY_PARAMS_A {
    cbSize: DWORD,
    QueueHandle: HSPFILEQ,
    SourceRootPath: PCSTR,
    SourcePath: PCSTR,
    SourceFilename: PCSTR,
    SourceDescription: PCSTR,
    SourceTagfile: PCSTR,
    TargetDirectory: PCSTR,
    TargetFilename: PCSTR,
    CopyStyle: DWORD,
    LayoutInf: HINF,
    SecurityDescriptor: PCSTR,
}}
pub type PSP_FILE_COPY_PARAMS_A = *mut SP_FILE_COPY_PARAMS_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_FILE_COPY_PARAMS_W {
    cbSize: DWORD,
    QueueHandle: HSPFILEQ,
    SourceRootPath: PCWSTR,
    SourcePath: PCWSTR,
    SourceFilename: PCWSTR,
    SourceDescription: PCWSTR,
    SourceTagfile: PCWSTR,
    TargetDirectory: PCWSTR,
    TargetFilename: PCWSTR,
    CopyStyle: DWORD,
    LayoutInf: HINF,
    SecurityDescriptor: PCWSTR,
}}
pub type PSP_FILE_COPY_PARAMS_W = *mut SP_FILE_COPY_PARAMS_W;
pub type HDSKSPC = PVOID;
pub type HDEVINFO = PVOID;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_DEVINFO_DATA {
    cbSize: DWORD,
    ClassGuid: GUID,
    DevInst: DWORD,
    Reserved: ULONG_PTR,
}}
pub type PSP_DEVINFO_DATA = *mut SP_DEVINFO_DATA;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_DEVICE_INTERFACE_DATA {
    cbSize: DWORD,
    InterfaceClassGuid: GUID,
    Flags: DWORD,
    Reserved: ULONG_PTR,
}}
pub type PSP_DEVICE_INTERFACE_DATA = *mut SP_DEVICE_INTERFACE_DATA;
pub const SPINT_ACTIVE: DWORD = 0x00000001;
pub const SPINT_DEFAULT: DWORD = 0x00000002;
pub const SPINT_REMOVED: DWORD = 0x00000004;
pub type SP_INTERFACE_DEVICE_DATA = SP_DEVICE_INTERFACE_DATA;
pub type PSP_INTERFACE_DEVICE_DATA = PSP_DEVICE_INTERFACE_DATA;
pub const SPID_ACTIVE: DWORD = SPINT_ACTIVE;
pub const SPID_DEFAULT: DWORD = SPINT_DEFAULT;
pub const SPID_REMOVED: DWORD = SPINT_REMOVED;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    cbSize: DWORD,
    DevicePath: [CHAR; ANYSIZE_ARRAY],
}}
pub type PSP_DEVICE_INTERFACE_DETAIL_DATA_A = *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    cbSize: DWORD,
    DevicePath: [WCHAR; ANYSIZE_ARRAY],
}}
pub type PSP_DEVICE_INTERFACE_DETAIL_DATA_W = *mut SP_DEVICE_INTERFACE_DETAIL_DATA_W;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_DEVINFO_LIST_DETAIL_DATA_A {
    cbSize: DWORD,
    ClassGuid: GUID,
    RemoteMachineHandle: HANDLE,
    RemoteMachineName: [CHAR; SP_MAX_MACHINENAME_LENGTH],
}}
pub type PSP_DEVINFO_LIST_DETAIL_DATA_A = *mut SP_DEVINFO_LIST_DETAIL_DATA_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_DEVINFO_LIST_DETAIL_DATA_W {
    cbSize: DWORD,
    ClassGuid: GUID,
    RemoteMachineHandle: HANDLE,
    RemoteMachineName: [WCHAR; SP_MAX_MACHINENAME_LENGTH],
}}
pub type PSP_DEVINFO_LIST_DETAIL_DATA_W = *mut SP_DEVINFO_LIST_DETAIL_DATA_W;
pub const DIF_SELECTDEVICE: DI_FUNCTION = 0x00000001;
pub const DIF_INSTALLDEVICE: DI_FUNCTION = 0x00000002;
pub const DIF_ASSIGNRESOURCES: DI_FUNCTION = 0x00000003;
pub const DIF_PROPERTIES: DI_FUNCTION = 0x00000004;
pub const DIF_REMOVE: DI_FUNCTION = 0x00000005;
pub const DIF_FIRSTTIMESETUP: DI_FUNCTION = 0x00000006;
pub const DIF_FOUNDDEVICE: DI_FUNCTION = 0x00000007;
pub const DIF_SELECTCLASSDRIVERS: DI_FUNCTION = 0x00000008;
pub const DIF_VALIDATECLASSDRIVERS: DI_FUNCTION = 0x00000009;
pub const DIF_INSTALLCLASSDRIVERS: DI_FUNCTION = 0x0000000A;
pub const DIF_CALCDISKSPACE: DI_FUNCTION = 0x0000000B;
pub const DIF_DESTROYPRIVATEDATA: DI_FUNCTION = 0x0000000C;
pub const DIF_VALIDATEDRIVER: DI_FUNCTION = 0x0000000D;
pub const DIF_DETECT: DI_FUNCTION = 0x0000000F;
pub const DIF_INSTALLWIZARD: DI_FUNCTION = 0x00000010;
pub const DIF_DESTROYWIZARDDATA: DI_FUNCTION = 0x00000011;
pub const DIF_PROPERTYCHANGE: DI_FUNCTION = 0x00000012;
pub const DIF_ENABLECLASS: DI_FUNCTION = 0x00000013;
pub const DIF_DETECTVERIFY: DI_FUNCTION = 0x00000014;
pub const DIF_INSTALLDEVICEFILES: DI_FUNCTION = 0x00000015;
pub const DIF_UNREMOVE: DI_FUNCTION = 0x00000016;
pub const DIF_SELECTBESTCOMPATDRV: DI_FUNCTION = 0x00000017;
pub const DIF_ALLOW_INSTALL: DI_FUNCTION = 0x00000018;
pub const DIF_REGISTERDEVICE: DI_FUNCTION = 0x00000019;
pub const DIF_NEWDEVICEWIZARD_PRESELECT: DI_FUNCTION = 0x0000001A;
pub const DIF_NEWDEVICEWIZARD_SELECT: DI_FUNCTION = 0x0000001B;
pub const DIF_NEWDEVICEWIZARD_PREANALYZE: DI_FUNCTION = 0x0000001C;
pub const DIF_NEWDEVICEWIZARD_POSTANALYZE: DI_FUNCTION = 0x0000001D;
pub const DIF_NEWDEVICEWIZARD_FINISHINSTALL: DI_FUNCTION = 0x0000001E;
pub const DIF_UNUSED1: DI_FUNCTION = 0x0000001F;
pub const DIF_INSTALLINTERFACES: DI_FUNCTION = 0x00000020;
pub const DIF_DETECTCANCEL: DI_FUNCTION = 0x00000021;
pub const DIF_REGISTER_COINSTALLERS: DI_FUNCTION = 0x00000022;
pub const DIF_ADDPROPERTYPAGE_ADVANCED: DI_FUNCTION = 0x00000023;
pub const DIF_ADDPROPERTYPAGE_BASIC: DI_FUNCTION = 0x00000024;
pub const DIF_RESERVED1: DI_FUNCTION = 0x00000025;
pub const DIF_TROUBLESHOOTER: DI_FUNCTION = 0x00000026;
pub const DIF_POWERMESSAGEWAKE: DI_FUNCTION = 0x00000027;
pub const DIF_ADDREMOTEPROPERTYPAGE_ADVANCED: DI_FUNCTION = 0x00000028;
pub const DIF_UPDATEDRIVER_UI: DI_FUNCTION = 0x00000029;
pub const DIF_FINISHINSTALL_ACTION: DI_FUNCTION = 0x0000002A;
pub const DIF_RESERVED2: DI_FUNCTION = 0x00000030;
pub const DIF_MOVEDEVICE: DI_FUNCTION = 0x0000000E;
pub type DI_FUNCTION = UINT;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_DEVINSTALL_PARAMS_A {
    cbSize: DWORD,
    Flags: DWORD,
    FlagsEx: DWORD,
    hwndParent: HWND,
    InstallMsgHandler: PSP_FILE_CALLBACK_A,
    InstallMsgHandlerContext: PVOID,
    FileQueue: HSPFILEQ,
    ClassInstallReserved: ULONG_PTR,
    Reserved: DWORD,
    DriverPath: [CHAR; MAX_PATH],
}}
pub type PSP_DEVINSTALL_PARAMS_A = *mut SP_DEVINSTALL_PARAMS_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_DEVINSTALL_PARAMS_W {
    cbSize: DWORD,
    Flags: DWORD,
    FlagsEx: DWORD,
    hwndParent: HWND,
    InstallMsgHandler: PSP_FILE_CALLBACK_W,
    InstallMsgHandlerContext: PVOID,
    FileQueue: HSPFILEQ,
    ClassInstallReserved: ULONG_PTR,
    Reserved: DWORD,
    DriverPath: [WCHAR; MAX_PATH],
}}
pub type PSP_DEVINSTALL_PARAMS_W = *mut SP_DEVINSTALL_PARAMS_W;
pub const DI_SHOWOEM: DWORD = 0x00000001;
pub const DI_SHOWCOMPAT: DWORD = 0x00000002;
pub const DI_SHOWCLASS: DWORD = 0x00000004;
pub const DI_SHOWALL: DWORD = 0x00000007;
pub const DI_NOVCP: DWORD = 0x00000008;
pub const DI_DIDCOMPAT: DWORD = 0x00000010;
pub const DI_DIDCLASS: DWORD = 0x00000020;
pub const DI_AUTOASSIGNRES: DWORD = 0x00000040;
pub const DI_NEEDRESTART: DWORD = 0x00000080;
pub const DI_NEEDREBOOT: DWORD = 0x00000100;
pub const DI_NOBROWSE: DWORD = 0x00000200;
pub const DI_MULTMFGS: DWORD = 0x00000400;
pub const DI_DISABLED: DWORD = 0x00000800;
pub const DI_GENERALPAGE_ADDED: DWORD = 0x00001000;
pub const DI_RESOURCEPAGE_ADDED: DWORD = 0x00002000;
pub const DI_PROPERTIES_CHANGE: DWORD = 0x00004000;
pub const DI_INF_IS_SORTED: DWORD = 0x00008000;
pub const DI_ENUMSINGLEINF: DWORD = 0x00010000;
pub const DI_DONOTCALLCONFIGMG: DWORD = 0x00020000;
pub const DI_INSTALLDISABLED: DWORD = 0x00040000;
pub const DI_COMPAT_FROM_CLASS: DWORD = 0x00080000;
pub const DI_CLASSINSTALLPARAMS: DWORD = 0x00100000;
pub const DI_NODI_DEFAULTACTION: DWORD = 0x00200000;
pub const DI_QUIETINSTALL: DWORD = 0x00800000;
pub const DI_NOFILECOPY: DWORD = 0x01000000;
pub const DI_FORCECOPY: DWORD = 0x02000000;
pub const DI_DRIVERPAGE_ADDED: DWORD = 0x04000000;
pub const DI_USECI_SELECTSTRINGS: DWORD = 0x08000000;
pub const DI_OVERRIDE_INFFLAGS: DWORD = 0x10000000;
pub const DI_PROPS_NOCHANGEUSAGE: DWORD = 0x20000000;
pub const DI_NOSELECTICONS: DWORD = 0x40000000;
pub const DI_NOWRITE_IDS: DWORD = 0x80000000;
pub const DI_FLAGSEX_RESERVED2: DWORD = 0x00000001;
pub const DI_FLAGSEX_RESERVED3: DWORD = 0x00000002;
pub const DI_FLAGSEX_CI_FAILED: DWORD = 0x00000004;
pub const DI_FLAGSEX_FINISHINSTALL_ACTION: DWORD = 0x00000008;
pub const DI_FLAGSEX_DIDINFOLIST: DWORD = 0x00000010;
pub const DI_FLAGSEX_DIDCOMPATINFO: DWORD = 0x00000020;
pub const DI_FLAGSEX_FILTERCLASSES: DWORD = 0x00000040;
pub const DI_FLAGSEX_SETFAILEDINSTALL: DWORD = 0x00000080;
pub const DI_FLAGSEX_DEVICECHANGE: DWORD = 0x00000100;
pub const DI_FLAGSEX_ALWAYSWRITEIDS: DWORD = 0x00000200;
pub const DI_FLAGSEX_PROPCHANGE_PENDING: DWORD = 0x00000400;
pub const DI_FLAGSEX_ALLOWEXCLUDEDDRVS: DWORD = 0x00000800;
pub const DI_FLAGSEX_NOUIONQUERYREMOVE: DWORD = 0x00001000;
pub const DI_FLAGSEX_USECLASSFORCOMPAT: DWORD = 0x00002000;
pub const DI_FLAGSEX_RESERVED4: DWORD = 0x00004000;
pub const DI_FLAGSEX_NO_DRVREG_MODIFY: DWORD = 0x00008000;
pub const DI_FLAGSEX_IN_SYSTEM_SETUP: DWORD = 0x00010000;
pub const DI_FLAGSEX_INET_DRIVER: DWORD = 0x00020000;
pub const DI_FLAGSEX_APPENDDRIVERLIST: DWORD = 0x00040000;
pub const DI_FLAGSEX_PREINSTALLBACKUP: DWORD = 0x00080000;
pub const DI_FLAGSEX_BACKUPONREPLACE: DWORD = 0x00100000;
pub const DI_FLAGSEX_DRIVERLIST_FROM_URL: DWORD = 0x00200000;
pub const DI_FLAGSEX_RESERVED1: DWORD = 0x00400000;
pub const DI_FLAGSEX_EXCLUDE_OLD_INET_DRIVERS: DWORD = 0x00800000;
pub const DI_FLAGSEX_POWERPAGE_ADDED: DWORD = 0x01000000;
pub const DI_FLAGSEX_FILTERSIMILARDRIVERS: DWORD = 0x02000000;
pub const DI_FLAGSEX_INSTALLEDDRIVER: DWORD = 0x04000000;
pub const DI_FLAGSEX_NO_CLASSLIST_NODE_MERGE: DWORD = 0x08000000;
pub const DI_FLAGSEX_ALTPLATFORM_DRVSEARCH: DWORD = 0x10000000;
pub const DI_FLAGSEX_RESTART_DEVICE_ONLY: DWORD = 0x20000000;
pub const DI_FLAGSEX_RECURSIVESEARCH: DWORD = 0x40000000;
pub const DI_FLAGSEX_SEARCH_PUBLISHED_INFS: DWORD = 0x80000000;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_CLASSINSTALL_HEADER {
    cbSize: DWORD,
    InstallFunction: DI_FUNCTION,
}}
pub type PSP_CLASSINSTALL_HEADER = *mut SP_CLASSINSTALL_HEADER;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_ENABLECLASS_PARAMS {
    ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    ClassGuid: GUID,
    EnableMessage: DWORD,
}}
pub type PSP_ENABLECLASS_PARAMS = *mut SP_ENABLECLASS_PARAMS;
pub const ENABLECLASS_QUERY: DWORD = 0;
pub const ENABLECLASS_SUCCESS: DWORD = 1;
pub const ENABLECLASS_FAILURE: DWORD = 2;
pub const DICS_ENABLE: DWORD = 0x00000001;
pub const DICS_DISABLE: DWORD = 0x00000002;
pub const DICS_PROPCHANGE: DWORD = 0x00000003;
pub const DICS_START: DWORD = 0x00000004;
pub const DICS_STOP: DWORD = 0x00000005;
pub const DICS_FLAG_GLOBAL: DWORD = 0x00000001;
pub const DICS_FLAG_CONFIGSPECIFIC: DWORD = 0x00000002;
pub const DICS_FLAG_CONFIGGENERAL: DWORD = 0x00000004;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_PROPCHANGE_PARAMS {
    ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    StateChange: DWORD,
    Scope: DWORD,
    HwProfile: DWORD,
}}
pub type PSP_PROPCHANGE_PARAMS = *mut SP_PROPCHANGE_PARAMS;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_REMOVEDEVICE_PARAMS {
    ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    Scope: DWORD,
    HwProfile: DWORD,
}}
pub type PSP_REMOVEDEVICE_PARAMS = *mut SP_REMOVEDEVICE_PARAMS;
pub const DI_REMOVEDEVICE_GLOBAL: DWORD = 0x00000001;
pub const DI_REMOVEDEVICE_CONFIGSPECIFIC: DWORD = 0x00000002;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_UNREMOVEDEVICE_PARAMS {
    ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    Scope: DWORD,
    HwProfile: DWORD,
}}
pub type PSP_UNREMOVEDEVICE_PARAMS = *mut SP_UNREMOVEDEVICE_PARAMS;
pub const DI_UNREMOVEDEVICE_CONFIGSPECIFIC: DWORD = 0x00000002;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_SELECTDEVICE_PARAMS_A {
    ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    Title: [CHAR; MAX_TITLE_LEN],
    Instructions: [CHAR; MAX_INSTRUCTION_LEN],
    ListLabel: [CHAR; MAX_LABEL_LEN],
    SubTitle: [CHAR; MAX_SUBTITLE_LEN],
    Reserved: [BYTE; 2],
}}
pub type PSP_SELECTDEVICE_PARAMS_A = *mut SP_SELECTDEVICE_PARAMS_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_SELECTDEVICE_PARAMS_W {
    ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    Title: [WCHAR; MAX_TITLE_LEN],
    Instructions: [WCHAR; MAX_INSTRUCTION_LEN],
    ListLabel: [WCHAR; MAX_LABEL_LEN],
    SubTitle: [WCHAR; MAX_SUBTITLE_LEN],
}}
pub type PSP_SELECTDEVICE_PARAMS_W = *mut SP_SELECTDEVICE_PARAMS_W;
FN!{stdcall PDETECT_PROGRESS_NOTIFY(
    ProgressNotifyParam: PVOID,
    DetectComplete: DWORD,
) -> BOOL}
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_DETECTDEVICE_PARAMS {
    ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    DetectProgressNotify: PDETECT_PROGRESS_NOTIFY,
    ProgressNotifyParam: PVOID,
}}
pub type PSP_DETECTDEVICE_PARAMS = *mut SP_DETECTDEVICE_PARAMS;
pub const MAX_INSTALLWIZARD_DYNAPAGES: usize = 20;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_INSTALLWIZARD_DATA {
    ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    Flags: DWORD,
    DynamicPages: [HPROPSHEETPAGE; MAX_INSTALLWIZARD_DYNAPAGES],
    NumDynamicPages: DWORD,
    DynamicPageFlags: DWORD,
    PrivateFlags: DWORD,
    PrivateData: LPARAM,
    hwndWizardDlg: HWND,
}}
pub type PSP_INSTALLWIZARD_DATA = *mut SP_INSTALLWIZARD_DATA;
pub const NDW_INSTALLFLAG_DIDFACTDEFS: DWORD = 0x00000001;
pub const NDW_INSTALLFLAG_HARDWAREALLREADYIN: DWORD = 0x00000002;
pub const NDW_INSTALLFLAG_NEEDRESTART: DWORD = DI_NEEDRESTART;
pub const NDW_INSTALLFLAG_NEEDREBOOT: DWORD = DI_NEEDREBOOT;
pub const NDW_INSTALLFLAG_NEEDSHUTDOWN: DWORD = 0x00000200;
pub const NDW_INSTALLFLAG_EXPRESSINTRO: DWORD = 0x00000400;
pub const NDW_INSTALLFLAG_SKIPISDEVINSTALLED: DWORD = 0x00000800;
pub const NDW_INSTALLFLAG_NODETECTEDDEVS: DWORD = 0x00001000;
pub const NDW_INSTALLFLAG_INSTALLSPECIFIC: DWORD = 0x00002000;
pub const NDW_INSTALLFLAG_SKIPCLASSLIST: DWORD = 0x00004000;
pub const NDW_INSTALLFLAG_CI_PICKED_OEM: DWORD = 0x00008000;
pub const NDW_INSTALLFLAG_PCMCIAMODE: DWORD = 0x00010000;
pub const NDW_INSTALLFLAG_PCMCIADEVICE: DWORD = 0x00020000;
pub const NDW_INSTALLFLAG_USERCANCEL: DWORD = 0x00040000;
pub const NDW_INSTALLFLAG_KNOWNCLASS: DWORD = 0x00080000;
pub const DYNAWIZ_FLAG_PAGESADDED: DWORD = 0x00000001;
pub const DYNAWIZ_FLAG_ANALYZE_HANDLECONFLICT: DWORD = 0x00000008;
pub const DYNAWIZ_FLAG_INSTALLDET_NEXT: DWORD = 0x00000002;
pub const DYNAWIZ_FLAG_INSTALLDET_PREV: DWORD = 0x00000004;
pub const MIN_IDD_DYNAWIZ_RESOURCE_ID: c_int = 10000;
pub const MAX_IDD_DYNAWIZ_RESOURCE_ID: c_int = 11000;
pub const IDD_DYNAWIZ_FIRSTPAGE: c_int = 10000;
pub const IDD_DYNAWIZ_SELECT_PREVPAGE: c_int = 10001;
pub const IDD_DYNAWIZ_SELECT_NEXTPAGE: c_int = 10002;
pub const IDD_DYNAWIZ_ANALYZE_PREVPAGE: c_int = 10003;
pub const IDD_DYNAWIZ_ANALYZE_NEXTPAGE: c_int = 10004;
pub const IDD_DYNAWIZ_SELECTDEV_PAGE: c_int = 10009;
pub const IDD_DYNAWIZ_ANALYZEDEV_PAGE: c_int = 10010;
pub const IDD_DYNAWIZ_INSTALLDETECTEDDEVS_PAGE: c_int = 10011;
pub const IDD_DYNAWIZ_SELECTCLASS_PAGE: c_int = 10012;
pub const IDD_DYNAWIZ_INSTALLDETECTED_PREVPAGE: c_int = 10006;
pub const IDD_DYNAWIZ_INSTALLDETECTED_NEXTPAGE: c_int = 10007;
pub const IDD_DYNAWIZ_INSTALLDETECTED_NODEVS: c_int = 10008;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_NEWDEVICEWIZARD_DATA {
    ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    Flags: DWORD,
    DynamicPages: [HPROPSHEETPAGE; MAX_INSTALLWIZARD_DYNAPAGES],
    NumDynamicPages: DWORD,
    hwndWizardDlg: HWND,
}}
pub type PSP_NEWDEVICEWIZARD_DATA = *mut SP_NEWDEVICEWIZARD_DATA;
pub type SP_ADDPROPERTYPAGE_DATA = SP_NEWDEVICEWIZARD_DATA;
pub type PSP_ADDPROPERTYPAGE_DATA = PSP_NEWDEVICEWIZARD_DATA;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_TROUBLESHOOTER_PARAMS_A {
    ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    ChmFile: [CHAR; MAX_PATH],
    HtmlTroubleShooter: [CHAR; MAX_PATH],
}}
pub type PSP_TROUBLESHOOTER_PARAMS_A = *mut SP_TROUBLESHOOTER_PARAMS_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_TROUBLESHOOTER_PARAMS_W {
    ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    ChmFile: [WCHAR; MAX_PATH],
    HtmlTroubleShooter: [WCHAR; MAX_PATH],
}}
pub type PSP_TROUBLESHOOTER_PARAMS_W = *mut SP_TROUBLESHOOTER_PARAMS_W;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_POWERMESSAGEWAKE_PARAMS_A {
    ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    PowerMessageWake: [CHAR; LINE_LEN * 2],
}}
pub type PSP_POWERMESSAGEWAKE_PARAMS_A = *mut SP_POWERMESSAGEWAKE_PARAMS_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_POWERMESSAGEWAKE_PARAMS_W {
    ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    PowerMessageWake: [WCHAR; LINE_LEN * 2],
}}
pub type PSP_POWERMESSAGEWAKE_PARAMS_W = *mut SP_POWERMESSAGEWAKE_PARAMS_W;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_DRVINFO_DATA_V2_A {
    cbSize: DWORD,
    DriverType: DWORD,
    Reserved: ULONG_PTR,
    Description: [CHAR; LINE_LEN],
    MfgName: [CHAR; LINE_LEN],
    ProviderName: [CHAR; LINE_LEN],
    DriverDate: FILETIME,
    DriverVersion: DWORDLONG,
}}
pub type PSP_DRVINFO_DATA_V2_A = *mut SP_DRVINFO_DATA_V2_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_DRVINFO_DATA_V2_W {
    cbSize: DWORD,
    DriverType: DWORD,
    Reserved: ULONG_PTR,
    Description: [WCHAR; LINE_LEN],
    MfgName: [WCHAR; LINE_LEN],
    ProviderName: [WCHAR; LINE_LEN],
    DriverDate: FILETIME,
    DriverVersion: DWORDLONG,
}}
pub type PSP_DRVINFO_DATA_V2_W = *mut SP_DRVINFO_DATA_V2_W;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_DRVINFO_DATA_V1_A {
    cbSize: DWORD,
    DriverType: DWORD,
    Reserved: ULONG_PTR,
    Description: [CHAR; LINE_LEN],
    MfgName: [CHAR; LINE_LEN],
    ProviderName: [CHAR; LINE_LEN],
}}
pub type PSP_DRVINFO_DATA_V1_A = *mut SP_DRVINFO_DATA_V1_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_DRVINFO_DATA_V1_W {
    cbSize: DWORD,
    DriverType: DWORD,
    Reserved: ULONG_PTR,
    Description: [WCHAR; LINE_LEN],
    MfgName: [WCHAR; LINE_LEN],
    ProviderName: [WCHAR; LINE_LEN],
}}
pub type PSP_DRVINFO_DATA_V1_W = *mut SP_DRVINFO_DATA_V1_W;
pub type SP_DRVINFO_DATA_A = SP_DRVINFO_DATA_V2_A;
pub type PSP_DRVINFO_DATA_A = PSP_DRVINFO_DATA_V2_A;
pub type SP_DRVINFO_DATA_W = SP_DRVINFO_DATA_V2_W;
pub type PSP_DRVINFO_DATA_W = PSP_DRVINFO_DATA_V2_W;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_DRVINFO_DETAIL_DATA_A {
    cbSize: DWORD,
    InfDate: FILETIME,
    CompatIDsOffset: DWORD,
    CompatIDsLength: DWORD,
    Reserved: ULONG_PTR,
    SectionName: [CHAR; LINE_LEN],
    InfFileName: [CHAR; MAX_PATH],
    DrvDescription: [CHAR; LINE_LEN],
    HardwareID: [CHAR; ANYSIZE_ARRAY],
}}
pub type PSP_DRVINFO_DETAIL_DATA_A = *mut SP_DRVINFO_DETAIL_DATA_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_DRVINFO_DETAIL_DATA_W {
    cbSize: DWORD,
    InfDate: FILETIME,
    CompatIDsOffset: DWORD,
    CompatIDsLength: DWORD,
    Reserved: ULONG_PTR,
    SectionName: [WCHAR; LINE_LEN],
    InfFileName: [WCHAR; MAX_PATH],
    DrvDescription: [WCHAR; LINE_LEN],
    HardwareID: [WCHAR; ANYSIZE_ARRAY],
}}
pub type PSP_DRVINFO_DETAIL_DATA_W = *mut SP_DRVINFO_DETAIL_DATA_W;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_DRVINSTALL_PARAMS {
    cbSize: DWORD,
    Rank: DWORD,
    Flags: DWORD,
    PrivateData: DWORD_PTR,
    Reserved: DWORD,
}}
pub type PSP_DRVINSTALL_PARAMS = *mut SP_DRVINSTALL_PARAMS;
pub const DNF_DUPDESC: DWORD = 0x00000001;
pub const DNF_OLDDRIVER: DWORD = 0x00000002;
pub const DNF_EXCLUDEFROMLIST: DWORD = 0x00000004;
pub const DNF_NODRIVER: DWORD = 0x00000008;
pub const DNF_LEGACYINF: DWORD = 0x00000010;
pub const DNF_CLASS_DRIVER: DWORD = 0x00000020;
pub const DNF_COMPATIBLE_DRIVER: DWORD = 0x00000040;
pub const DNF_INET_DRIVER: DWORD = 0x00000080;
pub const DNF_UNUSED1: DWORD = 0x00000100;
pub const DNF_UNUSED2: DWORD = 0x00000200;
pub const DNF_OLD_INET_DRIVER: DWORD = 0x00000400;
pub const DNF_BAD_DRIVER: DWORD = 0x00000800;
pub const DNF_DUPPROVIDER: DWORD = 0x00001000;
pub const DNF_INF_IS_SIGNED: DWORD = 0x00002000;
pub const DNF_OEM_F6_INF: DWORD = 0x00004000;
pub const DNF_DUPDRIVERVER: DWORD = 0x00008000;
pub const DNF_BASIC_DRIVER: DWORD = 0x00010000;
pub const DNF_AUTHENTICODE_SIGNED: DWORD = 0x00020000;
pub const DNF_INSTALLEDDRIVER: DWORD = 0x00040000;
pub const DNF_ALWAYSEXCLUDEFROMLIST: DWORD = 0x00080000;
pub const DNF_INBOX_DRIVER: DWORD = 0x00100000;
pub const DNF_REQUESTADDITIONALSOFTWARE: DWORD = 0x00200000;
pub const DNF_UNUSED_22: DWORD = 0x00400000;
pub const DNF_UNUSED_23: DWORD = 0x00800000;
pub const DNF_UNUSED_24: DWORD = 0x01000000;
pub const DNF_UNUSED_25: DWORD = 0x02000000;
pub const DNF_UNUSED_26: DWORD = 0x04000000;
pub const DNF_UNUSED_27: DWORD = 0x08000000;
pub const DNF_UNUSED_28: DWORD = 0x10000000;
pub const DNF_UNUSED_29: DWORD = 0x20000000;
pub const DNF_UNUSED_30: DWORD = 0x40000000;
pub const DNF_UNUSED_31: DWORD = 0x80000000;
pub const DRIVER_HARDWAREID_RANK: DWORD = 0x00000FFF;
pub const DRIVER_HARDWAREID_MASK: DWORD = 0x80000FFF;
pub const DRIVER_UNTRUSTED_RANK: DWORD = 0x80000000;
pub const DRIVER_W9X_SUSPECT_RANK: DWORD = 0xC0000000;
FN!{stdcall PSP_DETSIG_CMPPROC(
    DeviceInfoSet: HDEVINFO,
    NewDeviceData: PSP_DEVINFO_DATA,
    ExistingDeviceData: PSP_DEVINFO_DATA,
    CompareContext: PVOID,
) -> DWORD}
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct COINSTALLER_CONTEXT_DATA {
    PostProcessing: BOOL,
    InstallResult: DWORD,
    PrivateData: PVOID,
}}
pub type PCOINSTALLER_CONTEXT_DATA = *mut COINSTALLER_CONTEXT_DATA;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_CLASSIMAGELIST_DATA {
    cbSize: DWORD,
    ImageList: HIMAGELIST,
    Reserved: ULONG_PTR,
}}
pub type PSP_CLASSIMAGELIST_DATA = *mut SP_CLASSIMAGELIST_DATA;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_PROPSHEETPAGE_REQUEST {
    cbSize: DWORD,
    PageRequested: DWORD,
    DeviceInfoSet: HDEVINFO,
    DeviceInfoData: PSP_DEVINFO_DATA,
}}
pub type PSP_PROPSHEETPAGE_REQUEST = *mut SP_PROPSHEETPAGE_REQUEST;
pub const SPPSR_SELECT_DEVICE_RESOURCES: DWORD = 1;
pub const SPPSR_ENUM_BASIC_DEVICE_PROPERTIES: DWORD = 2;
pub const SPPSR_ENUM_ADV_DEVICE_PROPERTIES: DWORD = 3;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_BACKUP_QUEUE_PARAMS_V2_A {
    cbSize: DWORD,
    FullInfPath: [CHAR; MAX_PATH],
    FilenameOffset: INT,
    ReinstallInstance: [CHAR; MAX_PATH],
}}
pub type PSP_BACKUP_QUEUE_PARAMS_V2_A = *mut SP_BACKUP_QUEUE_PARAMS_V2_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_BACKUP_QUEUE_PARAMS_V2_W {
    cbSize: DWORD,
    FullInfPath: [WCHAR; MAX_PATH],
    FilenameOffset: INT,
    ReinstallInstance: [WCHAR; MAX_PATH],
}}
pub type PSP_BACKUP_QUEUE_PARAMS_V2_W = *mut SP_BACKUP_QUEUE_PARAMS_V2_W;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_BACKUP_QUEUE_PARAMS_V1_A {
    cbSize: DWORD,
    FullInfPath: [CHAR; MAX_PATH],
    FilenameOffset: INT,
}}
pub type PSP_BACKUP_QUEUE_PARAMS_V1_A = *mut SP_BACKUP_QUEUE_PARAMS_V1_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_BACKUP_QUEUE_PARAMS_V1_W {
    cbSize: DWORD,
    FullInfPath: [WCHAR; MAX_PATH],
    FilenameOffset: INT,
}}
pub type PSP_BACKUP_QUEUE_PARAMS_V1_W = *mut SP_BACKUP_QUEUE_PARAMS_V1_W;
pub type SP_BACKUP_QUEUE_PARAMS_A = SP_BACKUP_QUEUE_PARAMS_V2_A;
pub type PSP_BACKUP_QUEUE_PARAMS_A = PSP_BACKUP_QUEUE_PARAMS_V2_A;
pub type SP_BACKUP_QUEUE_PARAMS_W = SP_BACKUP_QUEUE_PARAMS_V2_W;
pub type PSP_BACKUP_QUEUE_PARAMS_W = PSP_BACKUP_QUEUE_PARAMS_V2_W;
pub const ERROR_EXPECTED_SECTION_NAME: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0;
pub const ERROR_BAD_SECTION_NAME_LINE: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 1;
pub const ERROR_SECTION_NAME_TOO_LONG: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 2;
pub const ERROR_GENERAL_SYNTAX: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 3;
pub const ERROR_WRONG_INF_STYLE: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x100;
pub const ERROR_SECTION_NOT_FOUND: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x101;
pub const ERROR_LINE_NOT_FOUND: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x102;
pub const ERROR_NO_BACKUP: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x103;
pub const ERROR_NO_ASSOCIATED_CLASS: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x200;
pub const ERROR_CLASS_MISMATCH: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x201;
pub const ERROR_DUPLICATE_FOUND: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x202;
pub const ERROR_NO_DRIVER_SELECTED: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x203;
pub const ERROR_KEY_DOES_NOT_EXIST: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x204;
pub const ERROR_INVALID_DEVINST_NAME: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x205;
pub const ERROR_INVALID_CLASS: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x206;
pub const ERROR_DEVINST_ALREADY_EXISTS: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x207;
pub const ERROR_DEVINFO_NOT_REGISTERED: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x208;
pub const ERROR_INVALID_REG_PROPERTY: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x209;
pub const ERROR_NO_INF: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x20A;
pub const ERROR_NO_SUCH_DEVINST: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x20B;
pub const ERROR_CANT_LOAD_CLASS_ICON: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x20C;
pub const ERROR_INVALID_CLASS_INSTALLER: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x20D;
pub const ERROR_DI_DO_DEFAULT: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x20E;
pub const ERROR_DI_NOFILECOPY: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x20F;
pub const ERROR_INVALID_HWPROFILE: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x210;
pub const ERROR_NO_DEVICE_SELECTED: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x211;
pub const ERROR_DEVINFO_LIST_LOCKED: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x212;
pub const ERROR_DEVINFO_DATA_LOCKED: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x213;
pub const ERROR_DI_BAD_PATH: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x214;
pub const ERROR_NO_CLASSINSTALL_PARAMS: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x215;
pub const ERROR_FILEQUEUE_LOCKED: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x216;
pub const ERROR_BAD_SERVICE_INSTALLSECT: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x217;
pub const ERROR_NO_CLASS_DRIVER_LIST: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x218;
pub const ERROR_NO_ASSOCIATED_SERVICE: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x219;
pub const ERROR_NO_DEFAULT_DEVICE_INTERFACE: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x21A;
pub const ERROR_DEVICE_INTERFACE_ACTIVE: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x21B;
pub const ERROR_DEVICE_INTERFACE_REMOVED: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x21C;
pub const ERROR_BAD_INTERFACE_INSTALLSECT: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x21D;
pub const ERROR_NO_SUCH_INTERFACE_CLASS: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x21E;
pub const ERROR_INVALID_REFERENCE_STRING: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x21F;
pub const ERROR_INVALID_MACHINENAME: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x220;
pub const ERROR_REMOTE_COMM_FAILURE: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x221;
pub const ERROR_MACHINE_UNAVAILABLE: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x222;
pub const ERROR_NO_CONFIGMGR_SERVICES: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x223;
pub const ERROR_INVALID_PROPPAGE_PROVIDER: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x224;
pub const ERROR_NO_SUCH_DEVICE_INTERFACE: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x225;
pub const ERROR_DI_POSTPROCESSING_REQUIRED: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x226;
pub const ERROR_INVALID_COINSTALLER: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x227;
pub const ERROR_NO_COMPAT_DRIVERS: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x228;
pub const ERROR_NO_DEVICE_ICON: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x229;
pub const ERROR_INVALID_INF_LOGCONFIG: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x22A;
pub const ERROR_DI_DONT_INSTALL: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x22B;
pub const ERROR_INVALID_FILTER_DRIVER: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x22C;
pub const ERROR_NON_WINDOWS_NT_DRIVER: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x22D;
pub const ERROR_NON_WINDOWS_DRIVER: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x22E;
pub const ERROR_NO_CATALOG_FOR_OEM_INF: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x22F;
pub const ERROR_DEVINSTALL_QUEUE_NONNATIVE: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x230;
pub const ERROR_NOT_DISABLEABLE: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x231;
pub const ERROR_CANT_REMOVE_DEVINST: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x232;
pub const ERROR_INVALID_TARGET: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x233;
pub const ERROR_DRIVER_NONNATIVE: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x234;
pub const ERROR_IN_WOW64: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x235;
pub const ERROR_SET_SYSTEM_RESTORE_POINT: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x236;
pub const ERROR_SCE_DISABLED: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x238;
pub const ERROR_UNKNOWN_EXCEPTION: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x239;
pub const ERROR_PNP_REGISTRY_ERROR: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x23A;
pub const ERROR_REMOTE_REQUEST_UNSUPPORTED: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x23B;
pub const ERROR_NOT_AN_INSTALLED_OEM_INF: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x23C;
pub const ERROR_INF_IN_USE_BY_DEVICES: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x23D;
pub const ERROR_DI_FUNCTION_OBSOLETE: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x23E;
pub const ERROR_NO_AUTHENTICODE_CATALOG: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x23F;
pub const ERROR_AUTHENTICODE_DISALLOWED: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x240;
pub const ERROR_AUTHENTICODE_TRUSTED_PUBLISHER: DWORD = APPLICATION_ERROR_MASK
    | ERROR_SEVERITY_ERROR | 0x241;
pub const ERROR_AUTHENTICODE_TRUST_NOT_ESTABLISHED: DWORD = APPLICATION_ERROR_MASK
    | ERROR_SEVERITY_ERROR | 0x242;
pub const ERROR_AUTHENTICODE_PUBLISHER_NOT_TRUSTED: DWORD = APPLICATION_ERROR_MASK
    | ERROR_SEVERITY_ERROR | 0x243;
pub const ERROR_SIGNATURE_OSATTRIBUTE_MISMATCH: DWORD = APPLICATION_ERROR_MASK
    | ERROR_SEVERITY_ERROR | 0x244;
pub const ERROR_ONLY_VALIDATE_VIA_AUTHENTICODE: DWORD = APPLICATION_ERROR_MASK
    | ERROR_SEVERITY_ERROR | 0x245;
pub const ERROR_DEVICE_INSTALLER_NOT_READY: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x246;
pub const ERROR_DRIVER_STORE_ADD_FAILED: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x247;
pub const ERROR_DEVICE_INSTALL_BLOCKED: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x248;
pub const ERROR_DRIVER_INSTALL_BLOCKED: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x249;
pub const ERROR_WRONG_INF_TYPE: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR | 0x24A;
pub const ERROR_FILE_HASH_NOT_IN_CATALOG: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x24B;
pub const ERROR_DRIVER_STORE_DELETE_FAILED: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x24C;
pub const ERROR_UNRECOVERABLE_STACK_OVERFLOW: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x300;
pub const EXCEPTION_SPAPI_UNRECOVERABLE_STACK_OVERFLOW: DWORD = ERROR_UNRECOVERABLE_STACK_OVERFLOW;
pub const ERROR_NO_DEFAULT_INTERFACE_DEVICE: DWORD = ERROR_NO_DEFAULT_DEVICE_INTERFACE;
pub const ERROR_INTERFACE_DEVICE_ACTIVE: DWORD = ERROR_DEVICE_INTERFACE_ACTIVE;
pub const ERROR_INTERFACE_DEVICE_REMOVED: DWORD = ERROR_DEVICE_INTERFACE_REMOVED;
pub const ERROR_NO_SUCH_INTERFACE_DEVICE: DWORD = ERROR_NO_SUCH_DEVICE_INTERFACE;
pub const ERROR_NOT_INSTALLED: DWORD = APPLICATION_ERROR_MASK | ERROR_SEVERITY_ERROR
    | 0x1000;
extern "system" {
    pub fn SetupGetInfInformationA(
        InfSpec: LPCVOID,
        SearchControl: DWORD,
        ReturnBuffer: PSP_INF_INFORMATION,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetInfInformationW(
        InfSpec: LPCVOID,
        SearchControl: DWORD,
        ReturnBuffer: PSP_INF_INFORMATION,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
}
pub const INFINFO_INF_SPEC_IS_HINF: DWORD = 1;
pub const INFINFO_INF_NAME_IS_ABSOLUTE: DWORD = 2;
pub const INFINFO_DEFAULT_SEARCH: DWORD = 3;
pub const INFINFO_REVERSE_DEFAULT_SEARCH: DWORD = 4;
pub const INFINFO_INF_PATH_LIST_SEARCH: DWORD = 5;
extern "system" {
    pub fn SetupQueryInfFileInformationA(
        InfInformation: PSP_INF_INFORMATION,
        InfIndex: UINT,
        ReturnBuffer: PSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupQueryInfFileInformationW(
        InfInformation: PSP_INF_INFORMATION,
        InfIndex: UINT,
        ReturnBuffer: PWSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupQueryInfOriginalFileInformationA(
        InfInformation: PSP_INF_INFORMATION,
        InfIndex: UINT,
        AlternatePlatformInfo: PSP_ALTPLATFORM_INFO,
        OriginalFileInfo: PSP_ORIGINAL_FILE_INFO_A,
    ) -> BOOL;
    pub fn SetupQueryInfOriginalFileInformationW(
        InfInformation: PSP_INF_INFORMATION,
        InfIndex: UINT,
        AlternatePlatformInfo: PSP_ALTPLATFORM_INFO,
        OriginalFileInfo: PSP_ORIGINAL_FILE_INFO_W,
    ) -> BOOL;
    pub fn SetupQueryInfVersionInformationA(
        InfInformation: PSP_INF_INFORMATION,
        InfIndex: UINT,
        Key: PCSTR,
        ReturnBuffer: PSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupQueryInfVersionInformationW(
        InfInformation: PSP_INF_INFORMATION,
        InfIndex: UINT,
        Key: PCWSTR,
        ReturnBuffer: PWSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetInfDriverStoreLocationA(
        FileName: PCSTR,
        AlternatePlatformInfo: PSP_ALTPLATFORM_INFO,
        LocaleName: PCSTR,
        ReturnBuffer: PSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetInfDriverStoreLocationW(
        FileName: PCWSTR,
        AlternatePlatformInfo: PSP_ALTPLATFORM_INFO,
        LocaleName: PCWSTR,
        ReturnBuffer: PWSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetInfPublishedNameA(
        DriverStoreLocation: PCSTR,
        ReturnBuffer: PSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetInfPublishedNameW(
        DriverStoreLocation: PCWSTR,
        ReturnBuffer: PWSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetInfFileListA(
        DirectoryPath: PCSTR,
        InfStyle: DWORD,
        ReturnBuffer: PSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetInfFileListW(
        DirectoryPath: PCWSTR,
        InfStyle: DWORD,
        ReturnBuffer: PWSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupOpenInfFileW(
        FileName: PCWSTR,
        InfClass: PCWSTR,
        InfStyle: DWORD,
        ErrorLine: PUINT,
    ) -> HINF;
    pub fn SetupOpenInfFileA(
        FileName: PCSTR,
        InfClass: PCSTR,
        InfStyle: DWORD,
        ErrorLine: PUINT,
    ) -> HINF;
    pub fn SetupOpenMasterInf() -> HINF;
    pub fn SetupOpenAppendInfFileW(
        FileName: PCWSTR,
        InfHandle: HINF,
        ErrorLine: PUINT,
    ) -> BOOL;
    pub fn SetupOpenAppendInfFileA(
        FileName: PCSTR,
        InfHandle: HINF,
        ErrorLine: PUINT,
    ) -> BOOL;
    pub fn SetupCloseInfFile(
        InfHandle: HINF,
    ) -> ();
    pub fn SetupFindFirstLineA(
        InfHandle: HINF,
        Section: PCSTR,
        Key: PCSTR,
        Context: PINFCONTEXT,
    ) -> BOOL;
    pub fn SetupFindFirstLineW(
        InfHandle: HINF,
        Section: PCWSTR,
        Key: PCWSTR,
        Context: PINFCONTEXT,
    ) -> BOOL;
    pub fn SetupFindNextLine(
        ContextIn: PINFCONTEXT,
        ContextOut: PINFCONTEXT,
    ) -> BOOL;
    pub fn SetupFindNextMatchLineA(
        ContextIn: PINFCONTEXT,
        Key: PCSTR,
        ContextOut: PINFCONTEXT,
    ) -> BOOL;
    pub fn SetupFindNextMatchLineW(
        ContextIn: PINFCONTEXT,
        Key: PCWSTR,
        ContextOut: PINFCONTEXT,
    ) -> BOOL;
    pub fn SetupGetLineByIndexA(
        InfHandle: HINF,
        Section: PCSTR,
        Index: DWORD,
        Context: PINFCONTEXT,
    ) -> BOOL;
    pub fn SetupGetLineByIndexW(
        InfHandle: HINF,
        Section: PCWSTR,
        Index: DWORD,
        Context: PINFCONTEXT,
    ) -> BOOL;
    pub fn SetupGetLineCountA(
        InfHandle: HINF,
        Section: PCSTR,
    ) -> LONG;
    pub fn SetupGetLineCountW(
        InfHandle: HINF,
        Section: PCWSTR,
    ) -> LONG;
    pub fn SetupGetLineTextA(
        Context: PINFCONTEXT,
        InfHandle: HINF,
        Section: PCSTR,
        Key: PCSTR,
        ReturnBuffer: PSTR,
        ReturnBufferSize: DWORD,
        ReturnBufferSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetLineTextW(
        Context: PINFCONTEXT,
        InfHandle: HINF,
        Section: PCWSTR,
        Key: PCWSTR,
        ReturnBuffer: PWSTR,
        ReturnBufferSize: DWORD,
        ReturnBufferSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetFieldCount(
        Context: PINFCONTEXT,
    ) -> DWORD;
    pub fn SetupGetStringFieldA(
        Context: PINFCONTEXT,
        FieldIndex: DWORD,
        ReturnBuffer: PSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetStringFieldW(
        Context: PINFCONTEXT,
        FieldIndex: DWORD,
        ReturnBuffer: PWSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetIntField(
        Context: PINFCONTEXT,
        FieldIndex: DWORD,
        IntegerValue: PINT,
    ) -> BOOL;
    pub fn SetupGetMultiSzFieldA(
        Context: PINFCONTEXT,
        FieldIndex: DWORD,
        ReturnBuffer: PSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: LPDWORD,
    ) -> BOOL;
    pub fn SetupGetMultiSzFieldW(
        Context: PINFCONTEXT,
        FieldIndex: DWORD,
        ReturnBuffer: PWSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: LPDWORD,
    ) -> BOOL;
    pub fn SetupGetBinaryField(
        Context: PINFCONTEXT,
        FieldIndex: DWORD,
        ReturnBuffer: PBYTE,
        ReturnBufferSize: DWORD,
        RequiredSize: LPDWORD,
    ) -> BOOL;
    pub fn SetupGetFileCompressionInfoA(
        SourceFileName: PCSTR,
        ActualSourceFileName: *mut PSTR,
        SourceFileSize: PDWORD,
        TargetFileSize: PDWORD,
        CompressionType: PUINT,
    ) -> DWORD;
    pub fn SetupGetFileCompressionInfoW(
        SourceFileName: PCWSTR,
        ActualSourceFileName: *mut PWSTR,
        SourceFileSize: PDWORD,
        TargetFileSize: PDWORD,
        CompressionType: PUINT,
    ) -> DWORD;
    pub fn SetupGetFileCompressionInfoExA(
        SourceFileName: PCSTR,
        ActualSourceFileNameBuffer: PSTR,
        ActualSourceFileNameBufferLen: DWORD,
        RequiredBufferLen: PDWORD,
        SourceFileSize: PDWORD,
        TargetFileSize: PDWORD,
        CompressionType: PUINT,
    ) -> BOOL;
    pub fn SetupGetFileCompressionInfoExW(
        SourceFileName: PCWSTR,
        ActualSourceFileNameBuffer: PWSTR,
        ActualSourceFileNameBufferLen: DWORD,
        RequiredBufferLen: PDWORD,
        SourceFileSize: PDWORD,
        TargetFileSize: PDWORD,
        CompressionType: PUINT,
    ) -> BOOL;
}
pub const FILE_COMPRESSION_NONE: UINT = 0;
pub const FILE_COMPRESSION_WINLZA: UINT = 1;
pub const FILE_COMPRESSION_MSZIP: UINT = 2;
pub const FILE_COMPRESSION_NTCAB: UINT = 3;
extern "system" {
    pub fn SetupDecompressOrCopyFileA(
        SourceFileName: PCSTR,
        TargetFileName: PCSTR,
        CompressionType: PUINT,
    ) -> DWORD;
    pub fn SetupDecompressOrCopyFileW(
        SourceFileName: PCWSTR,
        TargetFileName: PCWSTR,
        CompressionType: PUINT,
    ) -> DWORD;
    pub fn SetupGetSourceFileLocationA(
        InfHandle: HINF,
        InfContext: PINFCONTEXT,
        FileName: PCSTR,
        SourceId: PUINT,
        ReturnBuffer: PSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetSourceFileLocationW(
        InfHandle: HINF,
        InfContext: PINFCONTEXT,
        FileName: PCWSTR,
        SourceId: PUINT,
        ReturnBuffer: PWSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetSourceFileSizeA(
        InfHandle: HINF,
        InfContext: PINFCONTEXT,
        FileName: PCSTR,
        Section: PCSTR,
        FileSize: PDWORD,
        RoundingFactor: UINT,
    ) -> BOOL;
    pub fn SetupGetSourceFileSizeW(
        InfHandle: HINF,
        InfContext: PINFCONTEXT,
        FileName: PCWSTR,
        Section: PCWSTR,
        FileSize: PDWORD,
        RoundingFactor: UINT,
    ) -> BOOL;
    pub fn SetupGetTargetPathA(
        InfHandle: HINF,
        InfContext: PINFCONTEXT,
        Section: PCSTR,
        ReturnBuffer: PSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetTargetPathW(
        InfHandle: HINF,
        InfContext: PINFCONTEXT,
        Section: PCWSTR,
        ReturnBuffer: PWSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
}
pub const SRCLIST_TEMPORARY: DWORD = 0x00000001;
pub const SRCLIST_NOBROWSE: DWORD = 0x00000002;
pub const SRCLIST_SYSTEM: DWORD = 0x00000010;
pub const SRCLIST_USER: DWORD = 0x00000020;
pub const SRCLIST_SYSIFADMIN: DWORD = 0x00000040;
pub const SRCLIST_SUBDIRS: DWORD = 0x00000100;
pub const SRCLIST_APPEND: DWORD = 0x00000200;
pub const SRCLIST_NOSTRIPPLATFORM: DWORD = 0x00000400;
extern "system" {
    pub fn SetupSetSourceListA(
        Flags: DWORD,
        SourceList: *mut PCSTR,
        SourceCount: UINT,
    ) -> BOOL;
    pub fn SetupSetSourceListW(
        Flags: DWORD,
        SourceList: *mut PCWSTR,
        SourceCount: UINT,
    ) -> BOOL;
    pub fn SetupCancelTemporarySourceList() -> BOOL;
    pub fn SetupAddToSourceListA(
        Flags: DWORD,
        Source: PCSTR,
    ) -> BOOL;
    pub fn SetupAddToSourceListW(
        Flags: DWORD,
        Source: PCWSTR,
    ) -> BOOL;
    pub fn SetupRemoveFromSourceListA(
        Flags: DWORD,
        Source: PCSTR,
    ) -> BOOL;
    pub fn SetupRemoveFromSourceListW(
        Flags: DWORD,
        Source: PCWSTR,
    ) -> BOOL;
    pub fn SetupQuerySourceListA(
        Flags: DWORD,
        List: *mut *mut PCSTR,
        Count: PUINT,
    ) -> BOOL;
    pub fn SetupQuerySourceListW(
        Flags: DWORD,
        List: *mut *mut PCWSTR,
        Count: PUINT,
    ) -> BOOL;
    pub fn SetupFreeSourceListA(
        List: *mut *mut PCSTR,
        Count: UINT,
    ) -> BOOL;
    pub fn SetupFreeSourceListW(
        List: *mut *mut PCWSTR,
        Count: UINT,
    ) -> BOOL;
    pub fn SetupPromptForDiskA(
        hwndParent: HWND,
        DialogTitle: PCSTR,
        DiskName: PCSTR,
        PathToSource: PCSTR,
        FileSought: PCSTR,
        TagFile: PCSTR,
        DiskPromptStyle: DWORD,
        PathBuffer: PSTR,
        PathBufferSize: DWORD,
        PathRequiredSize: PDWORD,
    ) -> UINT;
    pub fn SetupPromptForDiskW(
        hwndParent: HWND,
        DialogTitle: PCWSTR,
        DiskName: PCWSTR,
        PathToSource: PCWSTR,
        FileSought: PCWSTR,
        TagFile: PCWSTR,
        DiskPromptStyle: DWORD,
        PathBuffer: PWSTR,
        PathBufferSize: DWORD,
        PathRequiredSize: PDWORD,
    ) -> UINT;
    pub fn SetupCopyErrorA(
        hwndParent: HWND,
        DialogTitle: PCSTR,
        DiskName: PCSTR,
        PathToSource: PCSTR,
        SourceFile: PCSTR,
        TargetPathFile: PCSTR,
        Win32ErrorCode: UINT,
        Style: DWORD,
        PathBuffer: PSTR,
        PathBufferSize: DWORD,
        PathRequiredSize: PDWORD,
    ) -> UINT;
    pub fn SetupCopyErrorW(
        hwndParent: HWND,
        DialogTitle: PCWSTR,
        DiskName: PCWSTR,
        PathToSource: PCWSTR,
        SourceFile: PCWSTR,
        TargetPathFile: PCWSTR,
        Win32ErrorCode: UINT,
        Style: DWORD,
        PathBuffer: PWSTR,
        PathBufferSize: DWORD,
        PathRequiredSize: PDWORD,
    ) -> UINT;
    pub fn SetupRenameErrorA(
        hwndParent: HWND,
        DialogTitle: PCSTR,
        SourceFile: PCSTR,
        TargetFile: PCSTR,
        Win32ErrorCode: UINT,
        Style: DWORD,
    ) -> UINT;
    pub fn SetupRenameErrorW(
        hwndParent: HWND,
        DialogTitle: PCWSTR,
        SourceFile: PCWSTR,
        TargetFile: PCWSTR,
        Win32ErrorCode: UINT,
        Style: DWORD,
    ) -> UINT;
    pub fn SetupDeleteErrorA(
        hwndParent: HWND,
        DialogTitle: PCSTR,
        File: PCSTR,
        Win32ErrorCode: UINT,
        Style: DWORD,
    ) -> UINT;
    pub fn SetupDeleteErrorW(
        hwndParent: HWND,
        DialogTitle: PCWSTR,
        File: PCWSTR,
        Win32ErrorCode: UINT,
        Style: DWORD,
    ) -> UINT;
    pub fn SetupBackupErrorA(
        hwndParent: HWND,
        DialogTitle: PCSTR,
        SourceFile: PCSTR,
        TargetFile: PCSTR,
        Win32ErrorCode: UINT,
        Style: DWORD,
    ) -> UINT;
    pub fn SetupBackupErrorW(
        hwndParent: HWND,
        DialogTitle: PCWSTR,
        SourceFile: PCWSTR,
        TargetFile: PCWSTR,
        Win32ErrorCode: UINT,
        Style: DWORD,
    ) -> UINT;
}
pub const IDF_NOBROWSE: DWORD = 0x00000001;
pub const IDF_NOSKIP: DWORD = 0x00000002;
pub const IDF_NODETAILS: DWORD = 0x00000004;
pub const IDF_NOCOMPRESSED: DWORD = 0x00000008;
pub const IDF_CHECKFIRST: DWORD = 0x00000100;
pub const IDF_NOBEEP: DWORD = 0x00000200;
pub const IDF_NOFOREGROUND: DWORD = 0x00000400;
pub const IDF_WARNIFSKIP: DWORD = 0x00000800;
pub const IDF_NOREMOVABLEMEDIAPROMPT: DWORD = 0x00001000;
pub const IDF_USEDISKNAMEASPROMPT: DWORD = 0x00002000;
pub const IDF_OEMDISK: DWORD = 0x80000000;
pub const DPROMPT_SUCCESS: UINT = 0;
pub const DPROMPT_CANCEL: UINT = 1;
pub const DPROMPT_SKIPFILE: UINT = 2;
pub const DPROMPT_BUFFERTOOSMALL: UINT = 3;
pub const DPROMPT_OUTOFMEMORY: UINT = 4;
extern "system" {
    pub fn SetupSetDirectoryIdA(
        InfHandle: HINF,
        Id: DWORD,
        Directory: PCSTR,
    ) -> BOOL;
    pub fn SetupSetDirectoryIdW(
        InfHandle: HINF,
        Id: DWORD,
        Directory: PCWSTR,
    ) -> BOOL;
    pub fn SetupSetDirectoryIdExA(
        InfHandle: HINF,
        Id: DWORD,
        Directory: PCSTR,
        Flags: DWORD,
        Reserved1: DWORD,
        Reserved2: PVOID,
    ) -> BOOL;
    pub fn SetupSetDirectoryIdExW(
        InfHandle: HINF,
        Id: DWORD,
        Directory: PCWSTR,
        Flags: DWORD,
        Reserved1: DWORD,
        Reserved2: PVOID,
    ) -> BOOL;
}
pub const SETDIRID_NOT_FULL_PATH: DWORD = 0x00000001;
extern "system" {
    pub fn SetupGetSourceInfoA(
        InfHandle: HINF,
        SourceId: UINT,
        InfoDesired: UINT,
        ReturnBuffer: PSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetSourceInfoW(
        InfHandle: HINF,
        SourceId: UINT,
        InfoDesired: UINT,
        ReturnBuffer: PWSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
}
pub const SRCINFO_PATH: UINT = 1;
pub const SRCINFO_TAGFILE: UINT = 2;
pub const SRCINFO_DESCRIPTION: UINT = 3;
pub const SRCINFO_FLAGS: UINT = 4;
pub const SRCINFO_TAGFILE2: UINT = 4;
pub const SRC_FLAGS_CABFILE: UINT = 0x0010;
extern "system" {
    pub fn SetupInstallFileA(
        InfHandle: HINF,
        InfContext: PINFCONTEXT,
        SourceFile: PCSTR,
        SourcePathRoot: PCSTR,
        DestinationName: PCSTR,
        CopyStyle: DWORD,
        CopyMsgHandler: PSP_FILE_CALLBACK_A,
        Context: PVOID,
    ) -> BOOL;
    pub fn SetupInstallFileW(
        InfHandle: HINF,
        InfContext: PINFCONTEXT,
        SourceFile: PCWSTR,
        SourcePathRoot: PCWSTR,
        DestinationName: PCWSTR,
        CopyStyle: DWORD,
        CopyMsgHandler: PSP_FILE_CALLBACK_W,
        Context: PVOID,
    ) -> BOOL;
    pub fn SetupInstallFileExA(
        InfHandle: HINF,
        InfContext: PINFCONTEXT,
        SourceFile: PCSTR,
        SourcePathRoot: PCSTR,
        DestinationName: PCSTR,
        CopyStyle: DWORD,
        CopyMsgHandler: PSP_FILE_CALLBACK_A,
        Context: PVOID,
        FileWasInUse: PBOOL,
    ) -> BOOL;
    pub fn SetupInstallFileExW(
        InfHandle: HINF,
        InfContext: PINFCONTEXT,
        SourceFile: PCWSTR,
        SourcePathRoot: PCWSTR,
        DestinationName: PCWSTR,
        CopyStyle: DWORD,
        CopyMsgHandler: PSP_FILE_CALLBACK_W,
        Context: PVOID,
        FileWasInUse: PBOOL,
    ) -> BOOL;
}
pub const SP_COPY_DELETESOURCE: DWORD = 0x0000001;
pub const SP_COPY_REPLACEONLY: DWORD = 0x0000002;
pub const SP_COPY_NEWER: DWORD = 0x0000004;
pub const SP_COPY_NEWER_OR_SAME: DWORD = SP_COPY_NEWER;
pub const SP_COPY_NOOVERWRITE: DWORD = 0x0000008;
pub const SP_COPY_NODECOMP: DWORD = 0x0000010;
pub const SP_COPY_LANGUAGEAWARE: DWORD = 0x0000020;
pub const SP_COPY_SOURCE_ABSOLUTE: DWORD = 0x0000040;
pub const SP_COPY_SOURCEPATH_ABSOLUTE: DWORD = 0x0000080;
pub const SP_COPY_IN_USE_NEEDS_REBOOT: DWORD = 0x0000100;
pub const SP_COPY_FORCE_IN_USE: DWORD = 0x0000200;
pub const SP_COPY_NOSKIP: DWORD = 0x0000400;
pub const SP_FLAG_CABINETCONTINUATION: DWORD = 0x0000800;
pub const SP_COPY_FORCE_NOOVERWRITE: DWORD = 0x0001000;
pub const SP_COPY_FORCE_NEWER: DWORD = 0x0002000;
pub const SP_COPY_WARNIFSKIP: DWORD = 0x0004000;
pub const SP_COPY_NOBROWSE: DWORD = 0x0008000;
pub const SP_COPY_NEWER_ONLY: DWORD = 0x0010000;
pub const SP_COPY_RESERVED: DWORD = 0x0020000;
pub const SP_COPY_OEMINF_CATALOG_ONLY: DWORD = 0x0040000;
pub const SP_COPY_REPLACE_BOOT_FILE: DWORD = 0x0080000;
pub const SP_COPY_NOPRUNE: DWORD = 0x0100000;
pub const SP_COPY_OEM_F6_INF: DWORD = 0x0200000;
pub const SP_COPY_ALREADYDECOMP: DWORD = 0x0400000;
pub const SP_COPY_WINDOWS_SIGNED: DWORD = 0x1000000;
pub const SP_COPY_PNPLOCKED: DWORD = 0x2000000;
pub const SP_COPY_IN_USE_TRY_RENAME: DWORD = 0x4000000;
pub const SP_COPY_INBOX_INF: DWORD = 0x8000000;
pub const SP_COPY_HARDLINK: DWORD = 0x10000000;
pub const SP_BACKUP_BACKUPPASS: DWORD = 0x00000001;
pub const SP_BACKUP_DEMANDPASS: DWORD = 0x00000002;
pub const SP_BACKUP_SPECIAL: DWORD = 0x00000004;
pub const SP_BACKUP_BOOTFILE: DWORD = 0x00000008;
extern "system" {
    pub fn SetupOpenFileQueue(
    ) -> HSPFILEQ;
    pub fn SetupCloseFileQueue(
        QueueHandle: HSPFILEQ,
    ) -> BOOL;
    pub fn SetupSetFileQueueAlternatePlatformA(
        QueueHandle: HSPFILEQ,
        AlternatePlatformInfo: PSP_ALTPLATFORM_INFO,
        AlternateDefaultCatalogFile: PCSTR,
    ) -> BOOL;
    pub fn SetupSetFileQueueAlternatePlatformW(
        QueueHandle: HSPFILEQ,
        AlternatePlatformInfo: PSP_ALTPLATFORM_INFO,
        AlternateDefaultCatalogFile: PCWSTR,
    ) -> BOOL;
    pub fn SetupSetPlatformPathOverrideA(
        Override: PCSTR,
    ) -> BOOL;
    pub fn SetupSetPlatformPathOverrideW(
        Override: PCWSTR,
    ) -> BOOL;
    pub fn SetupQueueCopyA(
        QueueHandle: HSPFILEQ,
        SourceRootPath: PCSTR,
        SourcePath: PCSTR,
        SourceFilename: PCSTR,
        SourceDescription: PCSTR,
        SourceTagfile: PCSTR,
        TargetDirectory: PCSTR,
        TargetFilename: PCSTR,
        CopyStyle: DWORD,
    ) -> BOOL;
    pub fn SetupQueueCopyW(
        QueueHandle: HSPFILEQ,
        SourceRootPath: PCWSTR,
        SourcePath: PCWSTR,
        SourceFilename: PCWSTR,
        SourceDescription: PCWSTR,
        SourceTagfile: PCWSTR,
        TargetDirectory: PCWSTR,
        TargetFilename: PCWSTR,
        CopyStyle: DWORD,
    ) -> BOOL;
    pub fn SetupQueueCopyIndirectA(
        CopyParams: PSP_FILE_COPY_PARAMS_A,
    ) -> BOOL;
    pub fn SetupQueueCopyIndirectW(
        CopyParams: PSP_FILE_COPY_PARAMS_W,
    ) -> BOOL;
    pub fn SetupQueueDefaultCopyA(
        QueueHandle: HSPFILEQ,
        InfHandle: HINF,
        SourceRootPath: PCSTR,
        SourceFilename: PCSTR,
        TargetFilename: PCSTR,
        CopyStyle: DWORD,
    ) -> BOOL;
    pub fn SetupQueueDefaultCopyW(
        QueueHandle: HSPFILEQ,
        InfHandle: HINF,
        SourceRootPath: PCWSTR,
        SourceFilename: PCWSTR,
        TargetFilename: PCWSTR,
        CopyStyle: DWORD,
    ) -> BOOL;
    pub fn SetupQueueCopySectionA(
        QueueHandle: HSPFILEQ,
        SourceRootPath: PCSTR,
        InfHandle: HINF,
        ListInfHandle: HINF,
        Section: PCSTR,
        CopyStyle: DWORD,
    ) -> BOOL;
    pub fn SetupQueueCopySectionW(
        QueueHandle: HSPFILEQ,
        SourceRootPath: PCWSTR,
        InfHandle: HINF,
        ListInfHandle: HINF,
        Section: PCWSTR,
        CopyStyle: DWORD,
    ) -> BOOL;
    pub fn SetupQueueDeleteA(
        QueueHandle: HSPFILEQ,
        PathPart1: PCSTR,
        PathPart2: PCSTR,
    ) -> BOOL;
    pub fn SetupQueueDeleteW(
        QueueHandle: HSPFILEQ,
        PathPart1: PCWSTR,
        PathPart2: PCWSTR,
    ) -> BOOL;
    pub fn SetupQueueDeleteSectionA(
        QueueHandle: HSPFILEQ,
        InfHandle: HINF,
        ListInfHandle: HINF,
        Section: PCSTR,
    ) -> BOOL;
    pub fn SetupQueueDeleteSectionW(
        QueueHandle: HSPFILEQ,
        InfHandle: HINF,
        ListInfHandle: HINF,
        Section: PCWSTR,
    ) -> BOOL;
    pub fn SetupQueueRenameA(
        QueueHandle: HSPFILEQ,
        SourcePath: PCSTR,
        SourceFilename: PCSTR,
        TargetPath: PCSTR,
        TargetFilename: PCSTR,
    ) -> BOOL;
    pub fn SetupQueueRenameW(
        QueueHandle: HSPFILEQ,
        SourcePath: PCWSTR,
        SourceFilename: PCWSTR,
        TargetPath: PCWSTR,
        TargetFilename: PCWSTR,
    ) -> BOOL;
    pub fn SetupQueueRenameSectionA(
        QueueHandle: HSPFILEQ,
        InfHandle: HINF,
        ListInfHandle: HINF,
        Section: PCSTR,
    ) -> BOOL;
    pub fn SetupQueueRenameSectionW(
        QueueHandle: HSPFILEQ,
        InfHandle: HINF,
        ListInfHandle: HINF,
        Section: PCWSTR,
    ) -> BOOL;
    pub fn SetupCommitFileQueueA(
        Owner: HWND,
        QueueHandle: HSPFILEQ,
        MsgHandler: PSP_FILE_CALLBACK_A,
        Context: PVOID,
    ) -> BOOL;
    pub fn SetupCommitFileQueueW(
        Owner: HWND,
        QueueHandle: HSPFILEQ,
        MsgHandler: PSP_FILE_CALLBACK_W,
        Context: PVOID,
    ) -> BOOL;
    pub fn SetupScanFileQueueA(
        FileQueue: HSPFILEQ,
        Flags: DWORD,
        Window: HWND,
        CallbackRoutine: PSP_FILE_CALLBACK_A,
        CallbackContext: PVOID,
        Result: PDWORD,
    ) -> BOOL;
    pub fn SetupScanFileQueueW(
        FileQueue: HSPFILEQ,
        Flags: DWORD,
        Window: HWND,
        CallbackRoutine: PSP_FILE_CALLBACK_W,
        CallbackContext: PVOID,
        Result: PDWORD,
    ) -> BOOL;
}
pub const SPQ_SCAN_FILE_PRESENCE: DWORD = 0x00000001;
pub const SPQ_SCAN_FILE_VALIDITY: DWORD = 0x00000002;
pub const SPQ_SCAN_USE_CALLBACK: DWORD = 0x00000004;
pub const SPQ_SCAN_USE_CALLBACKEX: DWORD = 0x00000008;
pub const SPQ_SCAN_INFORM_USER: DWORD = 0x00000010;
pub const SPQ_SCAN_PRUNE_COPY_QUEUE: DWORD = 0x00000020;
pub const SPQ_SCAN_USE_CALLBACK_SIGNERINFO: DWORD = 0x00000040;
pub const SPQ_SCAN_PRUNE_DELREN: DWORD = 0x00000080;
pub const SPQ_SCAN_FILE_PRESENCE_WITHOUT_SOURCE: DWORD = 0x00000100;
pub const SPQ_SCAN_FILE_COMPARISON: DWORD = 0x00000200;
pub const SPQ_SCAN_ACTIVATE_DRP: DWORD = 0x00000400;
pub const SPQ_DELAYED_COPY: DWORD = 0x00000001;
extern "system" {
    pub fn SetupGetFileQueueCount(
        FileQueue: HSPFILEQ,
        SubQueueFileOp: UINT,
        NumOperations: PUINT,
    ) -> BOOL;
    pub fn SetupGetFileQueueFlags(
        FileQueue: HSPFILEQ,
        Flags: PDWORD,
    ) -> BOOL;
    pub fn SetupSetFileQueueFlags(
        FileQueue: HSPFILEQ,
        FlagMask: DWORD,
        Flags: DWORD,
    ) -> BOOL;
}
pub const SPQ_FLAG_BACKUP_AWARE: DWORD = 0x00000001;
pub const SPQ_FLAG_ABORT_IF_UNSIGNED: DWORD = 0x00000002;
pub const SPQ_FLAG_FILES_MODIFIED: DWORD = 0x00000004;
pub const SPQ_FLAG_DO_SHUFFLEMOVE: DWORD = 0x00000008;
pub const SPQ_FLAG_VALID: DWORD = 0x0000000F;
pub const SPOST_NONE: DWORD = 0;
pub const SPOST_PATH: DWORD = 1;
pub const SPOST_URL: DWORD = 2;
pub const SPOST_MAX: DWORD = 3;
extern "system" {
    pub fn SetupCopyOEMInfA(
        SourceInfFileName: PCSTR,
        OEMSourceMediaLocation: PCSTR,
        OEMSourceMediaType: DWORD,
        CopyStyle: DWORD,
        DestinationInfFileName: PSTR,
        DestinationInfFileNameSize: DWORD,
        RequiredSize: PDWORD,
        DestinationInfFileNameComponent: *mut PSTR,
    ) -> BOOL;
    pub fn SetupCopyOEMInfW(
        SourceInfFileName: PCWSTR,
        OEMSourceMediaLocation: PCWSTR,
        OEMSourceMediaType: DWORD,
        CopyStyle: DWORD,
        DestinationInfFileName: PWSTR,
        DestinationInfFileNameSize: DWORD,
        RequiredSize: PDWORD,
        DestinationInfFileNameComponent: *mut PWSTR,
    ) -> BOOL;
}
pub const SUOI_FORCEDELETE: DWORD = 0x00000001;
pub const SUOI_INTERNAL1: DWORD = 0x00000002;
extern "system" {
    pub fn SetupUninstallOEMInfA(
        InfFileName: PCSTR,
        Flags: DWORD,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupUninstallOEMInfW(
        InfFileName: PCWSTR,
        Flags: DWORD,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupUninstallNewlyCopiedInfs(
        FileQueue: HSPFILEQ,
        Flags: DWORD,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupCreateDiskSpaceListA(
        Reserved1: PVOID,
        Reserved2: DWORD,
        Flags: UINT,
    ) -> HDSKSPC;
    pub fn SetupCreateDiskSpaceListW(
        Reserved1: PVOID,
        Reserved2: DWORD,
        Flags: UINT,
    ) -> HDSKSPC;
}
pub const SPDSL_IGNORE_DISK: UINT = 0x00000001;
pub const SPDSL_DISALLOW_NEGATIVE_ADJUST: UINT = 0x00000002;
extern "system" {
    pub fn SetupDuplicateDiskSpaceListA(
        DiskSpace: HDSKSPC,
        Reserved1: PVOID,
        Reserved2: DWORD,
        Flags: UINT,
    ) -> HDSKSPC;
    pub fn SetupDuplicateDiskSpaceListW(
        DiskSpace: HDSKSPC,
        Reserved1: PVOID,
        Reserved2: DWORD,
        Flags: UINT,
    ) -> HDSKSPC;
    pub fn SetupDestroyDiskSpaceList(
        DiskSpace: HDSKSPC,
    ) -> BOOL;
    pub fn SetupQueryDrivesInDiskSpaceListA(
        DiskSpace: HDSKSPC,
        ReturnBuffer: PSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupQueryDrivesInDiskSpaceListW(
        DiskSpace: HDSKSPC,
        ReturnBuffer: PWSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupQuerySpaceRequiredOnDriveA(
        DiskSpace: HDSKSPC,
        DriveSpec: PCSTR,
        SpaceRequired: *mut LONGLONG,
        Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupQuerySpaceRequiredOnDriveW(
        DiskSpace: HDSKSPC,
        DriveSpec: PCWSTR,
        SpaceRequired: *mut LONGLONG,
        Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupAdjustDiskSpaceListA(
        DiskSpace: HDSKSPC,
        DriveRoot: LPCSTR,
        Amount: LONGLONG,
        Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupAdjustDiskSpaceListW(
        DiskSpace: HDSKSPC,
        DriveRoot: LPCWSTR,
        Amount: LONGLONG,
        Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupAddToDiskSpaceListA(
        DiskSpace: HDSKSPC,
        TargetFilespec: PCSTR,
        FileSize: LONGLONG,
        Operation: UINT,
        Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupAddToDiskSpaceListW(
        DiskSpace: HDSKSPC,
        TargetFilespec: PCWSTR,
        FileSize: LONGLONG,
        Operation: UINT,
        Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupAddSectionToDiskSpaceListA(
        DiskSpace: HDSKSPC,
        InfHandle: HINF,
        ListInfHandle: HINF,
        SectionName: PCSTR,
        Operation: UINT,
        Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupAddSectionToDiskSpaceListW(
        DiskSpace: HDSKSPC,
        InfHandle: HINF,
        ListInfHandle: HINF,
        SectionName: PCWSTR,
        Operation: UINT,
        Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupAddInstallSectionToDiskSpaceListA(
        DiskSpace: HDSKSPC,
        InfHandle: HINF,
        LayoutInfHandle: HINF,
        SectionName: PCSTR,
        Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupAddInstallSectionToDiskSpaceListW(
        DiskSpace: HDSKSPC,
        InfHandle: HINF,
        LayoutInfHandle: HINF,
        SectionName: PCWSTR,
        Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupRemoveFromDiskSpaceListA(
        DiskSpace: HDSKSPC,
        TargetFilespec: PCSTR,
        Operation: UINT,
        Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupRemoveFromDiskSpaceListW(
        DiskSpace: HDSKSPC,
        TargetFilespec: PCWSTR,
        Operation: UINT,
        Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupRemoveSectionFromDiskSpaceListA(
        DiskSpace: HDSKSPC,
        InfHandle: HINF,
        ListInfHandle: HINF,
        SectionName: PCSTR,
        Operation: UINT,
        Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupRemoveSectionFromDiskSpaceListW(
        DiskSpace: HDSKSPC,
        InfHandle: HINF,
        ListInfHandle: HINF,
        SectionName: PCWSTR,
        Operation: UINT,
        Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupRemoveInstallSectionFromDiskSpaceListA(
        DiskSpace: HDSKSPC,
        InfHandle: HINF,
        LayoutInfHandle: HINF,
        SectionName: PCSTR,
        Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupRemoveInstallSectionFromDiskSpaceListW(
        DiskSpace: HDSKSPC,
        InfHandle: HINF,
        LayoutInfHandle: HINF,
        SectionName: PCWSTR,
        Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupIterateCabinetA(
        CabinetFile: PCSTR,
        Reserved: DWORD,
        MsgHandler: PSP_FILE_CALLBACK_A,
        Context: PVOID,
    ) -> BOOL;
    pub fn SetupIterateCabinetW(
        CabinetFile: PCWSTR,
        Reserved: DWORD,
        MsgHandler: PSP_FILE_CALLBACK_W,
        Context: PVOID,
    ) -> BOOL;
    pub fn SetupPromptReboot(
        FileQueue: HSPFILEQ,
        Owner: HWND,
        ScanOnly: BOOL,
    ) -> INT;
}
pub const SPFILEQ_FILE_IN_USE: INT = 0x00000001;
pub const SPFILEQ_REBOOT_RECOMMENDED: INT = 0x00000002;
pub const SPFILEQ_REBOOT_IN_PROGRESS: INT = 0x00000004;
extern "system" {
    pub fn SetupInitDefaultQueueCallback(
        OwnerWindow: HWND,
    ) -> PVOID;
    pub fn SetupInitDefaultQueueCallbackEx(
        OwnerWindow: HWND,
        AlternateProgressWindow: HWND,
        ProgressMessage: UINT,
        Reserved1: DWORD,
        Reserved2: PVOID,
    ) -> PVOID;
    pub fn SetupTermDefaultQueueCallback(
        Context: PVOID,
    ) -> ();
    pub fn SetupDefaultQueueCallbackA(
        Context: PVOID,
        Notification: UINT,
        Param1: UINT_PTR,
        Param2: UINT_PTR,
    ) -> UINT;
    pub fn SetupDefaultQueueCallbackW(
        Context: PVOID,
        Notification: UINT,
        Param1: UINT_PTR,
        Param2: UINT_PTR,
    ) -> UINT;
}
pub const FLG_ADDREG_DELREG_BIT: DWORD = 0x00008000;
pub const FLG_ADDREG_BINVALUETYPE: DWORD = 0x00000001;
pub const FLG_ADDREG_NOCLOBBER: DWORD = 0x00000002;
pub const FLG_ADDREG_DELVAL: DWORD = 0x00000004;
pub const FLG_ADDREG_APPEND: DWORD = 0x00000008;
pub const FLG_ADDREG_KEYONLY: DWORD = 0x00000010;
pub const FLG_ADDREG_OVERWRITEONLY: DWORD = 0x00000020;
pub const FLG_ADDREG_64BITKEY: DWORD = 0x00001000;
pub const FLG_ADDREG_KEYONLY_COMMON: DWORD = 0x00002000;
pub const FLG_ADDREG_32BITKEY: DWORD = 0x00004000;
pub const FLG_ADDREG_TYPE_MASK: DWORD = 0xFFFF0000 | FLG_ADDREG_BINVALUETYPE;
pub const FLG_ADDREG_TYPE_SZ: DWORD = 0x00000000;
pub const FLG_ADDREG_TYPE_MULTI_SZ: DWORD = 0x00010000;
pub const FLG_ADDREG_TYPE_EXPAND_SZ: DWORD = 0x00020000;
pub const FLG_ADDREG_TYPE_BINARY: DWORD = 0x00000000 | FLG_ADDREG_BINVALUETYPE;
pub const FLG_ADDREG_TYPE_DWORD: DWORD = 0x00010000 | FLG_ADDREG_BINVALUETYPE;
pub const FLG_ADDREG_TYPE_NONE: DWORD = 0x00020000 | FLG_ADDREG_BINVALUETYPE;
pub const FLG_DELREG_VALUE: DWORD = 0x00000000;
pub const FLG_DELREG_TYPE_MASK: DWORD = FLG_ADDREG_TYPE_MASK;
pub const FLG_DELREG_TYPE_SZ: DWORD = FLG_ADDREG_TYPE_SZ;
pub const FLG_DELREG_TYPE_MULTI_SZ: DWORD = FLG_ADDREG_TYPE_MULTI_SZ;
pub const FLG_DELREG_TYPE_EXPAND_SZ: DWORD = FLG_ADDREG_TYPE_EXPAND_SZ;
pub const FLG_DELREG_TYPE_BINARY: DWORD = FLG_ADDREG_TYPE_BINARY;
pub const FLG_DELREG_TYPE_DWORD: DWORD = FLG_ADDREG_TYPE_DWORD;
pub const FLG_DELREG_TYPE_NONE: DWORD = FLG_ADDREG_TYPE_NONE;
pub const FLG_DELREG_64BITKEY: DWORD = FLG_ADDREG_64BITKEY;
pub const FLG_DELREG_KEYONLY_COMMON: DWORD = FLG_ADDREG_KEYONLY_COMMON;
pub const FLG_DELREG_32BITKEY: DWORD = FLG_ADDREG_32BITKEY;
pub const FLG_DELREG_OPERATION_MASK: DWORD = 0x000000FE;
pub const FLG_DELREG_MULTI_SZ_DELSTRING: DWORD = FLG_DELREG_TYPE_MULTI_SZ | FLG_ADDREG_DELREG_BIT
    | 0x00000002;
pub const FLG_BITREG_CLEARBITS: DWORD = 0x00000000;
pub const FLG_BITREG_SETBITS: DWORD = 0x00000001;
pub const FLG_BITREG_64BITKEY: DWORD = 0x00001000;
pub const FLG_BITREG_32BITKEY: DWORD = 0x00004000;
pub const FLG_INI2REG_64BITKEY: DWORD = 0x00001000;
pub const FLG_INI2REG_32BITKEY: DWORD = 0x00004000;
pub const FLG_REGSVR_DLLREGISTER: DWORD = 0x00000001;
pub const FLG_REGSVR_DLLINSTALL: DWORD = 0x00000002;
pub const FLG_PROFITEM_CURRENTUSER: DWORD = 0x00000001;
pub const FLG_PROFITEM_DELETE: DWORD = 0x00000002;
pub const FLG_PROFITEM_GROUP: DWORD = 0x00000004;
pub const FLG_PROFITEM_CSIDL: DWORD = 0x00000008;
pub const FLG_ADDPROPERTY_NOCLOBBER: DWORD = 0x00000001;
pub const FLG_ADDPROPERTY_OVERWRITEONLY: DWORD = 0x00000002;
pub const FLG_ADDPROPERTY_APPEND: DWORD = 0x00000004;
pub const FLG_ADDPROPERTY_OR: DWORD = 0x00000008;
pub const FLG_ADDPROPERTY_AND: DWORD = 0x00000010;
pub const FLG_DELPROPERTY_MULTI_SZ_DELSTRING: DWORD = 0x00000001;
extern "system" {
    pub fn SetupInstallFromInfSectionA(
        Owner: HWND,
        InfHandle: HINF,
        SectionName: PCSTR,
        Flags: UINT,
        RelativeKeyRoot: HKEY,
        SourceRootPath: PCSTR,
        CopyFlags: UINT,
        MsgHandler: PSP_FILE_CALLBACK_A,
        Context: PVOID,
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupInstallFromInfSectionW(
        Owner: HWND,
        InfHandle: HINF,
        SectionName: PCWSTR,
        Flags: UINT,
        RelativeKeyRoot: HKEY,
        SourceRootPath: PCWSTR,
        CopyFlags: UINT,
        MsgHandler: PSP_FILE_CALLBACK_W,
        Context: PVOID,
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
}
pub const SPINST_LOGCONFIG: UINT = 0x00000001;
pub const SPINST_INIFILES: UINT = 0x00000002;
pub const SPINST_REGISTRY: UINT = 0x00000004;
pub const SPINST_INI2REG: UINT = 0x00000008;
pub const SPINST_FILES: UINT = 0x00000010;
pub const SPINST_BITREG: UINT = 0x00000020;
pub const SPINST_REGSVR: UINT = 0x00000040;
pub const SPINST_UNREGSVR: UINT = 0x00000080;
pub const SPINST_PROFILEITEMS: UINT = 0x00000100;
pub const SPINST_COPYINF: UINT = 0x00000200;
pub const SPINST_PROPERTIES: UINT = 0x00000400;
pub const SPINST_ALL: UINT = 0x000007ff;
pub const SPINST_SINGLESECTION: UINT = 0x00010000;
pub const SPINST_LOGCONFIG_IS_FORCED: UINT = 0x00020000;
pub const SPINST_LOGCONFIGS_ARE_OVERRIDES: UINT = 0x00040000;
pub const SPINST_REGISTERCALLBACKAWARE: UINT = 0x00080000;
pub const SPINST_DEVICEINSTALL: UINT = 0x00100000;
extern "system" {
    pub fn SetupInstallFilesFromInfSectionA(
        InfHandle: HINF,
        LayoutInfHandle: HINF,
        FileQueue: HSPFILEQ,
        SectionName: PCSTR,
        SourceRootPath: PCSTR,
        CopyFlags: UINT,
    ) -> BOOL;
    pub fn SetupInstallFilesFromInfSectionW(
        InfHandle: HINF,
        LayoutInfHandle: HINF,
        FileQueue: HSPFILEQ,
        SectionName: PCWSTR,
        SourceRootPath: PCWSTR,
        CopyFlags: UINT,
    ) -> BOOL;
}
pub const SPSVCINST_TAGTOFRONT: DWORD = 0x00000001;
pub const SPSVCINST_ASSOCSERVICE: DWORD = 0x00000002;
pub const SPSVCINST_DELETEEVENTLOGENTRY: DWORD = 0x00000004;
pub const SPSVCINST_NOCLOBBER_DISPLAYNAME: DWORD = 0x00000008;
pub const SPSVCINST_NOCLOBBER_STARTTYPE: DWORD = 0x00000010;
pub const SPSVCINST_NOCLOBBER_ERRORCONTROL: DWORD = 0x00000020;
pub const SPSVCINST_NOCLOBBER_LOADORDERGROUP: DWORD = 0x00000040;
pub const SPSVCINST_NOCLOBBER_DEPENDENCIES: DWORD = 0x00000080;
pub const SPSVCINST_NOCLOBBER_DESCRIPTION: DWORD = 0x00000100;
pub const SPSVCINST_STOPSERVICE: DWORD = 0x00000200;
pub const SPSVCINST_CLOBBER_SECURITY: DWORD = 0x00000400;
pub const SPSVCINST_STARTSERVICE: DWORD = 0x00000800;
pub const SPSVCINST_NOCLOBBER_REQUIREDPRIVILEGES: DWORD = 0x00001000;
extern "system" {
    pub fn SetupInstallServicesFromInfSectionA(
        InfHandle: HINF,
        SectionName: PCSTR,
        Flags: DWORD,
    ) -> BOOL;
    pub fn SetupInstallServicesFromInfSectionW(
        InfHandle: HINF,
        SectionName: PCWSTR,
        Flags: DWORD,
    ) -> BOOL;
    pub fn SetupInstallServicesFromInfSectionExA(
        InfHandle: HINF,
        SectionName: PCSTR,
        Flags: DWORD,
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        Reserved1: PVOID,
        Reserved2: PVOID,
    ) -> BOOL;
    pub fn SetupInstallServicesFromInfSectionExW(
        InfHandle: HINF,
        SectionName: PCWSTR,
        Flags: DWORD,
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        Reserved1: PVOID,
        Reserved2: PVOID,
    ) -> BOOL;
    pub fn InstallHinfSectionA(
        Window: HWND,
        ModuleHandle: HINSTANCE,
        CommandLine: PCSTR,
        ShowCommand: INT,
    ) -> ();
    pub fn InstallHinfSectionW(
        Window: HWND,
        ModuleHandle: HINSTANCE,
        CommandLine: PCWSTR,
        ShowCommand: INT,
    ) -> ();
}
pub type HSPFILELOG = PVOID;
extern "system" {
    pub fn SetupInitializeFileLogA(
        LogFileName: PCSTR,
        Flags: DWORD,
    ) -> HSPFILELOG;
    pub fn SetupInitializeFileLogW(
        LogFileName: PCWSTR,
        Flags: DWORD,
    ) -> HSPFILELOG;
}
pub const SPFILELOG_SYSTEMLOG: DWORD = 0x00000001;
pub const SPFILELOG_FORCENEW: DWORD = 0x00000002;
pub const SPFILELOG_QUERYONLY: DWORD = 0x00000004;
extern "system" {
    pub fn SetupTerminateFileLog(
        FileLogHandle: HSPFILELOG,
    ) -> BOOL;
    pub fn SetupLogFileA(
        FileLogHandle: HSPFILELOG,
        LogSectionName: PCSTR,
        SourceFilename: PCSTR,
        TargetFilename: PCSTR,
        Checksum: DWORD,
        DiskTagfile: PCSTR,
        DiskDescription: PCSTR,
        OtherInfo: PCSTR,
        Flags: DWORD,
    ) -> BOOL;
    pub fn SetupLogFileW(
        FileLogHandle: HSPFILELOG,
        LogSectionName: PCWSTR,
        SourceFilename: PCWSTR,
        TargetFilename: PCWSTR,
        Checksum: DWORD,
        DiskTagfile: PCWSTR,
        DiskDescription: PCWSTR,
        OtherInfo: PCWSTR,
        Flags: DWORD,
    ) -> BOOL;
}
pub const SPFILELOG_OEMFILE: DWORD = 0x00000001;
extern "system" {
    pub fn SetupRemoveFileLogEntryA(
        FileLogHandle: HSPFILELOG,
        LogSectionName: PCSTR,
        TargetFilename: PCSTR,
    ) -> BOOL;
    pub fn SetupRemoveFileLogEntryW(
        FileLogHandle: HSPFILELOG,
        LogSectionName: PCWSTR,
        TargetFilename: PCWSTR,
    ) -> BOOL;
}
ENUM!{enum SetupFileLogInfo {
    SetupFileLogSourceFilename,
    SetupFileLogChecksum,
    SetupFileLogDiskTagfile,
    SetupFileLogDiskDescription,
    SetupFileLogOtherInfo,
    SetupFileLogMax,
}}
extern "system" {
    pub fn SetupQueryFileLogA(
        FileLogHandle: HSPFILELOG,
        LogSectionName: PCSTR,
        TargetFilename: PCSTR,
        DesiredInfo: SetupFileLogInfo,
        DataOut: PSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupQueryFileLogW(
        FileLogHandle: HSPFILELOG,
        LogSectionName: PCWSTR,
        TargetFilename: PCWSTR,
        DesiredInfo: SetupFileLogInfo,
        DataOut: PWSTR,
        ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
}
pub type LogSeverity = DWORD;
pub const LogSevInformation: LogSeverity = 0x00000000;
pub const LogSevWarning: LogSeverity = 0x00000001;
pub const LogSevError: LogSeverity = 0x00000002;
pub const LogSevFatalError: LogSeverity = 0x00000003;
pub const LogSevMaximum: LogSeverity = 0x00000004;
extern "system" {
    pub fn SetupOpenLog(
        Erase: BOOL,
    ) -> BOOL;
    pub fn SetupLogErrorA(
        MessageString: LPCSTR,
        Severity: LogSeverity,
    ) -> BOOL;
    pub fn SetupLogErrorW(
        MessageString: LPCWSTR,
        Severity: LogSeverity,
    ) -> BOOL;
    pub fn SetupCloseLog() -> ();
    pub fn SetupGetThreadLogToken() -> SP_LOG_TOKEN;
    pub fn SetupSetThreadLogToken(
        LogToken: SP_LOG_TOKEN,
    ) -> ();
}
//pub fn SetupWriteTextLog() -> ();
//pub fn SetupWriteTextLogError() -> ();
extern "system" {
    pub fn SetupWriteTextLogInfLine(
        LogToken: SP_LOG_TOKEN,
        Flags: DWORD,
        InfHandle: HINF,
        Context: PINFCONTEXT,
    ) -> ();
    pub fn SetupGetBackupInformationA(
        QueueHandle: HSPFILEQ,
        BackupParams: PSP_BACKUP_QUEUE_PARAMS_A,
    ) -> BOOL;
    pub fn SetupGetBackupInformationW(
        QueueHandle: HSPFILEQ,
        BackupParams: PSP_BACKUP_QUEUE_PARAMS_W,
    ) -> BOOL;
    pub fn SetupPrepareQueueForRestoreA(
        QueueHandle: HSPFILEQ,
        BackupPath: PCSTR,
        RestoreFlags: DWORD,
    ) -> BOOL;
    pub fn SetupPrepareQueueForRestoreW(
        QueueHandle: HSPFILEQ,
        BackupPath: PCWSTR,
        RestoreFlags: DWORD,
    ) -> BOOL;
    pub fn SetupSetNonInteractiveMode(
        NonInteractiveFlag: BOOL,
    ) -> BOOL;
    pub fn SetupGetNonInteractiveMode() -> BOOL;
    pub fn SetupDiCreateDeviceInfoList(
        ClassGuid: *const GUID,
        hwndParent: HWND,
    ) -> HDEVINFO;
    pub fn SetupDiCreateDeviceInfoListExA(
        ClassGuid: *const GUID,
        hwndParent: HWND,
        MachineName: PCSTR,
        Reserved: PVOID,
    ) -> HDEVINFO;
    pub fn SetupDiCreateDeviceInfoListExW(
        ClassGuid: *const GUID,
        hwndParent: HWND,
        MachineName: PCWSTR,
        Reserved: PVOID,
    ) -> HDEVINFO;
    pub fn SetupDiGetDeviceInfoListClass(
        DeviceInfoSet: HDEVINFO,
        ClassGuid: LPGUID,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInfoListDetailA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoSetDetailData: PSP_DEVINFO_LIST_DETAIL_DATA_A,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInfoListDetailW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoSetDetailData: PSP_DEVINFO_LIST_DETAIL_DATA_W,
    ) -> BOOL;
}
pub const DICD_GENERATE_ID: DWORD = 0x00000001;
pub const DICD_INHERIT_CLASSDRVS: DWORD = 0x00000002;
extern "system" {
    pub fn SetupDiCreateDeviceInfoA(
        DeviceInfoSet: HDEVINFO,
        DeviceName: PCSTR,
        ClassGuid: *const GUID,
        DeviceDescription: PCSTR,
        hwndParent: HWND,
        CreationFlags: DWORD,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiCreateDeviceInfoW(
        DeviceInfoSet: HDEVINFO,
        DeviceName: PCWSTR,
        ClassGuid: *const GUID,
        DeviceDescription: PCWSTR,
        hwndParent: HWND,
        CreationFlags: DWORD,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
}
pub const DIOD_INHERIT_CLASSDRVS: DWORD = 0x00000002;
pub const DIOD_CANCEL_REMOVE: DWORD = 0x00000004;
extern "system" {
    pub fn SetupDiOpenDeviceInfoA(
        DeviceInfoSet: HDEVINFO,
        DeviceInstanceId: PCSTR,
        hwndParent: HWND,
        OpenFlags: DWORD,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiOpenDeviceInfoW(
        DeviceInfoSet: HDEVINFO,
        DeviceInstanceId: PCWSTR,
        hwndParent: HWND,
        OpenFlags: DWORD,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInstanceIdA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DeviceInstanceId: PSTR,
        DeviceInstanceIdSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInstanceIdW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DeviceInstanceId: PWSTR,
        DeviceInstanceIdSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiDeleteDeviceInfo(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiEnumDeviceInfo(
        DeviceInfoSet: HDEVINFO,
        MemberIndex: DWORD,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiDestroyDeviceInfoList(
        DeviceInfoSet: HDEVINFO,
    ) -> BOOL;
    pub fn SetupDiEnumDeviceInterfaces(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        InterfaceClassGuid: *const GUID,
        MemberIndex: DWORD,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
    ) -> BOOL;
    pub fn SetupDiCreateDeviceInterfaceA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        InterfaceClassGuid: *const GUID,
        ReferenceString: PCSTR,
        CreationFlags: DWORD,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
    ) -> BOOL;
    pub fn SetupDiCreateDeviceInterfaceW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        InterfaceClassGuid: *const GUID,
        ReferenceString: PCWSTR,
        CreationFlags: DWORD,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
    ) -> BOOL;
}
pub const DIODI_NO_ADD: DWORD = 0x00000001;
extern "system" {
    pub fn SetupDiOpenDeviceInterfaceA(
        DeviceInfoSet: HDEVINFO,
        DevicePath: PCSTR,
        OpenFlags: DWORD,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
    ) -> BOOL;
    pub fn SetupDiOpenDeviceInterfaceW(
        DeviceInfoSet: HDEVINFO,
        DevicePath: PCWSTR,
        OpenFlags: DWORD,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInterfaceAlias(
        DeviceInfoSet: HDEVINFO,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
        AliasInterfaceClassGuid: *const GUID,
        AliasDeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
    ) -> BOOL;
    pub fn SetupDiDeleteDeviceInterfaceData(
        DeviceInfoSet: HDEVINFO,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
    ) -> BOOL;
    pub fn SetupDiRemoveDeviceInterface(
        DeviceInfoSet: HDEVINFO,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInterfaceDetailA(
        DeviceInfoSet: HDEVINFO,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
        DeviceInterfaceDetailData: PSP_DEVICE_INTERFACE_DETAIL_DATA_A,
        DeviceInterfaceDetailDataSize: DWORD,
        RequiredSize: PDWORD,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInterfaceDetailW(
        DeviceInfoSet: HDEVINFO,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
        DeviceInterfaceDetailData: PSP_DEVICE_INTERFACE_DETAIL_DATA_W,
        DeviceInterfaceDetailDataSize: DWORD,
        RequiredSize: PDWORD,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiInstallDeviceInterfaces(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiSetDeviceInterfaceDefault(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        Flags: DWORD,
        Reserved: PVOID,
    ) -> BOOL;
}
pub const SPRDI_FIND_DUPS: DWORD = 0x00000001;
extern "system" {
    pub fn SetupDiRegisterDeviceInfo(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        Flags: DWORD,
        CompareProc: PSP_DETSIG_CMPPROC,
        CompareContext: PVOID,
        DupDeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
}
pub const SPDIT_NODRIVER: DWORD = 0x00000000;
pub const SPDIT_CLASSDRIVER: DWORD = 0x00000001;
pub const SPDIT_COMPATDRIVER: DWORD = 0x00000002;
extern "system" {
    pub fn SetupDiBuildDriverInfoList(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DriverType: DWORD,
    ) -> BOOL;
    pub fn SetupDiCancelDriverInfoSearch(
        DeviceInfoSet: HDEVINFO,
    ) -> BOOL;
    pub fn SetupDiEnumDriverInfoA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DriverType: DWORD,
        MemberIndex: DWORD,
        DriverInfoData: PSP_DRVINFO_DATA_A,
    ) -> BOOL;
    pub fn SetupDiEnumDriverInfoW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DriverType: DWORD,
        MemberIndex: DWORD,
        DriverInfoData: PSP_DRVINFO_DATA_W,
    ) -> BOOL;
    pub fn SetupDiGetSelectedDriverA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_A,
    ) -> BOOL;
    pub fn SetupDiGetSelectedDriverW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_W,
    ) -> BOOL;
    pub fn SetupDiSetSelectedDriverA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_A,
    ) -> BOOL;
    pub fn SetupDiSetSelectedDriverW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_W,
    ) -> BOOL;
    pub fn SetupDiGetDriverInfoDetailA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_A,
        DriverInfoDetailData: PSP_DRVINFO_DETAIL_DATA_A,
        DriverInfoDetailDataSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetDriverInfoDetailW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_W,
        DriverInfoDetailData: PSP_DRVINFO_DETAIL_DATA_W,
        DriverInfoDetailDataSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiDestroyDriverInfoList(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DriverType: DWORD,
    ) -> BOOL;
}
pub const DIGCF_DEFAULT: DWORD = 0x00000001;
pub const DIGCF_PRESENT: DWORD = 0x00000002;
pub const DIGCF_ALLCLASSES: DWORD = 0x00000004;
pub const DIGCF_PROFILE: DWORD = 0x00000008;
pub const DIGCF_DEVICEINTERFACE: DWORD = 0x00000010;
extern "system" {
    pub fn SetupDiGetClassDevsA(
        ClassGuid: *const GUID,
        Enumerator: PCSTR,
        hwndParent: HWND,
        Flags: DWORD,
    ) -> HDEVINFO;
    pub fn SetupDiGetClassDevsW(
        ClassGuid: *const GUID,
        Enumerator: PCWSTR,
        hwndParent: HWND,
        Flags: DWORD,
    ) -> HDEVINFO;
    pub fn SetupDiGetClassDevsExA(
        ClassGuid: *const GUID,
        Enumerator: PCSTR,
        hwndParent: HWND,
        Flags: DWORD,
        DeviceInfoSet: HDEVINFO,
        MachineName: PCSTR,
        Reserved: PVOID,
    ) -> HDEVINFO;
    pub fn SetupDiGetClassDevsExW(
        ClassGuid: *const GUID,
        Enumerator: PCWSTR,
        hwndParent: HWND,
        Flags: DWORD,
        DeviceInfoSet: HDEVINFO,
        MachineName: PCWSTR,
        Reserved: PVOID,
    ) -> HDEVINFO;
    pub fn SetupDiGetINFClassA(
        InfName: PCSTR,
        ClassGuid: LPGUID,
        ClassName: PSTR,
        ClassNameSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetINFClassW(
        InfName: PCWSTR,
        ClassGuid: LPGUID,
        ClassName: PWSTR,
        ClassNameSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
}
pub const DIBCI_NOINSTALLCLASS: DWORD = 0x00000001;
pub const DIBCI_NODISPLAYCLASS: DWORD = 0x00000002;
extern "system" {
    pub fn SetupDiBuildClassInfoList(
        Flags: DWORD,
        ClassGuidList: LPGUID,
        ClassGuidListSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiBuildClassInfoListExA(
        Flags: DWORD,
        ClassGuidList: LPGUID,
        ClassGuidListSize: DWORD,
        RequiredSize: PDWORD,
        MachineName: PCSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiBuildClassInfoListExW(
        Flags: DWORD,
        ClassGuidList: LPGUID,
        ClassGuidListSize: DWORD,
        RequiredSize: PDWORD,
        MachineName: PCWSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetClassDescriptionA(
        ClassGuid: *const GUID,
        ClassDescription: PSTR,
        ClassDescriptionSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetClassDescriptionW(
        ClassGuid: *const GUID,
        ClassDescription: PWSTR,
        ClassDescriptionSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetClassDescriptionExA(
        ClassGuid: *const GUID,
        ClassDescription: PSTR,
        ClassDescriptionSize: DWORD,
        RequiredSize: PDWORD,
        MachineName: PCSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetClassDescriptionExW(
        ClassGuid: *const GUID,
        ClassDescription: PWSTR,
        ClassDescriptionSize: DWORD,
        RequiredSize: PDWORD,
        MachineName: PCWSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiCallClassInstaller(
        InstallFunction: DI_FUNCTION,
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiSelectDevice(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiSelectBestCompatDrv(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiInstallDevice(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiInstallDriverFiles(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiRegisterCoDeviceInstallers(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiRemoveDevice(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiUnremoveDevice(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiRestartDevices(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiChangeState(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiInstallClassA(
        hwndParent: HWND,
        InfFileName: PCSTR,
        Flags: DWORD,
        FileQueue: HSPFILEQ,
    ) -> BOOL;
    pub fn SetupDiInstallClassW(
        hwndParent: HWND,
        InfFileName: PCWSTR,
        Flags: DWORD,
        FileQueue: HSPFILEQ,
    ) -> BOOL;
    pub fn SetupDiInstallClassExA(
        hwndParent: HWND,
        InfFileName: PCSTR,
        Flags: DWORD,
        FileQueue: HSPFILEQ,
        InterfaceClassGuid: *const GUID,
        Reserved1: PVOID,
        Reserved2: PVOID,
    ) -> BOOL;
    pub fn SetupDiInstallClassExW(
        hwndParent: HWND,
        InfFileName: PCWSTR,
        Flags: DWORD,
        FileQueue: HSPFILEQ,
        InterfaceClassGuid: *const GUID,
        Reserved1: PVOID,
        Reserved2: PVOID,
    ) -> BOOL;
    pub fn SetupDiOpenClassRegKey(
        ClassGuid: *const GUID,
        samDesired: REGSAM,
    ) -> HKEY;
}
pub const DIOCR_INSTALLER: DWORD = 0x00000001;
pub const DIOCR_INTERFACE: DWORD = 0x00000002;
extern "system" {
    pub fn SetupDiOpenClassRegKeyExA(
        ClassGuid: *const GUID,
        samDesired: REGSAM,
        Flags: DWORD,
        MachineName: PCSTR,
        Reserved: PVOID,
    ) -> HKEY;
    pub fn SetupDiOpenClassRegKeyExW(
        ClassGuid: *const GUID,
        samDesired: REGSAM,
        Flags: DWORD,
        MachineName: PCWSTR,
        Reserved: PVOID,
    ) -> HKEY;
    pub fn SetupDiCreateDeviceInterfaceRegKeyA(
        DeviceInfoSet: HDEVINFO,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
        Reserved: DWORD,
        samDesired: REGSAM,
        InfHandle: HINF,
        InfSectionName: PCSTR,
    ) -> HKEY;
    pub fn SetupDiCreateDeviceInterfaceRegKeyW(
        DeviceInfoSet: HDEVINFO,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
        Reserved: DWORD,
        samDesired: REGSAM,
        InfHandle: HINF,
        InfSectionName: PCWSTR,
    ) -> HKEY;
    pub fn SetupDiOpenDeviceInterfaceRegKey(
        DeviceInfoSet: HDEVINFO,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
        Reserved: DWORD,
        samDesired: REGSAM,
    ) -> HKEY;
    pub fn SetupDiDeleteDeviceInterfaceRegKey(
        DeviceInfoSet: HDEVINFO,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
        Reserved: DWORD,
    ) -> BOOL;
}
pub const DIREG_DEV: DWORD = 0x00000001;
pub const DIREG_DRV: DWORD = 0x00000002;
pub const DIREG_BOTH: DWORD = 0x00000004;
extern "system" {
    pub fn SetupDiCreateDevRegKeyA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        Scope: DWORD,
        HwProfile: DWORD,
        KeyType: DWORD,
        InfHandle: HINF,
        InfSectionName: PCSTR,
    ) -> HKEY;
    pub fn SetupDiCreateDevRegKeyW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        Scope: DWORD,
        HwProfile: DWORD,
        KeyType: DWORD,
        InfHandle: HINF,
        InfSectionName: PCWSTR,
    ) -> HKEY;
    pub fn SetupDiOpenDevRegKey(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        Scope: DWORD,
        HwProfile: DWORD,
        KeyType: DWORD,
        samDesired: REGSAM,
    ) -> HKEY;
    pub fn SetupDiDeleteDevRegKey(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        Scope: DWORD,
        HwProfile: DWORD,
        KeyType: DWORD,
    ) -> BOOL;
    pub fn SetupDiGetHwProfileList(
        HwProfileList: PDWORD,
        HwProfileListSize: DWORD,
        RequiredSize: PDWORD,
        CurrentlyActiveIndex: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetHwProfileListExA(
        HwProfileList: PDWORD,
        HwProfileListSize: DWORD,
        RequiredSize: PDWORD,
        CurrentlyActiveIndex: PDWORD,
        MachineName: PCSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetHwProfileListExW(
        HwProfileList: PDWORD,
        HwProfileListSize: DWORD,
        RequiredSize: PDWORD,
        CurrentlyActiveIndex: PDWORD,
        MachineName: PCWSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetDevicePropertyKeys(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        PropertyKeyArray: *mut DEVPROPKEY,
        PropertyKeyCount: DWORD,
        RequiredPropertyKeyCount: PDWORD,
        Flags: DWORD,
    ) -> BOOL;
    pub fn SetupDiGetDevicePropertyW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        PropertyKey: *const DEVPROPKEY,
        PropertyType: *mut DEVPROPTYPE,
        PropertyBuffer: PBYTE,
        PropertyBufferSize: DWORD,
        RequiredSize: PDWORD,
        Flags: DWORD,
    ) -> BOOL;
    pub fn SetupDiSetDevicePropertyW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        PropertyKey: *const DEVPROPKEY,
        PropertyType: DEVPROPTYPE,
        PropertyBuffer: *const BYTE,
        PropertyBufferSize: DWORD,
        Flags: DWORD,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInterfacePropertyKeys(
        DeviceInfoSet: HDEVINFO,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
        PropertyKeyArray: *mut DEVPROPKEY,
        PropertyKeyCount: DWORD,
        RequiredPropertyKeyCount: PDWORD,
        Flags: DWORD,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInterfacePropertyW(
        DeviceInfoSet: HDEVINFO,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
        PropertyKey: *const DEVPROPKEY,
        PropertyType: *mut DEVPROPTYPE,
        PropertyBuffer: PBYTE,
        PropertyBufferSize: DWORD,
        RequiredSize: PDWORD,
        Flags: DWORD,
    ) -> BOOL;
    pub fn SetupDiSetDeviceInterfacePropertyW(
        DeviceInfoSet: HDEVINFO,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
        PropertyKey: *const DEVPROPKEY,
        PropertyType: DEVPROPTYPE,
        PropertyBuffer: *const BYTE,
        PropertyBufferSize: DWORD,
        Flags: DWORD,
    ) -> BOOL;
}
pub const DICLASSPROP_INSTALLER: DWORD = 0x00000001;
pub const DICLASSPROP_INTERFACE: DWORD = 0x00000002;
extern "system" {
    pub fn SetupDiGetClassPropertyKeys(
        ClassGuid: *const GUID,
        PropertyKeyArray: *mut DEVPROPKEY,
        PropertyKeyCount: DWORD,
        RequiredPropertyKeyCount: PDWORD,
        Flags: DWORD,
    ) -> BOOL;
    pub fn SetupDiGetClassPropertyKeysExW(
        ClassGuid: *const GUID,
        PropertyKeyArray: *mut DEVPROPKEY,
        PropertyKeyCount: DWORD,
        RequiredPropertyKeyCount: PDWORD,
        Flags: DWORD,
        MachineName: PCWSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetClassPropertyW(
        ClassGuid: *const GUID,
        PropertyKey: *const DEVPROPKEY,
        PropertyType: *mut DEVPROPTYPE,
        PropertyBuffer: PBYTE,
        PropertyBufferSize: DWORD,
        RequiredSize: PDWORD,
        Flags: DWORD,
    ) -> BOOL;
    pub fn SetupDiGetClassPropertyExW(
        ClassGuid: *const GUID,
        PropertyKey: *const DEVPROPKEY,
        PropertyType: *mut DEVPROPTYPE,
        PropertyBuffer: PBYTE,
        PropertyBufferSize: DWORD,
        RequiredSize: PDWORD,
        Flags: DWORD,
        MachineName: PCWSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiSetClassPropertyW(
        ClassGuid: *const GUID,
        PropertyKey: *const DEVPROPKEY,
        PropertyType: DEVPROPTYPE,
        PropertyBuffer: *const BYTE,
        PropertyBufferSize: DWORD,
        Flags: DWORD,
    ) -> BOOL;
    pub fn SetupDiSetClassPropertyExW(
        ClassGuid: *const GUID,
        PropertyKey: *const DEVPROPKEY,
        PropertyType: DEVPROPTYPE,
        PropertyBuffer: *const BYTE,
        PropertyBufferSize: DWORD,
        Flags: DWORD,
        MachineName: PCWSTR,
        Reserved: PVOID,
    ) -> BOOL;
}
pub const SPDRP_DEVICEDESC: DWORD = 0x00000000;
pub const SPDRP_HARDWAREID: DWORD = 0x00000001;
pub const SPDRP_COMPATIBLEIDS: DWORD = 0x00000002;
pub const SPDRP_UNUSED0: DWORD = 0x00000003;
pub const SPDRP_SERVICE: DWORD = 0x00000004;
pub const SPDRP_UNUSED1: DWORD = 0x00000005;
pub const SPDRP_UNUSED2: DWORD = 0x00000006;
pub const SPDRP_CLASS: DWORD = 0x00000007;
pub const SPDRP_CLASSGUID: DWORD = 0x00000008;
pub const SPDRP_DRIVER: DWORD = 0x00000009;
pub const SPDRP_CONFIGFLAGS: DWORD = 0x0000000A;
pub const SPDRP_MFG: DWORD = 0x0000000B;
pub const SPDRP_FRIENDLYNAME: DWORD = 0x0000000C;
pub const SPDRP_LOCATION_INFORMATION: DWORD = 0x0000000D;
pub const SPDRP_PHYSICAL_DEVICE_OBJECT_NAME: DWORD = 0x0000000E;
pub const SPDRP_CAPABILITIES: DWORD = 0x0000000F;
pub const SPDRP_UI_NUMBER: DWORD = 0x00000010;
pub const SPDRP_UPPERFILTERS: DWORD = 0x00000011;
pub const SPDRP_LOWERFILTERS: DWORD = 0x00000012;
pub const SPDRP_BUSTYPEGUID: DWORD = 0x00000013;
pub const SPDRP_LEGACYBUSTYPE: DWORD = 0x00000014;
pub const SPDRP_BUSNUMBER: DWORD = 0x00000015;
pub const SPDRP_ENUMERATOR_NAME: DWORD = 0x00000016;
pub const SPDRP_SECURITY: DWORD = 0x00000017;
pub const SPDRP_SECURITY_SDS: DWORD = 0x00000018;
pub const SPDRP_DEVTYPE: DWORD = 0x00000019;
pub const SPDRP_EXCLUSIVE: DWORD = 0x0000001A;
pub const SPDRP_CHARACTERISTICS: DWORD = 0x0000001B;
pub const SPDRP_ADDRESS: DWORD = 0x0000001C;
pub const SPDRP_UI_NUMBER_DESC_FORMAT: DWORD = 0x0000001D;
pub const SPDRP_DEVICE_POWER_DATA: DWORD = 0x0000001E;
pub const SPDRP_REMOVAL_POLICY: DWORD = 0x0000001F;
pub const SPDRP_REMOVAL_POLICY_HW_DEFAULT: DWORD = 0x00000020;
pub const SPDRP_REMOVAL_POLICY_OVERRIDE: DWORD = 0x00000021;
pub const SPDRP_INSTALL_STATE: DWORD = 0x00000022;
pub const SPDRP_LOCATION_PATHS: DWORD = 0x00000023;
pub const SPDRP_BASE_CONTAINERID: DWORD = 0x00000024;
pub const SPDRP_MAXIMUM_PROPERTY: DWORD = 0x00000025;
pub const SPCRP_UPPERFILTERS: DWORD = 0x00000011;
pub const SPCRP_LOWERFILTERS: DWORD = 0x00000012;
pub const SPCRP_SECURITY: DWORD = 0x00000017;
pub const SPCRP_SECURITY_SDS: DWORD = 0x00000018;
pub const SPCRP_DEVTYPE: DWORD = 0x00000019;
pub const SPCRP_EXCLUSIVE: DWORD = 0x0000001A;
pub const SPCRP_CHARACTERISTICS: DWORD = 0x0000001B;
pub const SPCRP_MAXIMUM_PROPERTY: DWORD = 0x0000001C;
extern "system" {
    pub fn SetupDiGetDeviceRegistryPropertyA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        Property: DWORD,
        PropertyRegDataType: PDWORD,
        PropertyBuffer: PBYTE,
        PropertyBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetDeviceRegistryPropertyW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        Property: DWORD,
        PropertyRegDataType: PDWORD,
        PropertyBuffer: PBYTE,
        PropertyBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetClassRegistryPropertyA(
        ClassGuid: *const GUID,
        Property: DWORD,
        PropertyRegDataType: PDWORD,
        PropertyBuffer: PBYTE,
        PropertyBufferSize: DWORD,
        RequiredSize: PDWORD,
        MachineName: PCSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetClassRegistryPropertyW(
        ClassGuid: *const GUID,
        Property: DWORD,
        PropertyRegDataType: PDWORD,
        PropertyBuffer: PBYTE,
        PropertyBufferSize: DWORD,
        RequiredSize: PDWORD,
        MachineName: PCWSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiSetDeviceRegistryPropertyA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        Property: DWORD,
        PropertyBuffer: *const BYTE,
        PropertyBufferSize: DWORD,
    ) -> BOOL;
    pub fn SetupDiSetDeviceRegistryPropertyW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        Property: DWORD,
        PropertyBuffer: *const BYTE,
        PropertyBufferSize: DWORD,
    ) -> BOOL;
    pub fn SetupDiSetClassRegistryPropertyA(
        ClassGuid: *const GUID,
        Property: DWORD,
        PropertyBuffer: *const BYTE,
        PropertyBufferSize: DWORD,
        MachineName: PCSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiSetClassRegistryPropertyW(
        ClassGuid: *const GUID,
        Property: DWORD,
        PropertyBuffer: *const BYTE,
        PropertyBufferSize: DWORD,
        MachineName: PCWSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInstallParamsA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DeviceInstallParams: PSP_DEVINSTALL_PARAMS_A,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInstallParamsW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DeviceInstallParams: PSP_DEVINSTALL_PARAMS_W,
    ) -> BOOL;
    pub fn SetupDiGetClassInstallParamsA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        ClassInstallParams: PSP_CLASSINSTALL_HEADER,
        ClassInstallParamsSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetClassInstallParamsW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        ClassInstallParams: PSP_CLASSINSTALL_HEADER,
        ClassInstallParamsSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiSetDeviceInstallParamsA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DeviceInstallParams: PSP_DEVINSTALL_PARAMS_A,
    ) -> BOOL;
    pub fn SetupDiSetDeviceInstallParamsW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DeviceInstallParams: PSP_DEVINSTALL_PARAMS_W,
    ) -> BOOL;
    pub fn SetupDiSetClassInstallParamsA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        ClassInstallParams: PSP_CLASSINSTALL_HEADER,
        ClassInstallParamsSize: DWORD,
    ) -> BOOL;
    pub fn SetupDiSetClassInstallParamsW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        ClassInstallParams: PSP_CLASSINSTALL_HEADER,
        ClassInstallParamsSize: DWORD,
    ) -> BOOL;
    pub fn SetupDiGetDriverInstallParamsA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_A,
        DriverInstallParams: PSP_DRVINSTALL_PARAMS,
    ) -> BOOL;
    pub fn SetupDiGetDriverInstallParamsW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_W,
        DriverInstallParams: PSP_DRVINSTALL_PARAMS,
    ) -> BOOL;
    pub fn SetupDiSetDriverInstallParamsA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_A,
        DriverInstallParams: PSP_DRVINSTALL_PARAMS,
    ) -> BOOL;
    pub fn SetupDiSetDriverInstallParamsW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_W,
        DriverInstallParams: PSP_DRVINSTALL_PARAMS,
    ) -> BOOL;
    pub fn SetupDiLoadClassIcon(
        ClassGuid: *const GUID,
        LargeIcon: *mut HICON,
        MiniIconIndex: PINT,
    ) -> BOOL;
    pub fn SetupDiLoadDeviceIcon(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        cxIcon: UINT,
        cyIcon: UINT,
        Flags: DWORD,
        hIcon: *mut HICON,
    ) -> BOOL;
}
pub const DMI_MASK: DWORD = 0x00000001;
pub const DMI_BKCOLOR: DWORD = 0x00000002;
pub const DMI_USERECT: DWORD = 0x00000004;
extern "system" {
    pub fn SetupDiDrawMiniIcon(
        hdc: HDC,
        rc: RECT,
        MiniIconIndex: INT,
        Flags: DWORD,
    ) -> INT;
    pub fn SetupDiGetClassBitmapIndex(
        ClassGuid: *const GUID,
        MiniIconIndex: PINT,
    ) -> BOOL;
    pub fn SetupDiGetClassImageList(
        ClassImageListData: PSP_CLASSIMAGELIST_DATA,
    ) -> BOOL;
    pub fn SetupDiGetClassImageListExA(
        ClassImageListData: PSP_CLASSIMAGELIST_DATA,
        MachineName: PCSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetClassImageListExW(
        ClassImageListData: PSP_CLASSIMAGELIST_DATA,
        MachineName: PCWSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetClassImageIndex(
        ClassImageListData: PSP_CLASSIMAGELIST_DATA,
        ClassGuid: *const GUID,
        ImageIndex: PINT,
    ) -> BOOL;
    pub fn SetupDiDestroyClassImageList(
        ClassImageListData: PSP_CLASSIMAGELIST_DATA,
    ) -> BOOL;
}
pub const DIGCDP_FLAG_BASIC: DWORD = 0x00000001;
pub const DIGCDP_FLAG_ADVANCED: DWORD = 0x00000002;
pub const DIGCDP_FLAG_REMOTE_BASIC: DWORD = 0x00000003;
pub const DIGCDP_FLAG_REMOTE_ADVANCED: DWORD = 0x00000004;
extern "system" {
    pub fn SetupDiGetClassDevPropertySheetsA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        PropertySheetHeader: LPPROPSHEETHEADERA,
        PropertySheetHeaderPageListSize: DWORD,
        RequiredSize: PDWORD,
        PropertySheetType: DWORD,
    ) -> BOOL;
    pub fn SetupDiGetClassDevPropertySheetsW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        PropertySheetHeader: LPPROPSHEETHEADERW,
        PropertySheetHeaderPageListSize: DWORD,
        RequiredSize: PDWORD,
        PropertySheetType: DWORD,
    ) -> BOOL;
}
pub const IDI_RESOURCEFIRST: c_int = 159;
pub const IDI_RESOURCE: c_int = 159;
pub const IDI_RESOURCELAST: c_int = 161;
pub const IDI_RESOURCEOVERLAYFIRST: c_int = 161;
pub const IDI_RESOURCEOVERLAYLAST: c_int = 161;
pub const IDI_CONFLICT: c_int = 161;
pub const IDI_CLASSICON_OVERLAYFIRST: c_int = 500;
pub const IDI_CLASSICON_OVERLAYLAST: c_int = 502;
pub const IDI_PROBLEM_OVL: c_int = 500;
pub const IDI_DISABLED_OVL: c_int = 501;
pub const IDI_FORCED_OVL: c_int = 502;
extern "system" {
    pub fn SetupDiAskForOEMDisk(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiSelectOEMDrv(
        hwndParent: HWND,
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiClassNameFromGuidA(
        ClassGuid: *const GUID,
        ClassName: PSTR,
        ClassNameSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiClassNameFromGuidW(
        ClassGuid: *const GUID,
        ClassName: PWSTR,
        ClassNameSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiClassNameFromGuidExA(
        ClassGuid: *const GUID,
        ClassName: PSTR,
        ClassNameSize: DWORD,
        RequiredSize: PDWORD,
        MachineName: PCSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiClassNameFromGuidExW(
        ClassGuid: *const GUID,
        ClassName: PWSTR,
        ClassNameSize: DWORD,
        RequiredSize: PDWORD,
        MachineName: PCWSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiClassGuidsFromNameA(
        ClassName: PCSTR,
        ClassGuidList: LPGUID,
        ClassGuidListSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiClassGuidsFromNameW(
        ClassName: PCWSTR,
        ClassGuidList: LPGUID,
        ClassGuidListSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiClassGuidsFromNameExA(
        ClassName: PCSTR,
        ClassGuidList: LPGUID,
        ClassGuidListSize: DWORD,
        RequiredSize: PDWORD,
        MachineName: PCSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiClassGuidsFromNameExW(
        ClassName: PCWSTR,
        ClassGuidList: LPGUID,
        ClassGuidListSize: DWORD,
        RequiredSize: PDWORD,
        MachineName: PCWSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetHwProfileFriendlyNameA(
        HwProfile: DWORD,
        FriendlyName: PSTR,
        FriendlyNameSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetHwProfileFriendlyNameW(
        HwProfile: DWORD,
        FriendlyName: PWSTR,
        FriendlyNameSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetHwProfileFriendlyNameExA(
        HwProfile: DWORD,
        FriendlyName: PSTR,
        FriendlyNameSize: DWORD,
        RequiredSize: PDWORD,
        MachineName: PCSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetHwProfileFriendlyNameExW(
        HwProfile: DWORD,
        FriendlyName: PWSTR,
        FriendlyNameSize: DWORD,
        RequiredSize: PDWORD,
        MachineName: PCWSTR,
        Reserved: PVOID,
    ) -> BOOL;
}
pub const SPWPT_SELECTDEVICE: DWORD = 0x00000001;
pub const SPWP_USE_DEVINFO_DATA: DWORD = 0x00000001;
extern "system" {
    pub fn SetupDiGetWizardPage(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        InstallWizardData: PSP_INSTALLWIZARD_DATA,
        PageType: DWORD,
        Flags: DWORD,
    ) -> HPROPSHEETPAGE;
    pub fn SetupDiGetSelectedDevice(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiSetSelectedDevice(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiGetActualModelsSectionA(
        Context: PINFCONTEXT,
        AlternatePlatformInfo: PSP_ALTPLATFORM_INFO,
        InfSectionWithExt: PSTR,
        InfSectionWithExtSize: DWORD,
        RequiredSize: PDWORD,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetActualModelsSectionW(
        Context: PINFCONTEXT,
        AlternatePlatformInfo: PSP_ALTPLATFORM_INFO,
        InfSectionWithExt: PWSTR,
        InfSectionWithExtSize: DWORD,
        RequiredSize: PDWORD,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetActualSectionToInstallA(
        InfHandle: HINF,
        InfSectionName: PCSTR,
        InfSectionWithExt: PSTR,
        InfSectionWithExtSize: DWORD,
        RequiredSize: PDWORD,
        Extension: *mut PSTR,
    ) -> BOOL;
    pub fn SetupDiGetActualSectionToInstallW(
        InfHandle: HINF,
        InfSectionName: PCWSTR,
        InfSectionWithExt: PWSTR,
        InfSectionWithExtSize: DWORD,
        RequiredSize: PDWORD,
        Extension: *mut PWSTR,
    ) -> BOOL;
    pub fn SetupDiGetActualSectionToInstallExA(
        InfHandle: HINF,
        InfSectionName: PCSTR,
        AlternatePlatformInfo: PSP_ALTPLATFORM_INFO,
        InfSectionWithExt: PSTR,
        InfSectionWithExtSize: DWORD,
        RequiredSize: PDWORD,
        Extension: *mut PSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetActualSectionToInstallExW(
        InfHandle: HINF,
        InfSectionName: PCWSTR,
        AlternatePlatformInfo: PSP_ALTPLATFORM_INFO,
        InfSectionWithExt: PWSTR,
        InfSectionWithExtSize: DWORD,
        RequiredSize: PDWORD,
        Extension: *mut PWSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupEnumInfSectionsA(
        InfHandle: HINF,
        Index: UINT,
        Buffer: PSTR,
        Size: UINT,
        SizeNeeded: *mut UINT,
    ) -> BOOL;
    pub fn SetupEnumInfSectionsW(
        InfHandle: HINF,
        Index: UINT,
        Buffer: PWSTR,
        Size: UINT,
        SizeNeeded: *mut UINT,
    ) -> BOOL;
}
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_INF_SIGNER_INFO_V1_A {
    cbSize: DWORD,
    CatalogFile: [CHAR; MAX_PATH],
    DigitalSigner: [CHAR; MAX_PATH],
    DigitalSignerVersion: [CHAR; MAX_PATH],
}}
pub type PSP_INF_SIGNER_INFO_V1_A = *mut SP_INF_SIGNER_INFO_V1_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_INF_SIGNER_INFO_V1_W {
    cbSize: DWORD,
    CatalogFile: [WCHAR; MAX_PATH],
    DigitalSigner: [WCHAR; MAX_PATH],
    DigitalSignerVersion: [WCHAR; MAX_PATH],
}}
pub type PSP_INF_SIGNER_INFO_V1_W = *mut SP_INF_SIGNER_INFO_V1_W;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_INF_SIGNER_INFO_V2_A {
    cbSize: DWORD,
    CatalogFile: [CHAR; MAX_PATH],
    DigitalSigner: [CHAR; MAX_PATH],
    DigitalSignerVersion: [CHAR; MAX_PATH],
    SignerScore: DWORD,
}}
pub type PSP_INF_SIGNER_INFO_V2_A = *mut SP_INF_SIGNER_INFO_V2_A;
STRUCT!{#[cfg_attr(any(target_arch = "x86", target_arch = "arm"), repr(packed))] struct SP_INF_SIGNER_INFO_V2_W {
    cbSize: DWORD,
    CatalogFile: [WCHAR; MAX_PATH],
    DigitalSigner: [WCHAR; MAX_PATH],
    DigitalSignerVersion: [WCHAR; MAX_PATH],
    SignerScore: DWORD,
}}
pub type PSP_INF_SIGNER_INFO_V2_W = *mut SP_INF_SIGNER_INFO_V2_W;
pub const SIGNERSCORE_UNKNOWN: DWORD = 0xFF000000;
pub const SIGNERSCORE_W9X_SUSPECT: DWORD = 0xC0000000;
pub const SIGNERSCORE_UNSIGNED: DWORD = 0x80000000;
pub const SIGNERSCORE_AUTHENTICODE: DWORD = 0x0F000000;
pub const SIGNERSCORE_WHQL: DWORD = 0x0D000005;
pub const SIGNERSCORE_UNCLASSIFIED: DWORD = 0x0D000004;
pub const SIGNERSCORE_INBOX: DWORD = 0x0D000003;
pub const SIGNERSCORE_LOGO_STANDARD: DWORD = 0x0D000002;
pub const SIGNERSCORE_LOGO_PREMIUM: DWORD = 0x0D000001;
pub const SIGNERSCORE_MASK: DWORD = 0xFF000000;
pub const SIGNERSCORE_SIGNED_MASK: DWORD = 0xF0000000;
pub type SP_INF_SIGNER_INFO_A = SP_INF_SIGNER_INFO_V2_A;
pub type PSP_INF_SIGNER_INFO_A = PSP_INF_SIGNER_INFO_V2_A;
pub type SP_INF_SIGNER_INFO_W = SP_INF_SIGNER_INFO_V2_W;
pub type PSP_INF_SIGNER_INFO_W = PSP_INF_SIGNER_INFO_V2_W;
extern "system" {
    pub fn SetupVerifyInfFileA(
        InfName: PCSTR,
        AltPlatformInfo: PSP_ALTPLATFORM_INFO,
        InfSignerInfo: PSP_INF_SIGNER_INFO_A,
    ) -> BOOL;
    pub fn SetupVerifyInfFileW(
        InfName: PCWSTR,
        AltPlatformInfo: PSP_ALTPLATFORM_INFO,
        InfSignerInfo: PSP_INF_SIGNER_INFO_W,
    ) -> BOOL;
}
pub const DICUSTOMDEVPROP_MERGE_MULTISZ: DWORD = 0x00000001;
extern "system" {
    pub fn SetupDiGetCustomDevicePropertyA(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        CustomPropertyName: PCSTR,
        Flags: DWORD,
        PropertyRegDataType: PDWORD,
        PropertyBuffer: PBYTE,
        PropertyBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetCustomDevicePropertyW(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        CustomPropertyName: PCWSTR,
        Flags: DWORD,
        PropertyRegDataType: PDWORD,
        PropertyBuffer: PBYTE,
        PropertyBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
}
pub const SCWMI_CLOBBER_SECURITY: DWORD = 0x00000001;
extern "system" {
    pub fn SetupConfigureWmiFromInfSectionA(
        InfHandle: HINF,
        SectionName: PCSTR,
        Flags: DWORD,
    ) -> BOOL;
    pub fn SetupConfigureWmiFromInfSectionW(
        InfHandle: HINF,
        SectionName: PCWSTR,
        Flags: DWORD,
    ) -> BOOL;
}
