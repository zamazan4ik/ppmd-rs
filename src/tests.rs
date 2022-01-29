#[cfg(test)]
mod test {
    use crate::ppmd_mini::{
        ferror, fflush, fread, freopen, fwrite, getchar_unlocked, hdr, header, opt_mem, opt_order,
        opt_restore, putchar, CharReader, CharWriter, Read, Write, _IO_putc, _IO_FILE,
    };
    use crate::Ppmd8::*;
    use crate::Ppmd8Dec::Ppmd8_DecodeSymbol;

    use crate::Ppmd8Enc::{Ppmd8_EncodeSymbol, Ppmd8_RangeEnc_FlushData};
    use std::ffi::CString;

    #[test]
    fn ppmd8_encode_small() {
        unsafe {
            let input = CString::new("tests/small_uncompressed.txt").unwrap();
            let reference_compressed_filename = "tests/small_compressed.txt.ppmd";
            let output_filename = format!("tests/generated/{}", uuid::Uuid::new_v4().to_string());
            let output = CString::new(output_filename.clone()).unwrap();
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
                | ('I' as i32 - 'A' as i32) << 12 as libc::c_int)
                as libc::c_ushort;
            fwrite(
                &mut hdr as *mut header as *const libc::c_void,
                ::std::mem::size_of::<header>() as libc::c_ulong,
                1 as libc::c_int as size_t,
                output_file as *mut _IO_FILE,
            );
            _IO_putc('a' as i32, output_file as *mut _IO_FILE);
            let mut cw: CharWriter = {
                let mut init = CharWriter {
                    Write: Some(Write as unsafe extern "C" fn(_: *mut libc::c_void, _: Byte) -> ()),
                    fp: output_file as *mut _IO_FILE,
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
                (opt_mem << 20 as libc::c_int) as UInt32,
                &mut ialloc as *mut ISzAlloc as ISzAllocPtr,
            );
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
                    input_file as *mut _IO_FILE,
                );
                if !(n != 0) {
                    break;
                }
                let mut i: size_t = 0 as libc::c_int as size_t;
                while i < n {
                    Ppmd8_EncodeSymbol(
                        &mut ppmd as *mut crate::Ppmd8::CPpmd8 as *mut crate::Ppmd8Enc::CPpmd8,
                        buf[i as usize] as libc::c_int,
                    );
                    i = i.wrapping_add(1)
                }
            }
            Ppmd8_EncodeSymbol(
                &mut ppmd as *mut crate::Ppmd8::CPpmd8 as *mut crate::Ppmd8Enc::CPpmd8,
                -(1 as libc::c_int),
            );
            Ppmd8_RangeEnc_FlushData(
                &mut ppmd as *mut crate::Ppmd8::CPpmd8 as *mut crate::Ppmd8Enc::CPpmd8,
            );
            (fflush(output_file as *mut _IO_FILE) != 0 as libc::c_int
                || ferror(input_file as *mut _IO_FILE) != 0) as libc::c_int;

            // Compare files
            let mut resulting_file = match std::fs::File::open(output_filename.clone()) {
                Ok(f) => f,
                Err(e) => panic!("{}", e),
            };
            let mut reference_file = match std::fs::File::open(reference_compressed_filename) {
                Ok(f) => f,
                Err(e) => panic!("{}", e),
            };

            assert!(file_diff::diff_files(
                &mut resulting_file,
                &mut reference_file
            ));
        }
    }

    #[test]
    fn ppmd8_decode_small() {
        unsafe {
            let input = CString::new("tests/small_compressed.txt.ppmd").unwrap();
            let reference_uncompressed_filename = "tests/small_uncompressed.txt";
            let output_filename = format!("tests/generated/{}", uuid::Uuid::new_v4().to_string());
            let output = CString::new(output_filename.clone()).unwrap();
            let mut input_file = libc::fopen(
                input.into_raw(),
                b"r\x00" as *const u8 as *const libc::c_char,
            );
            let mut output_file = libc::fopen(
                output.into_raw(),
                b"w\x00" as *const u8 as *const libc::c_char,
            );
            if fread(
                &mut hdr as *mut header as *mut libc::c_void,
                ::std::mem::size_of::<header>() as libc::c_ulong,
                1 as libc::c_int as size_t,
                input_file as *mut _IO_FILE,
            ) != 1 as libc::c_int as libc::c_ulong
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
            if fread(
                fname.as_mut_ptr() as *mut libc::c_void,
                fnlen,
                1 as libc::c_int as size_t,
                input_file as *mut _IO_FILE,
            ) != 1 as libc::c_int as libc::c_ulong
            {
                println!("4");
                //return 1 as libc::c_int;
            }
            opt_restore = hdr.fnlen as libc::c_int >> 14 as libc::c_int;
            opt_order = (hdr.info as libc::c_int & 0xf as libc::c_int) + 1 as libc::c_int;
            opt_mem = (hdr.info as libc::c_int >> 4 as libc::c_int & 0xff as libc::c_int)
                + 1 as libc::c_int;

            let mut cr: CharReader = unsafe {
                let mut init = CharReader {
                    Read: Some(Read as unsafe extern "C" fn(_: *mut libc::c_void) -> Byte),
                    fp: input_file as *mut _IO_FILE,
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
                (opt_mem << 20 as libc::c_int) as UInt32,
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
                c = Ppmd8_DecodeSymbol(
                    &mut ppmd as *mut crate::Ppmd8::CPpmd8 as *mut crate::Ppmd8Dec::CPpmd8,
                );
                if cr.eof as libc::c_int != 0 || c < 0 as libc::c_int {
                    break;
                }
                let fresh3 = n;
                n = n.wrapping_add(1);
                buf[fresh3 as usize] = c as libc::c_uchar;
                if n == ::std::mem::size_of::<[libc::c_uchar; 8192]>() as libc::c_ulong {
                    fwrite(
                        buf.as_mut_ptr() as *const libc::c_void,
                        (1 as libc::c_int as size_t).try_into().unwrap(),
                        (::std::mem::size_of::<[libc::c_uchar; 8192]>() as libc::c_ulong)
                            .try_into()
                            .unwrap(),
                        output_file as *mut _IO_FILE,
                    );
                    n = 0 as libc::c_int as size_t
                }
            }
            if n != 0 {
                fwrite(
                    buf.as_mut_ptr() as *const libc::c_void,
                    (1 as libc::c_int as size_t).try_into().unwrap(),
                    n.try_into().unwrap(),
                    output_file as *mut _IO_FILE,
                );
            }
            (fflush(output_file as *mut _IO_FILE) != 0 as libc::c_int
                || c != -(1 as libc::c_int)
                || !(ppmd.Code == 0 as libc::c_int as libc::c_uint)
                || ferror(input_file as *mut _IO_FILE) != 0) as libc::c_int;

            // Compare files
            let mut resulting_file = match std::fs::File::open(output_filename.clone()) {
                Ok(f) => f,
                Err(e) => panic!("{}", e),
            };
            let mut reference_file = match std::fs::File::open(reference_uncompressed_filename) {
                Ok(f) => f,
                Err(e) => panic!("{}", e),
            };

            assert!(file_diff::diff_files(
                &mut resulting_file,
                &mut reference_file
            ));
        }
    }
}
