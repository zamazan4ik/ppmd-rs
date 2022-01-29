pub mod Ppmd8;
pub mod Ppmd8Dec;
pub mod Ppmd8Enc;
pub mod ppmd_mini;
mod tests;

use crate::ppmd_mini::{
    hdr, header, opt_mem, opt_order, opt_restore, CharReader, CharWriter, Read, Write,
};
use crate::Ppmd8::*;
use crate::Ppmd8Dec::Ppmd8_DecodeSymbol;

use crate::Ppmd8Enc::{Ppmd8_EncodeSymbol, Ppmd8_RangeEnc_FlushData};
use std::ffi::CString;

pub unsafe fn compress(input: std::path::PathBuf, output: std::path::PathBuf) {
    let input = CString::new(input.to_str().unwrap()).unwrap();
    let output = CString::new(output.to_str().unwrap()).unwrap();

    let mut input_file = libc::fopen(
        input.into_raw(),
        b"r\x00" as *const u8 as *const libc::c_char,
    );
    let mut output_file = libc::fopen(
        output.into_raw(),
        b"w\x00" as *const u8 as *const libc::c_char,
    );
    hdr.info = (opt_order - 1 as libc::c_int
        | (opt_mem - 1 as libc::c_int) << 4 as libc::c_int
        | ('I' as i32 - 'A' as i32) << 12 as libc::c_int) as libc::c_ushort;
    libc::fwrite(
        &mut hdr as *mut header as *const libc::c_void,
        (::std::mem::size_of::<header>() as libc::c_ulong)
            .try_into()
            .unwrap(),
        (1 as libc::c_int as size_t).try_into().unwrap(),
        output_file,
    );
    libc::fputc('a' as i32, output_file);
    let mut cw: CharWriter = {
        let mut init = CharWriter {
            Write: Some(Write as unsafe extern "C" fn(_: *mut libc::c_void, _: Byte) -> ()),
            fp: output_file,
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
            Stream: C2RustUnnamed_0 {
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
    Ppmd8_Alloc(
        &mut ppmd,
        (opt_mem << 20 as libc::c_int) as libc::c_uint,
        &mut ialloc as *mut ISzAlloc as ISzAllocPtr,
    );
    ppmd.Low = 0 as libc::c_int as libc::c_uint;
    ppmd.Range = 0xffffffff as libc::c_uint;
    Ppmd8_Init(
        &mut ppmd,
        opt_order as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
    );
    let mut buf: [libc::c_uchar; 8192] = [0; 8192];
    let mut n: size_t = 0;
    loop {
        n = libc::fread(
            buf.as_mut_ptr() as *mut libc::c_void,
            1,
            (::std::mem::size_of::<[libc::c_uchar; 8192]>() as libc::c_ulong)
                .try_into()
                .unwrap(),
            input_file,
        ) as size_t;
        if !(n != 0) {
            break;
        }
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < n {
            Ppmd8_EncodeSymbol(
                &mut ppmd as *mut crate::Ppmd8::CPpmd8,
                buf[i as usize] as libc::c_int,
            );
            i = i.wrapping_add(1)
        }
    }
    Ppmd8_EncodeSymbol(&mut ppmd as *mut crate::Ppmd8::CPpmd8, -(1 as libc::c_int));
    Ppmd8_RangeEnc_FlushData(&mut ppmd as *mut crate::Ppmd8::CPpmd8);
    (libc::fflush(output_file) != 0 as libc::c_int || libc::ferror(input_file) != 0) as libc::c_int;
}

pub unsafe fn decompress(input: std::path::PathBuf, output: std::path::PathBuf) {
    let input = CString::new(input.to_str().unwrap()).unwrap();
    let output = CString::new(output.to_str().unwrap()).unwrap();
    let mut input_file = libc::fopen(
        input.into_raw(),
        b"r\x00" as *const u8 as *const libc::c_char,
    );
    let mut output_file = libc::fopen(
        output.into_raw(),
        b"w\x00" as *const u8 as *const libc::c_char,
    );
    if libc::fread(
        &mut hdr as *mut header as *mut libc::c_void,
        (::std::mem::size_of::<header>() as libc::c_ulong)
            .try_into()
            .unwrap(),
        1,
        input_file,
    ) != 1
    {
        println!("1");
        //return 1 as libc::c_int;
    }
    if hdr.magic != 0x84acaf8f as libc::c_uint {
        println!("2");
        //return 1 as libc::c_int;
    }
    if hdr.info as libc::c_int >> 12 as libc::c_int != 'I' as i32 - 'A' as i32 {
        println!("3");
        //return 1 as libc::c_int;
    }
    let mut fname: [libc::c_char; 511] = [0; 511];
    let mut fnlen: size_t = (hdr.fnlen as libc::c_int & 0x1ff as libc::c_int) as size_t;
    if libc::fread(
        fname.as_mut_ptr() as *mut libc::c_void,
        fnlen.try_into().unwrap(),
        1,
        input_file,
    ) != 1
    {
        println!("4");
        //return 1 as libc::c_int;
    }
    opt_restore = hdr.fnlen as libc::c_int >> 14 as libc::c_int;
    opt_order = (hdr.info as libc::c_int & 0xf as libc::c_int) + 1 as libc::c_int;
    opt_mem =
        (hdr.info as libc::c_int >> 4 as libc::c_int & 0xff as libc::c_int) + 1 as libc::c_int;

    let mut cr: CharReader = unsafe {
        let mut init = CharReader {
            Read: Some(Read as unsafe extern "C" fn(_: *mut libc::c_void) -> Byte),
            fp: input_file,
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
            Stream: C2RustUnnamed_0 {
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
    Ppmd8_Alloc(
        &mut ppmd,
        (opt_mem << 20 as libc::c_int) as libc::c_uint,
        &mut ialloc as *mut ISzAlloc as ISzAllocPtr,
    );
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
        c = Ppmd8_DecodeSymbol(&mut ppmd as *mut crate::Ppmd8::CPpmd8);
        if cr.eof as libc::c_int != 0 || c < 0 as libc::c_int {
            break;
        }
        let fresh3 = n;
        n = n.wrapping_add(1);
        buf[fresh3 as usize] = c as libc::c_uchar;
        if n == ::std::mem::size_of::<[libc::c_uchar; 8192]>() as libc::c_ulong {
            libc::fwrite(
                buf.as_mut_ptr() as *const libc::c_void,
                (1 as libc::c_int as size_t).try_into().unwrap(),
                (::std::mem::size_of::<[libc::c_uchar; 8192]>() as libc::c_ulong)
                    .try_into()
                    .unwrap(),
                output_file,
            );
            n = 0 as libc::c_int as size_t
        }
    }
    if n != 0 {
        libc::fwrite(
            buf.as_mut_ptr() as *const libc::c_void,
            (1 as libc::c_int as size_t).try_into().unwrap(),
            n.try_into().unwrap(),
            output_file,
        );
    }
    (libc::fflush(output_file) != 0 as libc::c_int
        || c != -(1 as libc::c_int)
        || !(ppmd.Code == 0 as libc::c_int as libc::c_uint)
        || libc::ferror(input_file) != 0) as libc::c_int;
}
