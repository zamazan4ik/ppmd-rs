/*extern "C" {

    static mut optind: i32;

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
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}

unsafe fn main_0(mut argc: i32, mut argv: *mut *mut libc::c_char) -> i32 {
    let mut current_block: u64;
    static mut longopts: [option; 9] = [
        {
            let mut init = option {
                name: b"decompress\x00" as *const u8 as *const libc::c_char,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'd' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"uncompress\x00" as *const u8 as *const libc::c_char,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'd' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"keep\x00" as *const u8 as *const libc::c_char,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'k' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"stdout\x00" as *const u8 as *const libc::c_char,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"to-stdout\x00" as *const u8 as *const libc::c_char,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"memory\x00" as *const u8 as *const libc::c_char,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'm' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"order\x00" as *const u8 as *const libc::c_char,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'o' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\x00" as *const u8 as *const libc::c_char,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 0 as i32,
            };
            init
        },
    ];
    let mut opt_d: bool = 0 as i32 != 0;
    let mut opt_k: bool = 0 as i32 != 0;
    let mut opt_c: bool = 0 as i32 != 0;
    let mut c: i32 = 0;
    loop {
        /*c = getopt_long(
            argc,
            argv,
            b"dkcm:o:36h\x00" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut i32,
        );*/
        if !(c != -(1 as i32)) {
            current_block = 7175849428784450219;
            break;
        }
        match c {
            100 => opt_d = 1 as i32 != 0,
            107 => opt_k = 1 as i32 != 0,
            99 => opt_c = 1 as i32 != 0,
            //109 => opt_mem = atoi(optarg),
            //111 => opt_order = atoi(optarg),
            51 => {
                //opt_mem = 1 as i32;
                //opt_order = 5 as i32
            }
            54 => {
                //opt_mem = 8 as i32;
                //opt_order = 6 as i32
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
            if argc > 1 as i32 {
            } else {
                return 0;
            }
        }
        _ => {}
    }

    return 1 as i32;
}
/* ex: set ts=8 sts=4 sw=4 noet: */
*/
