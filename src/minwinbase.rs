use winapi::minwinbase::*;
use winapi::minwindef::*;
use minwindef::*;

pub fn WIN32_FIND_DATAA_New() -> WIN32_FIND_DATAA{

    WIN32_FIND_DATAA{
        dwFileAttributes: 0,
        ftCreationTime: FILETIME_New(),
        ftLastAccessTime: FILETIME_New(),
        ftLastWriteTime: FILETIME_New(),
        nFileSizeHigh: 0,
        nFileSizeLow: 0,
        dwReserved0: 0,
        dwReserved1: 0,
        cFileName: [0; MAX_PATH],
        cAlternateFileName: [0; 14]
    }
}

pub fn WIN32_FIND_DATAW_New() -> WIN32_FIND_DATAW{
    
    WIN32_FIND_DATAW{
        dwFileAttributes: 0,
        ftCreationTime: FILETIME_New(),
        ftLastAccessTime: FILETIME_New(),
        ftLastWriteTime: FILETIME_New(),
        nFileSizeHigh: 0,
        nFileSizeLow: 0,
        dwReserved0: 0,
        dwReserved1: 0,
        cFileName: [0; MAX_PATH],
        cAlternateFileName: [0; 14]
    }
}