#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
//#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, main, register_tool)]
extern "C" {
    #[no_mangle]
    pub static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    pub static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn __uflow(_: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    fn __overflow(_: *mut _IO_FILE, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn _IO_putc(__c: libc::c_int, __fp: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    pub fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    #[no_mangle]
    pub fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn strtol(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_long;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    /* Ppmd8.h -- PPMdI codec
    2017-04-03 : Igor Pavlov : Public domain
    This code is based on:
      PPMd var.I (2002): Dmitry Shkarin : Public domain
      Carryless rangecoder (1999): Dmitry Subbotin : Public domain */
    /* The BUG in Shkarin's code for FREEZE mode was fixed, but that fixed
    code is not compatible with original code for some files compressed
    in FREEZE mode. So we disable FREEZE mode support. */
    /* must be 32-bit at least */
    /* Range Coder */
    #[no_mangle]
    fn Ppmd8_Alloc(p: *mut CPpmd8, size: UInt32, alloc: ISzAllocPtr) -> Bool;
    #[no_mangle]
    fn Ppmd8_Construct(p: *mut CPpmd8);
    #[no_mangle]
    fn Ppmd8_Init(p: *mut CPpmd8, maxOrder: libc::c_uint, restoreMethod: libc::c_uint);
    /* ---------- Decode ---------- */
    #[no_mangle]
    fn Ppmd8_RangeDec_Init(p: *mut CPpmd8) -> Bool;
    #[no_mangle]
    fn Ppmd8_DecodeSymbol(p: *mut CPpmd8) -> libc::c_int;
    /* returns: -1 as EndMarker, -2 as DataError */
    /* ---------- Encode ---------- */
    #[no_mangle]
    fn Ppmd8_RangeEnc_FlushData(p: *mut CPpmd8);
    #[no_mangle]
    fn Ppmd8_EncodeSymbol(p: *mut CPpmd8, symbol: libc::c_int);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type Byte = libc::c_uchar;
pub type UInt16 = libc::c_ushort;
pub type Int32 = libc::c_int;
pub type UInt32 = libc::c_uint;
pub type Bool = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IByteIn {
    pub Read: Option<unsafe extern "C" fn(_: *const IByteIn) -> Byte>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IByteOut {
    pub Write: Option<unsafe extern "C" fn(_: *const IByteOut, _: Byte) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ISzAlloc {
    pub Alloc: Option<unsafe extern "C" fn(_: ISzAllocPtr, _: size_t) -> *mut libc::c_void>,
    pub Free: Option<unsafe extern "C" fn(_: ISzAllocPtr, _: *mut libc::c_void) -> ()>,
}
pub type ISzAllocPtr = *const ISzAlloc;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct CPpmd_See {
    pub Summ: UInt16,
    pub Shift: Byte,
    pub Count: Byte,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct CPpmd_State {
    pub Symbol: Byte,
    pub Freq: Byte,
    pub SuccessorLow: UInt16,
    pub SuccessorHigh: UInt16,
}
pub type CPpmd_State_Ref = UInt32;
pub type CPpmd_Void_Ref = UInt32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd8_Context_ {
    pub NumStats: Byte,
    pub Flags: Byte,
    pub SummFreq: UInt16,
    pub Stats: CPpmd_State_Ref,
    pub Suffix: CPpmd8_Context_Ref,
}
pub type CPpmd8_Context_Ref = UInt32;
pub type CPpmd8_Context = CPpmd8_Context_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd8 {
    pub MinContext: *mut CPpmd8_Context,
    pub MaxContext: *mut CPpmd8_Context,
    pub FoundState: *mut CPpmd_State,
    pub OrderFall: libc::c_uint,
    pub InitEsc: libc::c_uint,
    pub PrevSuccess: libc::c_uint,
    pub MaxOrder: libc::c_uint,
    pub RunLength: Int32,
    pub InitRL: Int32,
    pub Size: UInt32,
    pub GlueCount: UInt32,
    pub Base: *mut Byte,
    pub LoUnit: *mut Byte,
    pub HiUnit: *mut Byte,
    pub Text: *mut Byte,
    pub UnitsStart: *mut Byte,
    pub AlignOffset: UInt32,
    pub RestoreMethod: libc::c_uint,
    pub Range: UInt32,
    pub Code: UInt32,
    pub Low: UInt32,
    pub Stream: C2RustUnnamed,
    pub Indx2Units: [Byte; 38],
    pub Units2Indx: [Byte; 128],
    pub FreeList: [CPpmd_Void_Ref; 38],
    pub Stamps: [UInt32; 38],
    pub NS2BSIndx: [Byte; 256],
    pub NS2Indx: [Byte; 260],
    pub DummySee: CPpmd_See,
    pub See: [[CPpmd_See; 32]; 24],
    pub BinSumm: [[UInt16; 64]; 25],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub In: *mut IByteIn,
    pub Out: *mut IByteOut,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharWriter {
    pub Write: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: Byte) -> ()>,
    pub fp: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharReader {
    pub Read: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> Byte>,
    pub fp: *mut FILE,
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
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int as libc::c_long != 0 {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
        *(fresh0 as *mut libc::c_uchar) as libc::c_int
    };
}
#[inline]
pub unsafe extern "C" fn getchar_unlocked() -> libc::c_int {
    return if ((*stdin)._IO_read_ptr >= (*stdin)._IO_read_end) as libc::c_int as libc::c_long != 0 {
        __uflow(stdin)
    } else {
        let fresh1 = (*stdin)._IO_read_ptr;
        (*stdin)._IO_read_ptr = (*stdin)._IO_read_ptr.offset(1);
        *(fresh1 as *mut libc::c_uchar) as libc::c_int
    };
}
#[inline]
pub unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return _IO_putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn putc_unlocked(mut __c: libc::c_int, mut __stream: *mut FILE) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long
        != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh2 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = (*__stream)._IO_write_ptr.offset(1);
        *fresh2 = __c as libc::c_char;
        *fresh2 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}

pub unsafe extern "C" fn Write(mut p: *mut libc::c_void, mut b: Byte) {
    let mut cw: *mut CharWriter = p as *mut CharWriter;
    putc_unlocked(b as libc::c_int, (*cw).fp);
}
pub unsafe extern "C" fn Read(mut p: *mut libc::c_void) -> Byte {
    let mut cr: *mut CharReader = p as *mut CharReader;
    if (*cr).eof {
        return 0 as libc::c_int as Byte;
    }
    let mut c: libc::c_int = getc_unlocked((*cr).fp);
    if c == -(1 as libc::c_int) {
        (*cr).eof = 1 as libc::c_int != 0;
        return 0 as libc::c_int as Byte;
    }
    return c as Byte;
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
unsafe extern "C" fn compress() -> libc::c_int {
    hdr.info = (opt_order - 1 as libc::c_int
        | (opt_mem - 1 as libc::c_int) << 4 as libc::c_int
        | ('I' as i32 - 'A' as i32) << 12 as libc::c_int) as libc::c_ushort;
    fwrite(
        &mut hdr as *mut header as *const libc::c_void,
        ::std::mem::size_of::<header>() as libc::c_ulong,
        1 as libc::c_int as size_t,
        stdout,
    );
    putchar('a' as i32);
    let mut cw: CharWriter = {
        let mut init = CharWriter {
            Write: Some(Write as unsafe extern "C" fn(_: *mut libc::c_void, _: Byte) -> ()),
            fp: stdout,
        };
        init
    };
    let mut ppmd: CPpmd8 = {
        let mut init = CPpmd8 {
            MinContext: 0 as *mut CPpmd8_Context,
            MaxContext: 0 as *mut CPpmd8_Context,
            FoundState: 0 as *mut CPpmd_State,
            OrderFall: 0,
            InitEsc: 0,
            PrevSuccess: 0,
            MaxOrder: 0,
            RunLength: 0,
            InitRL: 0,
            Size: 0,
            GlueCount: 0,
            Base: 0 as *mut Byte,
            LoUnit: 0 as *mut Byte,
            HiUnit: 0 as *mut Byte,
            Text: 0 as *mut Byte,
            UnitsStart: 0 as *mut Byte,
            AlignOffset: 0,
            RestoreMethod: 0,
            Range: 0,
            Code: 0,
            Low: 0,
            Stream: C2RustUnnamed {
                Out: &mut cw as *mut CharWriter as *mut IByteOut,
            },
            Indx2Units: [0; 38],
            Units2Indx: [0; 128],
            FreeList: [0; 38],
            Stamps: [0; 38],
            NS2BSIndx: [0; 256],
            NS2Indx: [0; 260],
            DummySee: CPpmd_See {
                Summ: 0,
                Shift: 0,
                Count: 0,
            },
            See: [[CPpmd_See {
                Summ: 0,
                Shift: 0,
                Count: 0,
            }; 32]; 24],
            BinSumm: [[0; 64]; 25],
        };
        init
    };
    Ppmd8_Construct(&mut ppmd);
    /*Ppmd8_Alloc(
        &mut ppmd,
        (opt_mem << 20 as libc::c_int) as UInt32,
        &mut ialloc as *mut ISzAlloc as ISzAllocPtr,
    );*/
    ppmd.Low = 0 as libc::c_int as UInt32;
    ppmd.Range = 0xffffffff as libc::c_uint;
    Ppmd8_Init(
        &mut ppmd,
        opt_order as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
    );
    let mut buf: [libc::c_uchar; 8192] = [0; 8192];
    let mut n: size_t = 0;
    loop {
        n = fread(
            buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<[libc::c_uchar; 8192]>() as libc::c_ulong,
            stdin,
        );
        if !(n != 0) {
            break;
        }
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < n {
            Ppmd8_EncodeSymbol(&mut ppmd, buf[i as usize] as libc::c_int);
            i = i.wrapping_add(1)
        }
    }
    Ppmd8_EncodeSymbol(&mut ppmd, -(1 as libc::c_int));
    Ppmd8_RangeEnc_FlushData(&mut ppmd);
    return (fflush(stdout) != 0 as libc::c_int || ferror(stdin) != 0) as libc::c_int;
}
unsafe extern "C" fn decompress() -> libc::c_int {
    if fread(
        &mut hdr as *mut header as *mut libc::c_void,
        ::std::mem::size_of::<header>() as libc::c_ulong,
        1 as libc::c_int as size_t,
        stdin,
    ) != 1 as libc::c_int as libc::c_ulong
    {
        return 1 as libc::c_int;
    }
    if hdr.magic != 0x84acaf8f as libc::c_uint {
        return 1 as libc::c_int;
    }
    if hdr.info as libc::c_int >> 12 as libc::c_int != 'I' as i32 - 'A' as i32 {
        return 1 as libc::c_int;
    }
    let mut fname: [libc::c_char; 511] = [0; 511];
    let mut fnlen: size_t = (hdr.fnlen as libc::c_int & 0x1ff as libc::c_int) as size_t;
    if fread(
        fname.as_mut_ptr() as *mut libc::c_void,
        fnlen,
        1 as libc::c_int as size_t,
        stdin,
    ) != 1 as libc::c_int as libc::c_ulong
    {
        return 1 as libc::c_int;
    }
    opt_restore = hdr.fnlen as libc::c_int >> 14 as libc::c_int;
    opt_order = (hdr.info as libc::c_int & 0xf as libc::c_int) + 1 as libc::c_int;
    opt_mem =
        (hdr.info as libc::c_int >> 4 as libc::c_int & 0xff as libc::c_int) + 1 as libc::c_int;
    let mut cr: CharReader = {
        let mut init = CharReader {
            Read: Some(Read as unsafe extern "C" fn(_: *mut libc::c_void) -> Byte),
            fp: stdin,
            eof: 0 as libc::c_int != 0,
        };
        init
    };
    let mut ppmd: CPpmd8 = {
        let mut init = CPpmd8 {
            MinContext: 0 as *mut CPpmd8_Context,
            MaxContext: 0 as *mut CPpmd8_Context,
            FoundState: 0 as *mut CPpmd_State,
            OrderFall: 0,
            InitEsc: 0,
            PrevSuccess: 0,
            MaxOrder: 0,
            RunLength: 0,
            InitRL: 0,
            Size: 0,
            GlueCount: 0,
            Base: 0 as *mut Byte,
            LoUnit: 0 as *mut Byte,
            HiUnit: 0 as *mut Byte,
            Text: 0 as *mut Byte,
            UnitsStart: 0 as *mut Byte,
            AlignOffset: 0,
            RestoreMethod: 0,
            Range: 0,
            Code: 0,
            Low: 0,
            Stream: C2RustUnnamed {
                In: &mut cr as *mut CharReader as *mut IByteIn,
            },
            Indx2Units: [0; 38],
            Units2Indx: [0; 128],
            FreeList: [0; 38],
            Stamps: [0; 38],
            NS2BSIndx: [0; 256],
            NS2Indx: [0; 260],
            DummySee: CPpmd_See {
                Summ: 0,
                Shift: 0,
                Count: 0,
            },
            See: [[CPpmd_See {
                Summ: 0,
                Shift: 0,
                Count: 0,
            }; 32]; 24],
            BinSumm: [[0; 64]; 25],
        };
        init
    };
    Ppmd8_Construct(&mut ppmd);
    /*Ppmd8_Alloc(
        &mut ppmd,
        (opt_mem << 20 as libc::c_int) as UInt32,
        &mut ialloc as *mut ISzAlloc as ISzAllocPtr,
    );*/
    Ppmd8_RangeDec_Init(&mut ppmd);
    Ppmd8_Init(
        &mut ppmd,
        opt_order as libc::c_uint,
        opt_restore as libc::c_uint,
    );
    let mut buf: [libc::c_uchar; 8192] = [0; 8192];
    let mut n: size_t = 0 as libc::c_int as size_t;
    let mut c: libc::c_int = 0;
    loop {
        c = Ppmd8_DecodeSymbol(&mut ppmd);
        if cr.eof as libc::c_int != 0 || c < 0 as libc::c_int {
            break;
        }
        let fresh3 = n;
        n = n.wrapping_add(1);
        buf[fresh3 as usize] = c as libc::c_uchar;
        if n == ::std::mem::size_of::<[libc::c_uchar; 8192]>() as libc::c_ulong {
            fwrite(
                buf.as_mut_ptr() as *const libc::c_void,
                1 as libc::c_int as size_t,
                ::std::mem::size_of::<[libc::c_uchar; 8192]>() as libc::c_ulong,
                stdout,
            );
            n = 0 as libc::c_int as size_t
        }
    }
    if n != 0 {
        fwrite(
            buf.as_mut_ptr() as *const libc::c_void,
            1 as libc::c_int as size_t,
            n,
            stdout,
        );
    }
    return (fflush(stdout) != 0 as libc::c_int
        || c != -(1 as libc::c_int)
        || !(ppmd.Code == 0 as libc::c_int as libc::c_uint)
        || ferror(stdin) != 0
        || getchar_unlocked() != -(1 as libc::c_int)) as libc::c_int;
}
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
        c = getopt_long(
            argc,
            argv,
            b"dkcm:o:36h\x00" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            current_block = 7175849428784450219;
            break;
        }
        match c {
            100 => opt_d = 1 as libc::c_int != 0,
            107 => opt_k = 1 as libc::c_int != 0,
            99 => opt_c = 1 as libc::c_int != 0,
            109 => opt_mem = atoi(optarg),
            111 => opt_order = atoi(optarg),
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
                fputs(
                    b"ppmid-mini: too many arguments\n\x00" as *const u8 as *const libc::c_char,
                    stderr,
                );
            } else {
                let mut fname: *mut libc::c_char = if argc != 0 {
                    *argv.offset(0 as libc::c_int as isize)
                } else {
                    0 as *mut libc::c_char
                };
                if !fname.is_null()
                    && strcmp(fname, b"-\x00" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                {
                    fname = 0 as *mut libc::c_char
                }
                if fname.is_null() {
                    opt_c = 1 as libc::c_int != 0
                }
                if fname.is_null() && opt_d as libc::c_int != 0 && isatty(0 as libc::c_int) != 0 {
                    fprintf(
                        stderr,
                        b"ppmid-mini: compressed data cannot be read from a terminal\n\x00"
                            as *const u8 as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
                if opt_c as libc::c_int != 0 && !opt_d && isatty(1 as libc::c_int) != 0 {
                    fprintf(
                        stderr,
                        b"ppmid-mini: compressed data cannot be written to a terminal\n\x00"
                            as *const u8 as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
                if !fname.is_null() {
                    stdin = freopen(fname, b"r\x00" as *const u8 as *const libc::c_char, stdin);
                    if stdin.is_null() {
                        fprintf(
                            stderr,
                            b"ppmid-mini: cannot open %s\n\x00" as *const u8 as *const libc::c_char,
                            fname,
                        );
                        return 1 as libc::c_int;
                    }
                }
                if opt_d as libc::c_int != 0 && !opt_c {
                    let mut dot: *mut libc::c_char = strrchr(fname, '.' as i32);
                    if dot.is_null()
                        || *dot.offset(1 as libc::c_int as isize) as libc::c_int != 'p' as i32
                        || !strchr(dot, '/' as i32).is_null()
                    {
                        fprintf(
                            stderr,
                            b"ppmid-mini: unknown suffix: %s\n\x00" as *const u8
                                as *const libc::c_char,
                            fname,
                        );
                        return 1 as libc::c_int;
                    }
                    *dot = '\u{0}' as i32 as libc::c_char;
                    stdout = freopen(fname, b"w\x00" as *const u8 as *const libc::c_char, stdout);
                    if stdout.is_null() {
                        fprintf(
                            stderr,
                            b"ppmid-mini: cannot open %s\n\x00" as *const u8 as *const libc::c_char,
                            fname,
                        );
                        return 1 as libc::c_int;
                    }
                    *dot = '.' as i32 as libc::c_char
                }
                if !opt_d && !opt_c {
                    let mut len: size_t = strlen(fname);
                    let vla = len.wrapping_add(6 as libc::c_int as libc::c_ulong) as usize;
                    let mut outname: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
                    memcpy(
                        outname.as_mut_ptr() as *mut libc::c_void,
                        fname as *const libc::c_void,
                        len,
                    );
                    memcpy(
                        outname.as_mut_ptr().offset(len as isize) as *mut libc::c_void,
                        b".ppmd\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                        6 as libc::c_int as libc::c_ulong,
                    );
                    stdout = freopen(
                        outname.as_mut_ptr(),
                        b"w\x00" as *const u8 as *const libc::c_char,
                        stdout,
                    );
                    if stdout.is_null() {
                        fprintf(
                            stderr,
                            b"ppmid-mini: cannot open %s\n\x00" as *const u8 as *const libc::c_char,
                            outname.as_mut_ptr(),
                        );
                        return 1 as libc::c_int;
                    }
                }
                let mut rc: libc::c_int = if opt_d as libc::c_int != 0 {
                    decompress()
                } else {
                    compress()
                };
                if rc == 0 as libc::c_int && !opt_k && !opt_c {
                    fclose(stdin);
                    remove(fname);
                }
                return rc;
            }
        }
        _ => {}
    }
    fputs(
        b"Usage: ppmid-mini [-d] [-k] [-c] [FILE]\n\x00" as *const u8 as *const libc::c_char,
        stderr,
    );
    return 1 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
/* ex: set ts=8 sts=4 sw=4 noet: */
