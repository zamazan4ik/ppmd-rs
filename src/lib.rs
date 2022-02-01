mod Ppmd8Dec;
mod Ppmd8Enc;
mod Ppmd8_new;
mod ppmd8;
mod tests;

use crate::ppmd8::*;

use std::ffi::CString;

/// # Safety
///
/// This function is unsafe
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

    let mut hdr = Header::default();

    let opt_mem: i32 = 8_i32;
    let opt_order: i32 = 6_i32;

    hdr.info = ((opt_order - 1_i32)
        | (opt_mem - 1_i32) << 4_i32
        | ('I' as i32 - 'A' as i32) << 12_i32) as libc::c_ushort;
    libc::fwrite(
        &mut hdr as *mut Header as *const libc::c_void,
        (::std::mem::size_of::<Header>() as u64).try_into().unwrap(),
        (1_i32 as u64).try_into().unwrap(),
        output_file,
    );
    libc::fputc('a' as i32, output_file);
    let mut char_writer = CharWriter {
        write: Some(write as unsafe extern "C" fn(_: *mut libc::c_void, _: u8) -> ()),
        fp: output_file,
    };

    let mut ppmd = ppmd8::CPpmd8::new_encoder(&mut char_writer);
    ppmd.allocate(
        (opt_mem << 20_i32) as u32,
        &mut IALLOC as *mut ISzAlloc as ISzAllocPtr,
    );
    ppmd.low = 0;
    ppmd.range = 0xffffffff_u32;
    ppmd.init(opt_order as u32, 0);
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
        if n == 0 {
            break;
        }
        let mut i: u64 = 0_i32 as u64;
        while i < n {
            ppmd.encode_symbol(buf[i as usize] as i32);
            i = i.wrapping_add(1)
        }
    }
    ppmd.encode_symbol(-1_i32);
    ppmd.range_enc_flush_data();
    let _ = if libc::fflush(output_file) == 0_i32 {
        let _ = libc::ferror(input_file);
    };
}

/// # Safety
///
/// This function is unsafe
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

    let mut hdr = Header::default();

    if libc::fread(
        &mut hdr as *mut Header as *mut libc::c_void,
        (::std::mem::size_of::<Header>() as u64).try_into().unwrap(),
        1,
        input_file,
    ) != 1
    {
        println!("1");
        //return 1 as i32;
    }
    if hdr.magic != 0x84acaf8f_u32 {
        println!("2");
        //return 1 as i32;
    }
    if hdr.info as i32 >> 12_i32 != 'I' as i32 - 'A' as i32 {
        println!("3");
        //return 1 as i32;
    }
    let mut fname: [libc::c_char; 511] = [0; 511];
    let fnlen: u64 = (hdr.fnlen as i32 & 0x1ff_i32) as u64;
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
    let opt_restore = hdr.fnlen as i32 >> 14_i32;
    let opt_order = (hdr.info as i32 & 0xf_i32) + 1_i32;
    let opt_mem = (hdr.info as i32 >> 4_i32 & 0xff_i32) + 1_i32;

    let mut char_reader = CharReader {
        read: Some(read as unsafe extern "C" fn(_: *mut libc::c_void) -> u8),
        fp: input_file,
        eof: false,
    };

    let mut ppmd = ppmd8::CPpmd8::new_decoder(&mut char_reader);
    ppmd.allocate(
        (opt_mem << 20_i32) as u32,
        &mut IALLOC as *mut ISzAlloc as ISzAllocPtr,
    );
    ppmd.range_decoder_init();
    ppmd.init(opt_order as u32, opt_restore as u32);
    let mut buf: [libc::c_uchar; 8192] = [0; 8192];
    let mut n: u64 = 0_i32 as u64;
    let mut c: i32;

    loop {
        c = ppmd.decode_symbol();
        if char_reader.eof as i32 != 0 || c < 0_i32 {
            break;
        }
        let fresh3 = n;
        n = n.wrapping_add(1);
        buf[fresh3 as usize] = c as libc::c_uchar;
        if n == ::std::mem::size_of::<[libc::c_uchar; 8192]>() as u64 {
            libc::fwrite(
                buf.as_mut_ptr() as *const libc::c_void,
                (1_i32 as u64).try_into().unwrap(),
                (::std::mem::size_of::<[libc::c_uchar; 8192]>() as u64)
                    .try_into()
                    .unwrap(),
                output_file,
            );
            n = 0_i32 as u64
        }
    }
    if n != 0 {
        libc::fwrite(
            buf.as_mut_ptr() as *const libc::c_void,
            (1_i32 as u64).try_into().unwrap(),
            n.try_into().unwrap(),
            output_file,
        );
    }
    let _ = if !(libc::fflush(output_file) != 0_i32 || c != -1_i32 || ppmd.code != 0_i32 as u32) {
        let _ = libc::ferror(input_file);
    };
}
