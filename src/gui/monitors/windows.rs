use std::{
    io,
    iter::FusedIterator,
    mem::size_of,
    ptr::{null, null_mut},
};
use winapi::{
    shared::{
        guiddef::GUID,
        minwindef::HKEY,
        winerror::{ERROR_NO_MORE_ITEMS, ERROR_SUCCESS},
    },
    um::{
        handleapi::INVALID_HANDLE_VALUE,
        setupapi::{
            SetupDiEnumDeviceInfo, SetupDiGetClassDevsExW, SetupDiOpenDevRegKey, DICS_FLAG_GLOBAL,
            DIGCF_PRESENT, DIREG_DEV, HDEVINFO, SP_DEVINFO_DATA,
        },
        winnt::KEY_READ,
        winreg::{RegGetValueW, RRF_RT_REG_BINARY},
    },
    DEFINE_GUID,
};

DEFINE_GUID!(
    GUID_CLASS_MONITOR,
    0x4d36e96e,
    0xe325,
    0x11ce,
    0xbf,
    0xc1,
    0x08,
    0x00,
    0x2b,
    0xe1,
    0x03,
    0x18
);
/// The string `EDID` in UTF-16, null-terminated with native endianness.
static EDID_UTF16_LITERAL: [u16; 5] = [0x45, 0x44, 0x49, 0x44, 0x00];

/// Iterator over all EDIDs for all monitors in the system.
///
/// Won't ignore EDID querying errors, so monitors without EDID information will mix with regular errors.
pub struct MonitorEdids(MonitorRegKeys);
impl MonitorEdids {
    pub fn new() -> io::Result<Self> {
        Ok(Self(MonitorRegKeys::new()?))
    }
}
impl Iterator for MonitorEdids {
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let key = match self.0.next()? {
            Ok(k) => k,
            Err(e) => return Some(Err(e)),
        };
        Some(get_edid_from_key(key))
    }
}

/// Returns the `HDEVINFO` for `GUID_CLASS_MONITOR`.
fn get_monitor_info_set() -> io::Result<HDEVINFO> {
    let result = unsafe {
        SetupDiGetClassDevsExW(
            &GUID_CLASS_MONITOR as *const _,
            null(),
            null_mut(),
            DIGCF_PRESENT,
            null_mut(),
            null_mut(),
            null_mut(),
        )
    };
    if result != INVALID_HANDLE_VALUE {
        Ok(result)
    } else {
        Err(io::Error::last_os_error())
    }
}

fn get_edid_from_key(key: HKEY) -> io::Result<Vec<u8>> {
    let get_value = |data: Option<&mut [u8]>, data_length: &mut u32| -> io::Result<()> {
        let data = data.map_or(null_mut(), |d| d.as_mut_ptr());
        let success = unsafe {
            RegGetValueW(
                key,
                null(),
                &EDID_UTF16_LITERAL as *const _,
                RRF_RT_REG_BINARY,
                null_mut(),
                data as *mut _,
                data_length as *mut _,
            )
        };
        if success == ERROR_SUCCESS as _ {
            Ok(())
        } else {
            Err(io::Error::from_raw_os_error(success as _))
        }
    };
    let mut edid_size = 0;
    get_value(None, &mut edid_size)?;
    let mut buffer = vec![0; edid_size as _];
    get_value(Some(&mut buffer[..]), &mut edid_size)?;
    Ok(buffer)
}

struct MonitorRegKeys(DevNodes);
impl MonitorRegKeys {
    fn new() -> io::Result<Self> {
        Ok(Self(DevNodes::new(get_monitor_info_set()?)))
    }
}
impl Iterator for MonitorRegKeys {
    type Item = io::Result<HKEY>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut monitor = match self.0.next()? {
            Ok(m) => m,
            Err(e) => return Some(Err(e)),
        };
        let handle = unsafe {
            SetupDiOpenDevRegKey(
                self.0.info_set,
                &mut monitor as *mut _,
                DICS_FLAG_GLOBAL,
                0,
                DIREG_DEV,
                KEY_READ,
            )
        };
        if handle != INVALID_HANDLE_VALUE as _ {
            Some(Ok(handle))
        } else {
            Some(Err(io::Error::last_os_error()))
        }
    }
}
impl FusedIterator for MonitorRegKeys {}

/// Iterator over SetupAPI devnodes, i.e. wrapper around `SetupDiEnumDeviceInfo`.
struct DevNodes {
    info_set: HDEVINFO,
    index: u32,
}
impl DevNodes {
    fn new(info_set: HDEVINFO) -> Self {
        Self { info_set, index: 0 }
    }
}
impl Iterator for DevNodes {
    type Item = io::Result<SP_DEVINFO_DATA>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut device = SP_DEVINFO_DATA {
            cbSize: size_of::<SP_DEVINFO_DATA>() as _,
            ClassGuid: GUID {
                Data1: 0,
                Data2: 0,
                Data3: 0,
                Data4: [0; 8],
            },
            DevInst: 0,
            Reserved: 0,
        };
        let success =
            unsafe { SetupDiEnumDeviceInfo(self.info_set, self.index, &mut device as *mut _) };
        if success != 0 {
            self.index += 1;
            Some(Ok(device))
        } else {
            let error = io::Error::last_os_error();
            if error.raw_os_error() == Some(ERROR_NO_MORE_ITEMS as _) {
                None
            } else {
                Some(Err(error))
            }
        }
    }
}
impl FusedIterator for DevNodes {}
