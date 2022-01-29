extern "C" {
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    /* Ppmd8.h -- PPMdI codec
    2017-04-03 : Igor Pavlov : Public domain
    This code is based on:
      PPMd var.I (2002): Dmitry Shkarin : Public domain
      Carryless rangecoder (1999): Dmitry Subbotin : Public domain */
    /* The BUG in Shkarin's code for FREEZE mode was fixed, but that fixed
    code is not compatible with original code for some files compressed
    in FREEZE mode. So we disable FREEZE mode support. */
    /* must be 32-bit at least */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharWriter {
    pub Write: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_uchar) -> ()>,
    pub fp: *mut libc::FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharReader {
    pub Read: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_uchar>,
    pub fp: *mut libc::FILE,
    pub eof: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct header {
    pub magic: libc::c_uint,
    pub attr: libc::c_uint,
    pub info: libc::c_ushort,
    pub fnlen: libc::c_ushort,
    pub date: libc::c_ushort,
    pub time: libc::c_ushort,
}

pub unsafe extern "C" fn Write(mut p: *mut libc::c_void, mut b: libc::c_uchar) {
    let mut cw: *mut CharWriter = p as *mut CharWriter;
    libc::fputc(b as libc::c_int, (*cw).fp as *mut libc::FILE);
}
pub unsafe extern "C" fn Read(mut p: *mut libc::c_void) -> libc::c_uchar {
    let mut cr: *mut CharReader = p as *mut CharReader;
    if (*cr).eof {
        return 0 as libc::c_int as libc::c_uchar;
    }
    let mut c: libc::c_int = libc::fgetc((*cr).fp as *mut libc::FILE);
    if c == -(1 as libc::c_int) {
        (*cr).eof = 1 as libc::c_int != 0;
        return 0 as libc::c_int as libc::c_uchar;
    }
    return c as libc::c_uchar;
}
pub static mut opt_mem: libc::c_int = 8 as libc::c_int;
pub static mut opt_order: libc::c_int = 6 as libc::c_int;
pub static mut opt_restore: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut hdr: header = {
    let mut init = header {
        magic: 0x84acaf8f as libc::c_uint,
        attr: 0x80 as libc::c_int as libc::c_uint,
        info: 0 as libc::c_int as libc::c_ushort,
        fnlen: 1 as libc::c_int as libc::c_ushort,
        date: 0 as libc::c_int as libc::c_ushort,
        time: 0 as libc::c_int as libc::c_ushort,
    };
    init
};

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut current_block: u64;
    static mut longopts: [option; 9] = [
        {
            let mut init = option {
                name: b"decompress\x00" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'd' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"uncompress\x00" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'd' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"keep\x00" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'k' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"stdout\x00" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"to-stdout\x00" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"memory\x00" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'm' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"order\x00" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'o' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\x00" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ];
    let mut opt_d: bool = 0 as libc::c_int != 0;
    let mut opt_k: bool = 0 as libc::c_int != 0;
    let mut opt_c: bool = 0 as libc::c_int != 0;
    let mut c: libc::c_int = 0;
    loop {
        /*c = getopt_long(
            argc,
            argv,
            b"dkcm:o:36h\x00" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );*/
        if !(c != -(1 as libc::c_int)) {
            current_block = 7175849428784450219;
            break;
        }
        match c {
            100 => opt_d = 1 as libc::c_int != 0,
            107 => opt_k = 1 as libc::c_int != 0,
            99 => opt_c = 1 as libc::c_int != 0,
            //109 => opt_mem = atoi(optarg),
            //111 => opt_order = atoi(optarg),
            51 => {
                opt_mem = 1 as libc::c_int;
                opt_order = 5 as libc::c_int
            }
            54 => {
                opt_mem = 8 as libc::c_int;
                opt_order = 6 as libc::c_int
            }
            _ => {
                current_block = 4840636708823783151;
                break;
            }
        }
    }
    match current_block {
        7175849428784450219 => {
            argc -= optind;
            argv = argv.offset(optind as isize);
            if argc > 1 as libc::c_int {
            } else {
                return 0;
            }
        }
        _ => {}
    }

    return 1 as libc::c_int;
}
/* ex: set ts=8 sts=4 sw=4 noet: */
