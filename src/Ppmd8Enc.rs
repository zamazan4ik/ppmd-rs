#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    static PPMD8_kExpEscape: [Byte; 16];
    fn Ppmd8_Update1(p: *mut CPpmd8);
    fn Ppmd8_Update1_0(p: *mut CPpmd8);
    fn Ppmd8_Update2(p: *mut CPpmd8);
    fn Ppmd8_UpdateBin(p: *mut CPpmd8);
    fn Ppmd8_MakeEscFreq(
        p: *mut CPpmd8,
        numMasked: libc::c_uint,
        scale: *mut UInt32,
    ) -> *mut CPpmd_See;
}
pub type size_t = libc::c_ulong;
pub type Byte = libc::c_uchar;
pub type UInt16 = libc::c_ushort;
pub type Int32 = libc::c_int;
pub type UInt32 = libc::c_uint;
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
/* Ppmd.h -- PPMD codec common code
2017-04-03 : Igor Pavlov : Public domain
This code is based on PPMd var.H (2001): Dmitry Shkarin : Public domain */
/* Most compilers works OK here even without #pragma pack(push, 1), but some GCC compilers need it. */
/* SEE-contexts for PPM-contexts with masked symbols */
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
/* returns: -1 as EndMarker, -2 as DataError */
/* ---------- Encode ---------- */
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_RangeEnc_FlushData(mut p: *mut CPpmd8) {
    let mut i: libc::c_uint = 0; /* EndMarker (symbol = -1) */
    i = 0 as libc::c_int as libc::c_uint;
    while i < 4 as libc::c_int as libc::c_uint {
        (*(*p).Stream.Out).Write.expect("non-null function pointer")(
            (*p).Stream.Out,
            ((*p).Low >> 24 as libc::c_int) as Byte,
        );
        i = i.wrapping_add(1);
        (*p).Low <<= 8 as libc::c_int
    }
}
unsafe extern "C" fn RangeEnc_Normalize(mut p: *mut CPpmd8) {
    while (*p).Low ^ (*p).Low.wrapping_add((*p).Range)
        < ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint
        || (*p).Range < ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint && {
            (*p).Range = (0 as libc::c_int as libc::c_uint).wrapping_sub((*p).Low)
                & (((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int) as libc::c_uint;
            (1 as libc::c_int) != 0
        }
    {
        (*(*p).Stream.Out).Write.expect("non-null function pointer")(
            (*p).Stream.Out,
            ((*p).Low >> 24 as libc::c_int) as Byte,
        );
        (*p).Range <<= 8 as libc::c_int;
        (*p).Low <<= 8 as libc::c_int
    }
}
unsafe extern "C" fn RangeEnc_Encode(
    mut p: *mut CPpmd8,
    mut start: UInt32,
    mut size: UInt32,
    mut total: UInt32,
) {
    (*p).Range = ((*p).Range as libc::c_uint).wrapping_div(total) as UInt32 as UInt32;
    (*p).Low =
        ((*p).Low as libc::c_uint).wrapping_add(start.wrapping_mul((*p).Range)) as UInt32 as UInt32;
    (*p).Range = ((*p).Range as libc::c_uint).wrapping_mul(size) as UInt32 as UInt32;
    RangeEnc_Normalize(p);
}
unsafe extern "C" fn RangeEnc_EncodeBit_0(mut p: *mut CPpmd8, mut size0: UInt32) {
    (*p).Range >>= 14 as libc::c_int;
    (*p).Range = ((*p).Range as libc::c_uint).wrapping_mul(size0) as UInt32 as UInt32;
    RangeEnc_Normalize(p);
}
unsafe extern "C" fn RangeEnc_EncodeBit_1(mut p: *mut CPpmd8, mut size0: UInt32) {
    (*p).Range >>= 14 as libc::c_int;
    (*p).Low =
        ((*p).Low as libc::c_uint).wrapping_add(size0.wrapping_mul((*p).Range)) as UInt32 as UInt32;
    (*p).Range = ((*p).Range as libc::c_uint).wrapping_mul(
        (((1 as libc::c_int) << 14 as libc::c_int) as libc::c_uint).wrapping_sub(size0),
    ) as UInt32 as UInt32;
    RangeEnc_Normalize(p);
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_EncodeSymbol(mut p: *mut CPpmd8, mut symbol: libc::c_int) {
    let mut charMask: [size_t; 32] = [0; 32];
    if (*(*p).MinContext).NumStats as libc::c_int != 0 as libc::c_int {
        let mut s: *mut CPpmd_State = (*p).Base.offset((*(*p).MinContext).Stats as isize)
            as *mut libc::c_void as *mut CPpmd_State;
        let mut sum: UInt32 = 0;
        let mut i: libc::c_uint = 0;
        if (*s).Symbol as libc::c_int == symbol {
            RangeEnc_Encode(
                p,
                0 as libc::c_int as UInt32,
                (*s).Freq as UInt32,
                (*(*p).MinContext).SummFreq as UInt32,
            );
            (*p).FoundState = s;
            Ppmd8_Update1_0(p);
            return;
        }
        (*p).PrevSuccess = 0 as libc::c_int as libc::c_uint;
        sum = (*s).Freq as UInt32;
        i = (*(*p).MinContext).NumStats as libc::c_uint;
        loop {
            s = s.offset(1);
            if (*s).Symbol as libc::c_int == symbol {
                RangeEnc_Encode(
                    p,
                    sum,
                    (*s).Freq as UInt32,
                    (*(*p).MinContext).SummFreq as UInt32,
                );
                (*p).FoundState = s;
                Ppmd8_Update1(p);
                return;
            }
            sum = (sum as libc::c_uint).wrapping_add((*s).Freq as libc::c_uint) as UInt32 as UInt32;
            i = i.wrapping_sub(1);
            if !(i != 0) {
                break;
            }
        }
        let mut z: size_t = 0;
        z = 0 as libc::c_int as size_t;
        while z
            < (256 as libc::c_int as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<size_t>() as libc::c_ulong)
        {
            charMask[z.wrapping_add(0 as libc::c_int as libc::c_ulong) as usize] =
                !(0 as libc::c_int as size_t);
            charMask[z.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(0 as libc::c_int as libc::c_ulong) as usize];
            charMask[z.wrapping_add(2 as libc::c_int as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize];
            charMask[z.wrapping_add(3 as libc::c_int as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(2 as libc::c_int as libc::c_ulong) as usize];
            charMask[z.wrapping_add(4 as libc::c_int as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(3 as libc::c_int as libc::c_ulong) as usize];
            charMask[z.wrapping_add(5 as libc::c_int as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(4 as libc::c_int as libc::c_ulong) as usize];
            charMask[z.wrapping_add(6 as libc::c_int as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(5 as libc::c_int as libc::c_ulong) as usize];
            charMask[z.wrapping_add(7 as libc::c_int as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(6 as libc::c_int as libc::c_ulong) as usize];
            z = (z as libc::c_ulong).wrapping_add(8 as libc::c_int as libc::c_ulong) as size_t
                as size_t
        }
        *(charMask.as_mut_ptr() as *mut libc::c_schar).offset((*s).Symbol as isize) =
            0 as libc::c_int as libc::c_schar;
        i = (*(*p).MinContext).NumStats as libc::c_uint;
        loop {
            s = s.offset(-1);
            *(charMask.as_mut_ptr() as *mut libc::c_schar).offset((*s).Symbol as isize) =
                0 as libc::c_int as libc::c_schar;
            i = i.wrapping_sub(1);
            if !(i != 0) {
                break;
            }
        }
        RangeEnc_Encode(
            p,
            sum,
            ((*(*p).MinContext).SummFreq as libc::c_uint).wrapping_sub(sum),
            (*(*p).MinContext).SummFreq as UInt32,
        );
    } else {
        let mut prob: *mut UInt16 = &mut *(*(*p).BinSumm.as_mut_ptr().offset(
            *(*p).NS2Indx.as_mut_ptr().offset(
                ((*(&mut (*(*p).MinContext).SummFreq as *mut UInt16 as *mut CPpmd_State)).Freq
                    as size_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as isize,
        ))
        .as_mut_ptr()
        .offset(
            (*(*p).NS2BSIndx.as_mut_ptr().offset(
                (*((*p).Base.offset((*(*p).MinContext).Suffix as isize) as *mut libc::c_void
                    as *mut CPpmd8_Context))
                    .NumStats as isize,
            ) as libc::c_uint)
                .wrapping_add((*p).PrevSuccess)
                .wrapping_add((*(*p).MinContext).Flags as libc::c_uint)
                .wrapping_add(
                    ((*p).RunLength >> 26 as libc::c_int & 0x20 as libc::c_int) as libc::c_uint,
                ) as isize,
        ) as *mut UInt16;
        let mut s_0: *mut CPpmd_State =
            &mut (*(*p).MinContext).SummFreq as *mut UInt16 as *mut CPpmd_State;
        if (*s_0).Symbol as libc::c_int == symbol {
            RangeEnc_EncodeBit_0(p, *prob as UInt32);
            *prob = (*prob as libc::c_int + ((1 as libc::c_int) << 7 as libc::c_int)
                - (*prob as libc::c_int
                    + ((1 as libc::c_int) << 7 as libc::c_int - 2 as libc::c_int)
                    >> 7 as libc::c_int)) as UInt16;
            (*p).FoundState = s_0;
            Ppmd8_UpdateBin(p);
            return;
        } else {
            RangeEnc_EncodeBit_1(p, *prob as UInt32);
            *prob = (*prob as libc::c_int
                - (*prob as libc::c_int
                    + ((1 as libc::c_int) << 7 as libc::c_int - 2 as libc::c_int)
                    >> 7 as libc::c_int)) as UInt16;
            (*p).InitEsc = PPMD8_kExpEscape[(*prob as libc::c_int >> 10 as libc::c_int) as usize]
                as libc::c_uint;
            let mut z_0: size_t = 0;
            z_0 = 0 as libc::c_int as size_t;
            while z_0
                < (256 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<size_t>() as libc::c_ulong)
            {
                charMask[z_0.wrapping_add(0 as libc::c_int as libc::c_ulong) as usize] =
                    !(0 as libc::c_int as size_t);
                charMask[z_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize] =
                    charMask[z_0.wrapping_add(0 as libc::c_int as libc::c_ulong) as usize];
                charMask[z_0.wrapping_add(2 as libc::c_int as libc::c_ulong) as usize] =
                    charMask[z_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize];
                charMask[z_0.wrapping_add(3 as libc::c_int as libc::c_ulong) as usize] =
                    charMask[z_0.wrapping_add(2 as libc::c_int as libc::c_ulong) as usize];
                charMask[z_0.wrapping_add(4 as libc::c_int as libc::c_ulong) as usize] =
                    charMask[z_0.wrapping_add(3 as libc::c_int as libc::c_ulong) as usize];
                charMask[z_0.wrapping_add(5 as libc::c_int as libc::c_ulong) as usize] =
                    charMask[z_0.wrapping_add(4 as libc::c_int as libc::c_ulong) as usize];
                charMask[z_0.wrapping_add(6 as libc::c_int as libc::c_ulong) as usize] =
                    charMask[z_0.wrapping_add(5 as libc::c_int as libc::c_ulong) as usize];
                charMask[z_0.wrapping_add(7 as libc::c_int as libc::c_ulong) as usize] =
                    charMask[z_0.wrapping_add(6 as libc::c_int as libc::c_ulong) as usize];
                z_0 = (z_0 as libc::c_ulong).wrapping_add(8 as libc::c_int as libc::c_ulong)
                    as size_t as size_t
            }
            *(charMask.as_mut_ptr() as *mut libc::c_schar).offset((*s_0).Symbol as isize) =
                0 as libc::c_int as libc::c_schar;
            (*p).PrevSuccess = 0 as libc::c_int as libc::c_uint
        }
    }
    loop {
        let mut escFreq: UInt32 = 0;
        let mut see: *mut CPpmd_See = 0 as *mut CPpmd_See;
        let mut s_1: *mut CPpmd_State = 0 as *mut CPpmd_State;
        let mut sum_0: UInt32 = 0;
        let mut i_0: libc::c_uint = 0;
        let mut numMasked: libc::c_uint = (*(*p).MinContext).NumStats as libc::c_uint;
        loop {
            (*p).OrderFall = (*p).OrderFall.wrapping_add(1);
            if (*(*p).MinContext).Suffix == 0 {
                return;
            }
            (*p).MinContext = (*p).Base.offset((*(*p).MinContext).Suffix as isize)
                as *mut libc::c_void as *mut CPpmd8_Context;
            if !((*(*p).MinContext).NumStats as libc::c_uint == numMasked) {
                break;
            }
        }
        see = Ppmd8_MakeEscFreq(p, numMasked, &mut escFreq);
        s_1 = (*p).Base.offset((*(*p).MinContext).Stats as isize) as *mut libc::c_void
            as *mut CPpmd_State;
        sum_0 = 0 as libc::c_int as UInt32;
        i_0 = ((*(*p).MinContext).NumStats as libc::c_int + 1 as libc::c_int) as libc::c_uint;
        loop {
            let mut cur: libc::c_int = (*s_1).Symbol as libc::c_int;
            if cur == symbol {
                let mut low: UInt32 = sum_0;
                let mut s1: *mut CPpmd_State = s_1;
                loop {
                    sum_0 = (sum_0 as libc::c_uint).wrapping_add(
                        ((*s_1).Freq as libc::c_int
                            & *(charMask.as_mut_ptr() as *mut libc::c_schar)
                                .offset((*s_1).Symbol as isize)
                                as libc::c_int) as libc::c_uint,
                    ) as UInt32 as UInt32;
                    s_1 = s_1.offset(1);
                    i_0 = i_0.wrapping_sub(1);
                    if !(i_0 != 0) {
                        break;
                    }
                }
                RangeEnc_Encode(p, low, (*s1).Freq as UInt32, sum_0.wrapping_add(escFreq));
                if ((*see).Shift as libc::c_int) < 7 as libc::c_int && {
                    (*see).Count = (*see).Count.wrapping_sub(1);
                    ((*see).Count as libc::c_int) == 0 as libc::c_int
                } {
                    (*see).Summ = (((*see).Summ as libc::c_int) << 1 as libc::c_int) as UInt16;
                    let fresh0 = (*see).Shift;
                    (*see).Shift = (*see).Shift.wrapping_add(1);
                    (*see).Count = ((3 as libc::c_int) << fresh0 as libc::c_int) as Byte
                }
                (*p).FoundState = s1;
                Ppmd8_Update2(p);
                return;
            }
            sum_0 = (sum_0 as libc::c_uint).wrapping_add(
                ((*s_1).Freq as libc::c_int
                    & *(charMask.as_mut_ptr() as *mut libc::c_schar).offset(cur as isize)
                        as libc::c_int) as libc::c_uint,
            ) as UInt32 as UInt32;
            *(charMask.as_mut_ptr() as *mut libc::c_schar).offset(cur as isize) =
                0 as libc::c_int as libc::c_schar;
            s_1 = s_1.offset(1);
            i_0 = i_0.wrapping_sub(1);
            if !(i_0 != 0) {
                break;
            }
        }
        RangeEnc_Encode(p, sum_0, escFreq, sum_0.wrapping_add(escFreq));
        (*see).Summ = ((*see).Summ as libc::c_uint)
            .wrapping_add(sum_0)
            .wrapping_add(escFreq) as UInt16
    }
}
