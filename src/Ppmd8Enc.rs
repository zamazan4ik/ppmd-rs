use crate::{
    CPpmd8, CPpmd8_Context, CPpmd8_Context_Ref, CPpmd_See, CPpmd_State, CPpmd_State_Ref, IByteIn,
    IByteOut, Ppmd8_MakeEscFreq, Ppmd8_Update1, Ppmd8_Update1_0, Ppmd8_Update2, Ppmd8_UpdateBin,
};

extern "C" {
    static PPMD8_kExpEscape: [libc::c_uchar; 16];
}

/* Ppmd.h -- PPMD codec common code
2017-04-03 : Igor Pavlov : Public domain
This code is based on PPMd var.H (2001): Dmitry Shkarin : Public domain */
/* Most compilers works OK here even without #pragma pack(push, 1), but some GCC compilers need it. */
/* SEE-contexts for PPM-contexts with masked symbols */

/* returns: -1 as EndMarker, -2 as DataError */
/* ---------- Encode ---------- */
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_RangeEnc_FlushData(mut p: *mut CPpmd8) {
    let mut i: u32 = 0; /* EndMarker (symbol = -1) */
    while i < 4 {
        (*(*p).Stream.Out).Write.expect("non-null function pointer")(
            (*p).Stream.Out,
            ((*p).Low >> 24 as i32) as libc::c_uchar,
        );
        i = i.wrapping_add(1);
        (*p).Low <<= 8 as i32
    }
}
unsafe extern "C" fn RangeEnc_Normalize(mut p: *mut CPpmd8) {
    while (*p).Low ^ (*p).Low.wrapping_add((*p).Range) < ((1 as i32) << 24 as i32) as u32
        || (*p).Range < ((1 as i32) << 15 as i32) as u32 && {
            (*p).Range = (0 as i32 as u32).wrapping_sub((*p).Low)
                & (((1 as i32) << 15 as i32) - 1 as i32) as u32;
            (1 as i32) != 0
        }
    {
        (*(*p).Stream.Out).Write.expect("non-null function pointer")(
            (*p).Stream.Out,
            ((*p).Low >> 24 as i32) as libc::c_uchar,
        );
        (*p).Range <<= 8 as i32;
        (*p).Low <<= 8 as i32
    }
}
unsafe extern "C" fn RangeEnc_Encode(mut p: *mut CPpmd8, start: u32, size: u32, total: u32) {
    (*p).Range = ((*p).Range as u32).wrapping_div(total) as u32;
    (*p).Low = ((*p).Low as u32).wrapping_add(start.wrapping_mul((*p).Range)) as u32;
    (*p).Range = ((*p).Range as u32).wrapping_mul(size) as u32;
    RangeEnc_Normalize(p);
}
unsafe extern "C" fn RangeEnc_EncodeBit_0(mut p: *mut CPpmd8, size0: u32) {
    (*p).Range >>= 14 as i32;
    (*p).Range = ((*p).Range as u32).wrapping_mul(size0) as u32;
    RangeEnc_Normalize(p);
}
unsafe extern "C" fn RangeEnc_EncodeBit_1(mut p: *mut CPpmd8, size0: u32) {
    (*p).Range >>= 14 as i32;
    (*p).Low = ((*p).Low as u32).wrapping_add(size0.wrapping_mul((*p).Range)) as u32 as u32;
    (*p).Range = ((*p).Range as u32)
        .wrapping_mul((((1 as i32) << 14 as i32) as u32).wrapping_sub(size0))
        as u32 as u32;
    RangeEnc_Normalize(p);
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_EncodeSymbol(mut p: *mut CPpmd8, symbol: i32) {
    let mut charMask: [u64; 32] = [0; 32];
    if (*(*p).MinContext).NumStats as i32 != 0 as i32 {
        let mut s: *mut CPpmd_State = (*p).Base.offset((*(*p).MinContext).Stats as isize)
            as *mut libc::c_void as *mut CPpmd_State;
        let mut sum: u32 = 0;
        let mut i: u32 = 0;
        if (*s).Symbol as i32 == symbol {
            RangeEnc_Encode(
                p,
                0 as i32 as u32,
                (*s).Freq as u32,
                (*(*p).MinContext).SummFreq as u32,
            );
            (*p).FoundState = s;
            Ppmd8_Update1_0(p);
            return;
        }
        (*p).PrevSuccess = 0 as i32 as u32;
        sum = (*s).Freq as u32;
        i = (*(*p).MinContext).NumStats as u32;
        loop {
            s = s.offset(1);
            if (*s).Symbol as i32 == symbol {
                RangeEnc_Encode(p, sum, (*s).Freq as u32, (*(*p).MinContext).SummFreq as u32);
                (*p).FoundState = s;
                Ppmd8_Update1(p);
                return;
            }
            sum = (sum as u32).wrapping_add((*s).Freq as u32) as u32 as u32;
            i = i.wrapping_sub(1);
            if !(i != 0) {
                break;
            }
        }
        let mut z: u64 = 0;
        z = 0 as i32 as u64;
        while z < (256 as i32 as u64).wrapping_div(::std::mem::size_of::<u64>() as u64) {
            charMask[z.wrapping_add(0 as i32 as u64) as usize] = !(0 as i32 as u64);
            charMask[z.wrapping_add(1 as i32 as u64) as usize] =
                charMask[z.wrapping_add(0 as i32 as u64) as usize];
            charMask[z.wrapping_add(2 as i32 as u64) as usize] =
                charMask[z.wrapping_add(1 as i32 as u64) as usize];
            charMask[z.wrapping_add(3 as i32 as u64) as usize] =
                charMask[z.wrapping_add(2 as i32 as u64) as usize];
            charMask[z.wrapping_add(4 as i32 as u64) as usize] =
                charMask[z.wrapping_add(3 as i32 as u64) as usize];
            charMask[z.wrapping_add(5 as i32 as u64) as usize] =
                charMask[z.wrapping_add(4 as i32 as u64) as usize];
            charMask[z.wrapping_add(6 as i32 as u64) as usize] =
                charMask[z.wrapping_add(5 as i32 as u64) as usize];
            charMask[z.wrapping_add(7 as i32 as u64) as usize] =
                charMask[z.wrapping_add(6 as i32 as u64) as usize];
            z = (z as u64).wrapping_add(8 as i32 as u64) as u64 as u64
        }
        *(charMask.as_mut_ptr() as *mut libc::c_schar).offset((*s).Symbol as isize) =
            0 as i32 as libc::c_schar;
        i = (*(*p).MinContext).NumStats as u32;
        loop {
            s = s.offset(-1);
            *(charMask.as_mut_ptr() as *mut libc::c_schar).offset((*s).Symbol as isize) =
                0 as i32 as libc::c_schar;
            i = i.wrapping_sub(1);
            if !(i != 0) {
                break;
            }
        }
        RangeEnc_Encode(
            p,
            sum,
            ((*(*p).MinContext).SummFreq as u32).wrapping_sub(sum),
            (*(*p).MinContext).SummFreq as u32,
        );
    } else {
        let prob: *mut u16 = &mut *(*(*p).BinSumm.as_mut_ptr().offset(
            *(*p).NS2Indx.as_mut_ptr().offset(
                ((*(&mut (*(*p).MinContext).SummFreq as *mut u16 as *mut CPpmd_State)).Freq as u64)
                    .wrapping_sub(1 as i32 as u64) as isize,
            ) as isize,
        ))
        .as_mut_ptr()
        .offset(
            (*(*p).NS2BSIndx.as_mut_ptr().offset(
                (*((*p).Base.offset((*(*p).MinContext).Suffix as isize) as *mut libc::c_void
                    as *mut CPpmd8_Context))
                    .NumStats as isize,
            ) as u32)
                .wrapping_add((*p).PrevSuccess)
                .wrapping_add((*(*p).MinContext).Flags as u32)
                .wrapping_add(((*p).RunLength >> 26 as i32 & 0x20 as i32) as u32)
                as isize,
        ) as *mut u16;
        let s_0: *mut CPpmd_State =
            &mut (*(*p).MinContext).SummFreq as *mut u16 as *mut CPpmd_State;
        if (*s_0).Symbol as i32 == symbol {
            RangeEnc_EncodeBit_0(p, *prob as u32);
            *prob = (*prob as i32 + ((1 as i32) << 7 as i32)
                - (*prob as i32 + ((1 as i32) << 7 as i32 - 2 as i32) >> 7 as i32))
                as u16;
            (*p).FoundState = s_0;
            Ppmd8_UpdateBin(p);
            return;
        } else {
            RangeEnc_EncodeBit_1(p, *prob as u32);
            *prob = (*prob as i32
                - (*prob as i32 + ((1 as i32) << 7 as i32 - 2 as i32) >> 7 as i32))
                as u16;
            (*p).InitEsc = PPMD8_kExpEscape[(*prob as i32 >> 10 as i32) as usize] as u32;
            let mut z_0: u64 = 0;
            while z_0 < (256 as i32 as u64).wrapping_div(::std::mem::size_of::<u64>() as u64) {
                charMask[z_0.wrapping_add(0) as usize] = !(0);
                charMask[z_0.wrapping_add(1) as usize] = charMask[z_0.wrapping_add(0) as usize];
                charMask[z_0.wrapping_add(2) as usize] = charMask[z_0.wrapping_add(1) as usize];
                charMask[z_0.wrapping_add(3) as usize] = charMask[z_0.wrapping_add(2) as usize];
                charMask[z_0.wrapping_add(4) as usize] = charMask[z_0.wrapping_add(3) as usize];
                charMask[z_0.wrapping_add(5) as usize] = charMask[z_0.wrapping_add(4) as usize];
                charMask[z_0.wrapping_add(6) as usize] = charMask[z_0.wrapping_add(5) as usize];
                charMask[z_0.wrapping_add(7) as usize] = charMask[z_0.wrapping_add(6) as usize];
                z_0 = (z_0 as u64).wrapping_add(8)
            }
            *(charMask.as_mut_ptr() as *mut libc::c_schar).offset((*s_0).Symbol as isize) =
                0 as i32 as libc::c_schar;
            (*p).PrevSuccess = 0 as i32 as u32
        }
    }
    loop {
        let mut escFreq: u32 = 0;
        let mut see: *mut CPpmd_See = 0 as *mut CPpmd_See;
        let mut s_1: *mut CPpmd_State = 0 as *mut CPpmd_State;
        let mut sum_0: u32 = 0;
        let mut i_0: u32 = 0;
        let numMasked: u32 = (*(*p).MinContext).NumStats as u32;
        loop {
            (*p).OrderFall = (*p).OrderFall.wrapping_add(1);
            if (*(*p).MinContext).Suffix == 0 {
                return;
            }
            (*p).MinContext = (*p).Base.offset((*(*p).MinContext).Suffix as isize)
                as *mut libc::c_void as *mut CPpmd8_Context;
            if !((*(*p).MinContext).NumStats as u32 == numMasked) {
                break;
            }
        }
        see = Ppmd8_MakeEscFreq(p, numMasked, &mut escFreq);
        s_1 = (*p).Base.offset((*(*p).MinContext).Stats as isize) as *mut libc::c_void
            as *mut CPpmd_State;
        sum_0 = 0 as i32 as u32;
        i_0 = ((*(*p).MinContext).NumStats as i32 + 1 as i32) as u32;
        loop {
            let cur: i32 = (*s_1).Symbol as i32;
            if cur == symbol {
                let low: u32 = sum_0;
                let s1: *mut CPpmd_State = s_1;
                loop {
                    sum_0 = (sum_0 as u32).wrapping_add(
                        ((*s_1).Freq as i32
                            & *(charMask.as_mut_ptr() as *mut libc::c_schar)
                                .offset((*s_1).Symbol as isize)
                                as i32) as u32,
                    ) as u32 as u32;
                    s_1 = s_1.offset(1);
                    i_0 = i_0.wrapping_sub(1);
                    if !(i_0 != 0) {
                        break;
                    }
                }
                RangeEnc_Encode(p, low, (*s1).Freq as u32, sum_0.wrapping_add(escFreq));
                if ((*see).Shift as i32) < 7 as i32 && {
                    (*see).Count = (*see).Count.wrapping_sub(1);
                    ((*see).Count as i32) == 0 as i32
                } {
                    (*see).Summ = (((*see).Summ as i32) << 1 as i32) as u16;
                    let fresh0 = (*see).Shift;
                    (*see).Shift = (*see).Shift.wrapping_add(1);
                    (*see).Count = ((3 as i32) << fresh0 as i32) as libc::c_uchar
                }
                (*p).FoundState = s1;
                Ppmd8_Update2(p);
                return;
            }
            sum_0 = (sum_0 as u32).wrapping_add(
                ((*s_1).Freq as i32
                    & *(charMask.as_mut_ptr() as *mut libc::c_schar).offset(cur as isize) as i32)
                    as u32,
            ) as u32 as u32;
            *(charMask.as_mut_ptr() as *mut libc::c_schar).offset(cur as isize) =
                0 as i32 as libc::c_schar;
            s_1 = s_1.offset(1);
            i_0 = i_0.wrapping_sub(1);
            if !(i_0 != 0) {
                break;
            }
        }
        RangeEnc_Encode(p, sum_0, escFreq, sum_0.wrapping_add(escFreq));
        (*see).Summ = ((*see).Summ as u32)
            .wrapping_add(sum_0)
            .wrapping_add(escFreq) as u16
    }
}
