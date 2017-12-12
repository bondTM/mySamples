use winapi::winnetwk::*;
use std::ptr::null_mut;

pub fn NETRESOURCEA_New() -> NETRESOURCEA{

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