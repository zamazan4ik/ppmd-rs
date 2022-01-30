use crate::{
    CPpmd8, CPpmd8_Context_, CPpmd_See, CPpmd_State, Ppmd8_MakeEscFreq, Ppmd8_Update1,
    Ppmd8_Update1_0, Ppmd8_Update2, Ppmd8_UpdateBin,
};

extern "C" {
    /* ---------- Internal Functions ---------- */
    static PPMD8_kExpEscape: [u8; 16];
}

pub type Bool = i32;

/* Ppmd.h -- PPMD codec common code
2017-04-03 : Igor Pavlov : Public domain
This code is based on PPMd var.H (2001): Dmitry Shkarin : Public domain */
/* Most compilers works OK here even without #pragma pack(push, 1), but some GCC compilers need it. */
/* SEE-contexts for PPM-contexts with masked symbols */

pub type CPpmd_State_Ref = u32;
pub type CPpmd_Void_Ref = u32;

pub type CPpmd8_Context_Ref = u32;
pub type CPpmd8_Context = CPpmd8_Context_;
/* ---------- Decode ---------- */

unsafe extern "C" fn RangeDec_GetThreshold(mut p: *mut CPpmd8, total: u32) -> u32 {
    (*p).Range = ((*p).Range as u32).wrapping_div(total) as u32;
    return (*p).Code.wrapping_div((*p).Range);
}
unsafe extern "C" fn RangeDec_Decode(mut p: *mut CPpmd8, mut start: u32, size: u32) {
    start = (start as u32).wrapping_mul((*p).Range) as u32;
    (*p).Low = ((*p).Low as u32).wrapping_add(start) as u32;
    (*p).Code = ((*p).Code as u32).wrapping_sub(start) as u32;
    (*p).Range = ((*p).Range as u32).wrapping_mul(size) as u32;
    while (*p).Low ^ (*p).Low.wrapping_add((*p).Range) < ((1 as i32) << 24 as i32) as u32
        || (*p).Range < ((1 as i32) << 15 as i32) as u32 && {
            (*p).Range = (0 as i32 as u32).wrapping_sub((*p).Low)
                & (((1 as i32) << 15 as i32) - 1 as i32) as u32;
            (1 as i32) != 0
        }
    {
        (*p).Code = (*p).Code << 8 as i32
            | (*(*p).Stream.In).Read.expect("non-null function pointer")((*p).Stream.In) as u32;
        (*p).Range <<= 8 as i32;
        (*p).Low <<= 8 as i32
    }
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_DecodeSymbol(mut p: *mut CPpmd8) -> i32 {
    let mut charMask: [u64; 32] = [0; 32];
    if (*(*p).MinContext).NumStats as i32 != 0 as i32 {
        let mut s: *mut CPpmd_State = (*p).Base.offset((*(*p).MinContext).Stats as isize)
            as *mut libc::c_void as *mut CPpmd_State;
        let mut i: u32 = 0;
        let mut count: u32 = 0;
        let mut hiCnt: u32 = 0;
        count = RangeDec_GetThreshold(p, (*(*p).MinContext).SummFreq as u32);
        hiCnt = (*s).Freq as u32;
        if count < hiCnt {
            let mut symbol: u8 = 0;
            RangeDec_Decode(p, 0 as i32 as u32, (*s).Freq as u32);
            (*p).FoundState = s;
            symbol = (*s).Symbol;
            Ppmd8_Update1_0(p);
            return symbol as i32;
        }
        (*p).PrevSuccess = 0 as i32 as u32;
        i = (*(*p).MinContext).NumStats as u32;
        loop {
            s = s.offset(1);
            hiCnt = (hiCnt as u32).wrapping_add((*s).Freq as u32) as u32;
            if hiCnt > count {
                let mut symbol_0: u8 = 0;
                RangeDec_Decode(p, hiCnt.wrapping_sub((*s).Freq as u32), (*s).Freq as u32);
                (*p).FoundState = s;
                symbol_0 = (*s).Symbol;
                Ppmd8_Update1(p as *mut crate::Ppmd8::CPpmd8);
                return symbol_0 as i32;
            }
            i = i.wrapping_sub(1);
            if !(i != 0) {
                break;
            }
        }
        if count >= (*(*p).MinContext).SummFreq as u32 {
            return -(2 as i32);
        }
        RangeDec_Decode(
            p,
            hiCnt,
            ((*(*p).MinContext).SummFreq as u32).wrapping_sub(hiCnt),
        );
        let mut z: u64 = 0;
        while z < (256 as i32 as u64).wrapping_div(::std::mem::size_of::<u64>() as u64) {
            charMask[z.wrapping_add(0) as usize] = !(0 as i32 as u64);
            charMask[z.wrapping_add(1) as usize] = charMask[z.wrapping_add(0) as usize];
            charMask[z.wrapping_add(2) as usize] = charMask[z.wrapping_add(1) as usize];
            charMask[z.wrapping_add(3) as usize] = charMask[z.wrapping_add(2) as usize];
            charMask[z.wrapping_add(4) as usize] = charMask[z.wrapping_add(3) as usize];
            charMask[z.wrapping_add(5) as usize] = charMask[z.wrapping_add(4) as usize];
            charMask[z.wrapping_add(6) as usize] = charMask[z.wrapping_add(5) as usize];
            charMask[z.wrapping_add(7) as usize] = charMask[z.wrapping_add(6) as usize];
            z = z.wrapping_add(8)
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
        (*p).Range >>= 14 as i32;
        if (*p).Code.wrapping_div((*p).Range) < *prob as u32 {
            let mut symbol_1: u8 = 0;
            RangeDec_Decode(p, 0 as i32 as u32, *prob as u32);
            *prob = (*prob as i32 + ((1 as i32) << 7 as i32)
                - (*prob as i32 + ((1 as i32) << 7 as i32 - 2 as i32) >> 7 as i32))
                as u16;
            (*p).FoundState = &mut (*(*p).MinContext).SummFreq as *mut u16 as *mut CPpmd_State;
            symbol_1 = (*(*p).FoundState).Symbol;
            Ppmd8_UpdateBin(p);
            return symbol_1 as i32;
        }
        RangeDec_Decode(
            p,
            *prob as u32,
            (((1 as i32) << 14 as i32) - *prob as i32) as u32,
        );
        *prob = (*prob as i32 - (*prob as i32 + ((1 as i32) << 7 as i32 - 2 as i32) >> 7 as i32))
            as u16;
        (*p).InitEsc = PPMD8_kExpEscape[(*prob as i32 >> 10 as i32) as usize] as u32;
        let mut z_0: u64 = 0;
        while z_0 < (256 as u64).wrapping_div(::std::mem::size_of::<u64>() as u64) {
            charMask[z_0.wrapping_add(0) as usize] = !(0);
            charMask[z_0.wrapping_add(1) as usize] = charMask[z_0.wrapping_add(0) as usize];
            charMask[z_0.wrapping_add(2) as usize] = charMask[z_0.wrapping_add(1) as usize];
            charMask[z_0.wrapping_add(3) as usize] = charMask[z_0.wrapping_add(2) as usize];
            charMask[z_0.wrapping_add(4) as usize] = charMask[z_0.wrapping_add(3) as usize];
            charMask[z_0.wrapping_add(5) as usize] = charMask[z_0.wrapping_add(4) as usize];
            charMask[z_0.wrapping_add(6) as usize] = charMask[z_0.wrapping_add(5) as usize];
            charMask[z_0.wrapping_add(7) as usize] = charMask[z_0.wrapping_add(6) as usize];
            z_0 = z_0.wrapping_add(8)
        }
        *(charMask.as_mut_ptr() as *mut libc::c_schar).offset(
            (*(&mut (*(*p).MinContext).SummFreq as *mut u16 as *mut CPpmd_State)).Symbol as isize,
        ) = 0 as i32 as libc::c_schar;
        (*p).PrevSuccess = 0 as i32 as u32
    }
    loop {
        let mut ps: [*mut CPpmd_State; 256] = [0 as *mut CPpmd_State; 256];
        let mut s_0: *mut CPpmd_State = 0 as *mut CPpmd_State;
        let mut freqSum: u32 = 0;
        let mut count_0: u32 = 0;
        let mut hiCnt_0: u32 = 0;
        let mut see: *mut CPpmd_See = 0 as *mut CPpmd_See;
        let mut i_0: u32 = 0;
        let mut num: u32 = 0;
        let numMasked: u32 = (*(*p).MinContext).NumStats as u32;
        loop {
            (*p).OrderFall = (*p).OrderFall.wrapping_add(1);
            if (*(*p).MinContext).Suffix == 0 {
                return -(1 as i32);
            }
            (*p).MinContext = (*p).Base.offset((*(*p).MinContext).Suffix as isize)
                as *mut libc::c_void as *mut CPpmd8_Context;
            if !((*(*p).MinContext).NumStats as u32 == numMasked) {
                break;
            }
        }
        hiCnt_0 = 0;
        s_0 = (*p).Base.offset((*(*p).MinContext).Stats as isize) as *mut libc::c_void
            as *mut CPpmd_State;
        i_0 = 0 as i32 as u32;
        num = ((*(*p).MinContext).NumStats as u32).wrapping_sub(numMasked);
        loop {
            let k: i32 = *(charMask.as_mut_ptr() as *mut libc::c_schar)
                .offset((*s_0).Symbol as isize) as i32;
            hiCnt_0 = (hiCnt_0 as u32).wrapping_add(((*s_0).Freq as i32 & k) as u32) as u32 as u32;
            let fresh0 = s_0;
            s_0 = s_0.offset(1);
            ps[i_0 as usize] = fresh0;
            i_0 = i_0.wrapping_sub(k as u32);
            if !(i_0 != num) {
                break;
            }
        }
        see = Ppmd8_MakeEscFreq(p, numMasked, &mut freqSum);
        freqSum = (freqSum as u32).wrapping_add(hiCnt_0) as u32 as u32;
        count_0 = RangeDec_GetThreshold(p, freqSum);
        if count_0 < hiCnt_0 {
            let mut symbol_2: u8 = 0;
            let mut pps: *mut *mut CPpmd_State = ps.as_mut_ptr();
            hiCnt_0 = 0 as i32 as u32;
            loop {
                hiCnt_0 = (hiCnt_0 as u32).wrapping_add((**pps).Freq as u32) as u32 as u32;
                if !(hiCnt_0 <= count_0) {
                    break;
                }
                pps = pps.offset(1)
            }
            s_0 = *pps;
            RangeDec_Decode(
                p,
                hiCnt_0.wrapping_sub((*s_0).Freq as u32),
                (*s_0).Freq as u32,
            );
            if ((*see).Shift as i32) < 7 as i32 && {
                (*see).Count = (*see).Count.wrapping_sub(1);
                ((*see).Count as i32) == 0 as i32
            } {
                (*see).Summ = (((*see).Summ as i32) << 1 as i32) as u16;
                let fresh1 = (*see).Shift;
                (*see).Shift = (*see).Shift.wrapping_add(1);
                (*see).Count = ((3 as i32) << fresh1 as i32) as u8
            }
            (*p).FoundState = s_0;
            symbol_2 = (*s_0).Symbol;
            Ppmd8_Update2(p);
            return symbol_2 as i32;
        }
        if count_0 >= freqSum {
            return -(2 as i32);
        }
        RangeDec_Decode(p, hiCnt_0, freqSum.wrapping_sub(hiCnt_0));
        (*see).Summ = ((*see).Summ as u32).wrapping_add(freqSum) as u16;
        loop {
            i_0 = i_0.wrapping_sub(1);
            *(charMask.as_mut_ptr() as *mut libc::c_schar)
                .offset((*ps[i_0 as usize]).Symbol as isize) = 0 as i32 as libc::c_schar;
            if !(i_0 != 0) {
                break;
            }
        }
    }
}
