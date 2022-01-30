pub mod Ppmd8;
pub mod Ppmd8Dec;
pub mod Ppmd8Enc;
pub mod ppmd_mini;
mod tests;

use crate::ppmd_mini::{header, CharReader, CharWriter, Read, Write};
use crate::Ppmd8::*;
use crate::Ppmd8Dec::Ppmd8_DecodeSymbol;

use crate::Ppmd8Enc::{Ppmd8_EncodeSymbol, Ppmd8_RangeEnc_FlushData};
use std::ffi::CString;

pub unsafe fn compress(input: std::path::PathBuf, output: std::path::PathBuf) {
    let input = CString::new(input.to_str().unwrap()).unwrap();
    let output = CString::new(output.to_str().unwrap()).unwrap();

    let input_file = libc::fopen(
        input.into_raw(),
        b"r\x00" as *const u8 as *const libc::c_char,
    );
    let output_file = libc::fopen(
        output.into_raw(),
        b"w\x00" as *const u8 as *const libc::c_char,
    );

    let mut hdr = header {
        magic: 0x84acaf8f as u32,
        attr: 0x80 as i32 as u32,
        info: 0 as i32 as libc::c_ushort,
        fnlen: 1 as i32 as libc::c_ushort,
        date: 0 as i32 as libc::c_ushort,
        time: 0 as i32 as libc::c_ushort,
    };

    let opt_mem: i32 = 8 as i32;
    let opt_order: i32 = 6 as i32;

    hdr.info = (opt_order - 1 as i32
        | (opt_mem - 1 as i32) << 4 as i32
        | ('I' as i32 - 'A' as i32) << 12 as i32) as libc::c_ushort;
    libc::fwrite(
        &mut hdr as *mut header as *const libc::c_void,
        (::std::mem::size_of::<header>() as u64).try_into().unwrap(),
        (1 as i32 as u64).try_into().unwrap(),
        output_file,
    );
    libc::fputc('a' as i32, output_file);
    let mut cw: CharWriter = {
        let init = CharWriter {
            Write: Some(Write as unsafe extern "C" fn(_: *mut libc::c_void, _: u8) -> ()),
            fp: output_file,
        };
        init
    };

    let mut ppmd: CPpmd8 = {
        let init = CPpmd8 {
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
            Base: 0 as *mut u8,
            LoUnit: 0 as *mut u8,
            HiUnit: 0 as *mut u8,
            Text: 0 as *mut u8,
            UnitsStart: 0 as *mut u8,
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
        (opt_mem << 20 as i32) as u32,
        &mut ialloc as *mut ISzAlloc as ISzAllocPtr,
    );
    ppmd.Low = 0 as i32 as u32;
    ppmd.Range = 0xffffffff as u32;
    Ppmd8_Init(&mut ppmd, opt_order as u32, 0 as i32 as u32);
    let mut buf: [libc::c_uchar; 8192] = [0; 8192];
    let mut n;
    loop {
        n = libc::fread(
            buf.as_mut_ptr() as *mut libc::c_void,
            1,
            (::std::mem::size_of::<[libc::c_uchar; 8192]>() as u64)
                .try_into()
                .unwrap(),
            input_file,
        ) as u64;
        if !(n != 0) {
            break;
        }
        let mut i: u64 = 0 as i32 as u64;
        while i < n {
            Ppmd8_EncodeSymbol(
                &mut ppmd as *mut crate::Ppmd8::CPpmd8,
                buf[i as usize] as i32,
            );
            i = i.wrapping_add(1)
        }
    }
    Ppmd8_EncodeSymbol(&mut ppmd as *mut crate::Ppmd8::CPpmd8, -(1 as i32));
    Ppmd8_RangeEnc_FlushData(&mut ppmd as *mut crate::Ppmd8::CPpmd8);
    (libc::fflush(output_file) != 0 as i32 || libc::ferror(input_file) != 0) as i32;
}

pub unsafe fn decompress(input: std::path::PathBuf, output: std::path::PathBuf) {
    let input = CString::new(input.to_str().unwrap()).unwrap();
    let output = CString::new(output.to_str().unwrap()).unwrap();
    let input_file = libc::fopen(
        input.into_raw(),
        b"r\x00" as *const u8 as *const libc::c_char,
    );
    let output_file = libc::fopen(
        output.into_raw(),
        b"w\x00" as *const u8 as *const libc::c_char,
    );

    let mut hdr = header {
        magic: 0x84acaf8f as u32,
        attr: 0x80 as i32 as u32,
        info: 0 as i32 as libc::c_ushort,
        fnlen: 1 as i32 as libc::c_ushort,
        date: 0 as i32 as libc::c_ushort,
        time: 0 as i32 as libc::c_ushort,
    };

    if libc::fread(
        &mut hdr as *mut header as *mut libc::c_void,
        (::std::mem::size_of::<header>() as u64).try_into().unwrap(),
        1,
        input_file,
    ) != 1
    {
        println!("1");
        //return 1 as i32;
    }
    if hdr.magic != 0x84acaf8f as u32 {
        println!("2");
        //return 1 as i32;
    }
    if hdr.info as i32 >> 12 as i32 != 'I' as i32 - 'A' as i32 {
        println!("3");
        //return 1 as i32;
    }
    let mut fname: [libc::c_char; 511] = [0; 511];
    let fnlen: u64 = (hdr.fnlen as i32 & 0x1ff as i32) as u64;
    if libc::fread(
        fname.as_mut_ptr() as *mut libc::c_void,
        fnlen.try_into().unwrap(),
        1,
        input_file,
    ) != 1
    {
        println!("4");
        //return 1 as i32;
    }
    let opt_restore = hdr.fnlen as i32 >> 14 as i32;
    let opt_order = (hdr.info as i32 & 0xf as i32) + 1 as i32;
    let opt_mem = (hdr.info as i32 >> 4 as i32 & 0xff as i32) + 1 as i32;

    let mut cr: CharReader = {
        let init = CharReader {
            Read: Some(Read as unsafe extern "C" fn(_: *mut libc::c_void) -> u8),
            fp: input_file,
            eof: 0 as i32 != 0,
        };
        init
    };

    let mut ppmd: CPpmd8 = {
        let init = CPpmd8 {
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
            Base: 0 as *mut u8,
            LoUnit: 0 as *mut u8,
            HiUnit: 0 as *mut u8,
            Text: 0 as *mut u8,
            UnitsStart: 0 as *mut u8,
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
        (opt_mem << 20 as i32) as u32,
        &mut ialloc as *mut ISzAlloc as ISzAllocPtr,
    );
    Ppmd8_RangeDec_Init(&mut ppmd);
    Ppmd8_Init(&mut ppmd, opt_order as u32, opt_restore as u32);
    let mut buf: [libc::c_uchar; 8192] = [0; 8192];
    let mut n: u64 = 0 as i32 as u64;
    let mut c: i32;

    loop {
        c = Ppmd8_DecodeSymbol(&mut ppmd as *mut crate::Ppmd8::CPpmd8);
        if cr.eof as i32 != 0 || c < 0 as i32 {
            break;
        }
        let fresh3 = n;
        n = n.wrapping_add(1);
        buf[fresh3 as usize] = c as libc::c_uchar;
        if n == ::std::mem::size_of::<[libc::c_uchar; 8192]>() as u64 {
            libc::fwrite(
                buf.as_mut_ptr() as *const libc::c_void,
                (1 as i32 as u64).try_into().unwrap(),
                (::std::mem::size_of::<[libc::c_uchar; 8192]>() as u64)
                    .try_into()
                    .unwrap(),
                output_file,
            );
            n = 0 as i32 as u64
        }
    }
    if n != 0 {
        libc::fwrite(
            buf.as_mut_ptr() as *const libc::c_void,
            (1 as i32 as u64).try_into().unwrap(),
            n.try_into().unwrap(),
            output_file,
        );
    }
    (libc::fflush(output_file) != 0 as i32
        || c != -(1 as i32)
        || !(ppmd.Code == 0 as i32 as u32)
        || libc::ferror(input_file) != 0) as i32;
}
