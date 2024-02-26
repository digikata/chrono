// Bindings generated by `windows-bindgen` 0.53.0

#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, clippy::all)]
::windows_targets::link!("kernel32.dll" "system" fn GetTimeZoneInformationForYear(wyear : u16, pdtzi : *const DYNAMIC_TIME_ZONE_INFORMATION, ptzi : *mut TIME_ZONE_INFORMATION) -> BOOL);
::windows_targets::link!("kernel32.dll" "system" fn SystemTimeToFileTime(lpsystemtime : *const SYSTEMTIME, lpfiletime : *mut FILETIME) -> BOOL);
::windows_targets::link!("kernel32.dll" "system" fn SystemTimeToTzSpecificLocalTime(lptimezoneinformation : *const TIME_ZONE_INFORMATION, lpuniversaltime : *const SYSTEMTIME, lplocaltime : *mut SYSTEMTIME) -> BOOL);
::windows_targets::link!("kernel32.dll" "system" fn TzSpecificLocalTimeToSystemTime(lptimezoneinformation : *const TIME_ZONE_INFORMATION, lplocaltime : *const SYSTEMTIME, lpuniversaltime : *mut SYSTEMTIME) -> BOOL);
pub type BOOL = i32;
pub type BOOLEAN = u8;
#[repr(C)]
pub struct DYNAMIC_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: SYSTEMTIME,
    pub DaylightBias: i32,
    pub TimeZoneKeyName: [u16; 128],
    pub DynamicDaylightTimeDisabled: BOOLEAN,
}
impl ::core::marker::Copy for DYNAMIC_TIME_ZONE_INFORMATION {}
impl ::core::clone::Clone for DYNAMIC_TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
impl ::core::marker::Copy for FILETIME {}
impl ::core::clone::Clone for FILETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
impl ::core::marker::Copy for SYSTEMTIME {}
impl ::core::clone::Clone for SYSTEMTIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: SYSTEMTIME,
    pub DaylightBias: i32,
}
impl ::core::marker::Copy for TIME_ZONE_INFORMATION {}
impl ::core::clone::Clone for TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
