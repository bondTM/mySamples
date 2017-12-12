use winapi::minwindef::*;

pub fn FILETIME_New() -> FILETIME{

    FILETIME{
        dwLowDateTime:0,
        dwHighDateTime:0
    }
}