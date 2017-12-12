extern crate winapi;

use winapi::winnetwk::*;
use winapi::minwinbase::*;
use winapi::minwindef::*;
use std::ptr::null_mut;


pub fn netresourcea_new() -> NETRESOURCEA{

    NETRESOURCEA{
        dwScope: 0,
        dwType: RESOURCETYPE_ANY,
        dwDisplayType: 0,
        dwUsage: 0,
        lpLocalName: null_mut(),
        lpRemoteName: null_mut(),
        lpComment: null_mut(),
        lpProvider: null_mut()
    }
}

pub fn win32_find_dataa_new() -> WIN32_FIND_DATAA{

    WIN32_FIND_DATAA{
        dwFileAttributes: 0,
        ftCreationTime: filetime_new(),
        ftLastAccessTime: filetime_new(),
        ftLastWriteTime: filetime_new(),
        nFileSizeHigh: 0,
        nFileSizeLow: 0,
        dwReserved0: 0,
        dwReserved1: 0,
        cFileName: [0; MAX_PATH],
        cAlternateFileName: [0; 14]
    }
}

pub fn filetime_new() -> FILETIME{

    FILETIME{
        dwLowDateTime:0,
        dwHighDateTime:0
    }
}