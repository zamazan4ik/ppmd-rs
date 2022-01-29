#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use libc::{free, malloc};

extern "C" {
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
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
/* Returns: result. (result != SZ_OK) means break.
Value (UInt64)(Int64)-1 for size means unknown value. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ISzAlloc {
    pub Alloc: Option<unsafe extern "C" fn(_: ISzAllocPtr, _: size_t) -> *mut libc::c_void>,
    pub Free: Option<unsafe extern "C" fn(_: ISzAllocPtr, _: *mut libc::c_void) -> ()>,
}
pub type ISzAllocPtr = *const ISzAlloc;
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
pub type CPpmd_Byte_Ref = UInt32;
/* Ppmd8.h -- PPMdI codec
2017-04-03 : Igor Pavlov : Public domain
This code is based on:
  PPMd var.I (2002): Dmitry Shkarin : Public domain
  Carryless rangecoder (1999): Dmitry Subbotin : Public domain */
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
pub type C2RustUnnamed = libc::c_uint;
pub const PPMD8_RESTORE_METHOD_CUT_OFF: C2RustUnnamed = 1;
pub const PPMD8_RESTORE_METHOD_RESTART: C2RustUnnamed = 0;
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
    pub Stream: C2RustUnnamed_0,
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
pub union C2RustUnnamed_0 {
    pub In: *mut IByteIn,
    pub Out: *mut IByteOut,
}
pub type CTX_PTR = *mut CPpmd8_Context;
pub type CPpmd8_Node = CPpmd8_Node_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd8_Node_ {
    pub Stamp: UInt32,
    pub Next: CPpmd8_Node_Ref,
    pub NU: UInt32,
}
pub type CPpmd8_Node_Ref = UInt32;
/* Ppmd8.c -- PPMdI codec
2017-04-03 : Igor Pavlov : Public domain
This code is based on PPMd var.I (2002): Dmitry Shkarin : Public domain */
#[no_mangle]
pub static mut PPMD8_kExpEscape: [Byte; 16] = [
    25 as libc::c_int as Byte,
    14 as libc::c_int as Byte,
    9 as libc::c_int as Byte,
    7 as libc::c_int as Byte,
    5 as libc::c_int as Byte,
    5 as libc::c_int as Byte,
    4 as libc::c_int as Byte,
    4 as libc::c_int as Byte,
    4 as libc::c_int as Byte,
    3 as libc::c_int as Byte,
    3 as libc::c_int as Byte,
    3 as libc::c_int as Byte,
    2 as libc::c_int as Byte,
    2 as libc::c_int as Byte,
    2 as libc::c_int as Byte,
    2 as libc::c_int as Byte,
];
static mut kInitBinEsc: [UInt16; 8] = [
    0x3cdd as libc::c_int as UInt16,
    0x1f3f as libc::c_int as UInt16,
    0x59bf as libc::c_int as UInt16,
    0x48f3 as libc::c_int as UInt16,
    0x64a1 as libc::c_int as UInt16,
    0x5abc as libc::c_int as UInt16,
    0x6632 as libc::c_int as UInt16,
    0x6051 as libc::c_int as UInt16,
];
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Construct(mut p: *mut CPpmd8) {
    let mut i: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut m: libc::c_uint = 0;
    (*p).Base = 0 as *mut Byte;
    i = 0 as libc::c_int as libc::c_uint;
    k = 0 as libc::c_int as libc::c_uint;
    while i
        < (4 as libc::c_int
            + 4 as libc::c_int
            + 4 as libc::c_int
            + (128 as libc::c_int + 3 as libc::c_int
                - 1 as libc::c_int * 4 as libc::c_int
                - 2 as libc::c_int * 4 as libc::c_int
                - 3 as libc::c_int * 4 as libc::c_int)
                / 4 as libc::c_int) as libc::c_uint
    {
        let mut step: libc::c_uint = if i >= 12 as libc::c_int as libc::c_uint {
            4 as libc::c_int as libc::c_uint
        } else {
            (i >> 2 as libc::c_int).wrapping_add(1 as libc::c_int as libc::c_uint)
        };
        loop {
            let fresh0 = k;
            k = k.wrapping_add(1);
            (*p).Units2Indx[fresh0 as usize] = i as Byte;
            step = step.wrapping_sub(1);
            if !(step != 0) {
                break;
            }
        }
        (*p).Indx2Units[i as usize] = k as Byte;
        i = i.wrapping_add(1)
    }
    (*p).NS2BSIndx[0 as libc::c_int as usize] = ((0 as libc::c_int) << 1 as libc::c_int) as Byte;
    (*p).NS2BSIndx[1 as libc::c_int as usize] = ((1 as libc::c_int) << 1 as libc::c_int) as Byte;
    memset(
        (*p).NS2BSIndx
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize) as *mut libc::c_void,
        (2 as libc::c_int) << 1 as libc::c_int,
        9 as libc::c_int as libc::c_ulong,
    );
    memset(
        (*p).NS2BSIndx
            .as_mut_ptr()
            .offset(11 as libc::c_int as isize) as *mut libc::c_void,
        (3 as libc::c_int) << 1 as libc::c_int,
        (256 as libc::c_int - 11 as libc::c_int) as libc::c_ulong,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < 5 as libc::c_int as libc::c_uint {
        (*p).NS2Indx[i as usize] = i as Byte;
        i = i.wrapping_add(1)
    }
    m = i;
    k = 1 as libc::c_int as libc::c_uint;
    while i < 260 as libc::c_int as libc::c_uint {
        (*p).NS2Indx[i as usize] = m as Byte;
        k = k.wrapping_sub(1);
        if k == 0 as libc::c_int as libc::c_uint {
            m = m.wrapping_add(1);
            k = m.wrapping_sub(4 as libc::c_int as libc::c_uint)
        }
        i = i.wrapping_add(1)
    }
}
/* The BUG in Shkarin's code for FREEZE mode was fixed, but that fixed
code is not compatible with original code for some files compressed
in FREEZE mode. So we disable FREEZE mode support. */
/* must be 32-bit at least */
/* Range Coder */
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Free(mut p: *mut CPpmd8, mut alloc: ISzAllocPtr) {
    (*alloc).Free.expect("non-null function pointer")(alloc, (*p).Base as *mut libc::c_void);
    (*p).Size = 0 as libc::c_int as UInt32;
    (*p).Base = 0 as *mut Byte;
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Alloc(
    mut p: *mut CPpmd8,
    mut size: UInt32,
    mut alloc: ISzAllocPtr,
) -> Bool {
    if (*p).Base.is_null() || (*p).Size != size {
        Ppmd8_Free(p, alloc);
        (*p).AlignOffset = (4 as libc::c_int as libc::c_uint)
            .wrapping_sub(size & 3 as libc::c_int as libc::c_uint);
        (*p).Base = (*alloc).Alloc.expect("non-null function pointer")(
            alloc,
            (*p).AlignOffset.wrapping_add(size) as size_t,
        ) as *mut Byte;
        if (*p).Base.is_null() {
            return 0 as libc::c_int;
        }
        (*p).Size = size
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn InsertNode(
    mut p: *mut CPpmd8,
    mut node: *mut libc::c_void,
    mut indx: libc::c_uint,
) {
    (*(node as *mut CPpmd8_Node)).Stamp = 0xffffffff as libc::c_uint;
    (*(node as *mut CPpmd8_Node)).Next = (*p).FreeList[indx as usize];
    (*(node as *mut CPpmd8_Node)).NU = (*p).Indx2Units[indx as usize] as UInt32;
    (*p).FreeList[indx as usize] =
        (node as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
    (*p).Stamps[indx as usize] = (*p).Stamps[indx as usize].wrapping_add(1);
}
unsafe extern "C" fn RemoveNode(mut p: *mut CPpmd8, mut indx: libc::c_uint) -> *mut libc::c_void {
    let mut node: *mut CPpmd8_Node =
        (*p).Base.offset((*p).FreeList[indx as usize] as isize) as *mut CPpmd8_Node;
    (*p).FreeList[indx as usize] = (*node).Next;
    (*p).Stamps[indx as usize] = (*p).Stamps[indx as usize].wrapping_sub(1);
    return node as *mut libc::c_void;
}
unsafe extern "C" fn SplitBlock(
    mut p: *mut CPpmd8,
    mut ptr: *mut libc::c_void,
    mut oldIndx: libc::c_uint,
    mut newIndx: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    let mut nu: libc::c_uint = ((*p).Indx2Units[oldIndx as usize] as libc::c_int
        - (*p).Indx2Units[newIndx as usize] as libc::c_int)
        as libc::c_uint;
    ptr = (ptr as *mut Byte).offset(
        ((*p).Indx2Units[newIndx as usize] as UInt32)
            .wrapping_mul(12 as libc::c_int as libc::c_uint) as isize,
    ) as *mut libc::c_void;
    i = (*p).Units2Indx[(nu as size_t).wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
        as libc::c_uint;
    if (*p).Indx2Units[i as usize] as libc::c_uint != nu {
        i = i.wrapping_sub(1);
        let mut k: libc::c_uint = (*p).Indx2Units[i as usize] as libc::c_uint;
        InsertNode(
            p,
            (ptr as *mut Byte).offset(k.wrapping_mul(12 as libc::c_int as libc::c_uint) as isize)
                as *mut libc::c_void,
            nu.wrapping_sub(k)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    InsertNode(p, ptr, i);
}
unsafe extern "C" fn GlueFreeBlocks(mut p: *mut CPpmd8) {
    let mut head: CPpmd8_Node_Ref = 0 as libc::c_int as CPpmd8_Node_Ref;
    let mut prev: *mut CPpmd8_Node_Ref = &mut head;
    let mut i: libc::c_uint = 0;
    (*p).GlueCount = ((1 as libc::c_int) << 13 as libc::c_int) as UInt32;
    memset(
        (*p).Stamps.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[UInt32; 38]>() as libc::c_ulong,
    );
    /* Order-0 context is always at top UNIT, so we don't need guard NODE at the end.
    All blocks up to p->LoUnit can be free, so we need guard NODE at LoUnit. */
    if (*p).LoUnit != (*p).HiUnit {
        (*((*p).LoUnit as *mut CPpmd8_Node)).Stamp = 0 as libc::c_int as UInt32
    }
    /* Glue free blocks */
    i = 0 as libc::c_int as libc::c_uint;
    while i
        < (4 as libc::c_int
            + 4 as libc::c_int
            + 4 as libc::c_int
            + (128 as libc::c_int + 3 as libc::c_int
                - 1 as libc::c_int * 4 as libc::c_int
                - 2 as libc::c_int * 4 as libc::c_int
                - 3 as libc::c_int * 4 as libc::c_int)
                / 4 as libc::c_int) as libc::c_uint
    {
        let mut next: CPpmd8_Node_Ref = (*p).FreeList[i as usize];
        (*p).FreeList[i as usize] = 0 as libc::c_int as CPpmd_Void_Ref;
        while next != 0 as libc::c_int as libc::c_uint {
            let mut node: *mut CPpmd8_Node = (*p).Base.offset(next as isize) as *mut CPpmd8_Node;
            if (*node).NU != 0 as libc::c_int as libc::c_uint {
                let mut node2: *mut CPpmd8_Node = 0 as *mut CPpmd8_Node;
                *prev = next;
                prev = &mut (*node).Next;
                loop {
                    node2 = node.offset((*node).NU as isize);
                    if !((*node2).Stamp == 0xffffffff as libc::c_uint) {
                        break;
                    }
                    (*node).NU =
                        ((*node).NU as libc::c_uint).wrapping_add((*node2).NU) as UInt32 as UInt32;
                    (*node2).NU = 0 as libc::c_int as UInt32
                }
            }
            next = (*node).Next
        }
        i = i.wrapping_add(1)
    }
    *prev = 0 as libc::c_int as CPpmd8_Node_Ref;
    /* Fill lists of free blocks */
    while head != 0 as libc::c_int as libc::c_uint {
        let mut node_0: *mut CPpmd8_Node = (*p).Base.offset(head as isize) as *mut CPpmd8_Node; /* AllocContext(p); */
        let mut nu: libc::c_uint = 0; /* AllocUnits(p, PPMD_NUM_INDEXES - 1); */
        head = (*node_0).Next; /* unused */
        nu = (*node_0).NU;
        if nu == 0 as libc::c_int as libc::c_uint {
            continue;
        }
        while nu > 128 as libc::c_int as libc::c_uint {
            InsertNode(
                p,
                node_0 as *mut libc::c_void,
                (4 as libc::c_int
                    + 4 as libc::c_int
                    + 4 as libc::c_int
                    + (128 as libc::c_int + 3 as libc::c_int
                        - 1 as libc::c_int * 4 as libc::c_int
                        - 2 as libc::c_int * 4 as libc::c_int
                        - 3 as libc::c_int * 4 as libc::c_int)
                        / 4 as libc::c_int
                    - 1 as libc::c_int) as libc::c_uint,
            );
            nu = nu.wrapping_sub(128 as libc::c_int as libc::c_uint);
            node_0 = node_0.offset(128 as libc::c_int as isize)
        }
        i = (*p).Units2Indx[(nu as size_t).wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
            as libc::c_uint;
        if (*p).Indx2Units[i as usize] as libc::c_uint != nu {
            i = i.wrapping_sub(1);
            let mut k: libc::c_uint = (*p).Indx2Units[i as usize] as libc::c_uint;
            InsertNode(
                p,
                node_0.offset(k as isize) as *mut libc::c_void,
                nu.wrapping_sub(k)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            );
        }
        InsertNode(p, node_0 as *mut libc::c_void, i);
    }
}
unsafe extern "C" fn AllocUnitsRare(
    mut p: *mut CPpmd8,
    mut indx: libc::c_uint,
) -> *mut libc::c_void {
    let mut i: libc::c_uint = 0;
    let mut retVal: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*p).GlueCount == 0 as libc::c_int as libc::c_uint {
        GlueFreeBlocks(p);
        if (*p).FreeList[indx as usize] != 0 as libc::c_int as libc::c_uint {
            return RemoveNode(p, indx);
        }
    }
    i = indx;
    loop {
        i = i.wrapping_add(1);
        if i == (4 as libc::c_int
            + 4 as libc::c_int
            + 4 as libc::c_int
            + (128 as libc::c_int + 3 as libc::c_int
                - 1 as libc::c_int * 4 as libc::c_int
                - 2 as libc::c_int * 4 as libc::c_int
                - 3 as libc::c_int * 4 as libc::c_int)
                / 4 as libc::c_int) as libc::c_uint
        {
            let mut numBytes: UInt32 = ((*p).Indx2Units[indx as usize] as UInt32)
                .wrapping_mul(12 as libc::c_int as libc::c_uint);
            (*p).GlueCount = (*p).GlueCount.wrapping_sub(1);
            return if (*p).UnitsStart.offset_from((*p).Text) as libc::c_long as UInt32 > numBytes {
                (*p).UnitsStart = (*p).UnitsStart.offset(-(numBytes as isize));
                (*p).UnitsStart
            } else {
                0 as *mut Byte
            } as *mut libc::c_void;
        }
        if !((*p).FreeList[i as usize] == 0 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    retVal = RemoveNode(p, i);
    SplitBlock(p, retVal, i, indx);
    return retVal;
}
unsafe extern "C" fn AllocUnits(mut p: *mut CPpmd8, mut indx: libc::c_uint) -> *mut libc::c_void {
    let mut numBytes: UInt32 = 0;
    if (*p).FreeList[indx as usize] != 0 as libc::c_int as libc::c_uint {
        return RemoveNode(p, indx);
    }
    numBytes =
        ((*p).Indx2Units[indx as usize] as UInt32).wrapping_mul(12 as libc::c_int as libc::c_uint);
    if numBytes <= (*p).HiUnit.offset_from((*p).LoUnit) as libc::c_long as UInt32 {
        let mut retVal: *mut libc::c_void = (*p).LoUnit as *mut libc::c_void;
        (*p).LoUnit = (*p).LoUnit.offset(numBytes as isize);
        return retVal;
    }
    return AllocUnitsRare(p, indx);
}
unsafe extern "C" fn ShrinkUnits(
    mut p: *mut CPpmd8,
    mut oldPtr: *mut libc::c_void,
    mut oldNU: libc::c_uint,
    mut newNU: libc::c_uint,
) -> *mut libc::c_void {
    let mut i0: libc::c_uint = (*p).Units2Indx
        [(oldNU as size_t).wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
        as libc::c_uint;
    let mut i1: libc::c_uint = (*p).Units2Indx
        [(newNU as size_t).wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
        as libc::c_uint;
    if i0 == i1 {
        return oldPtr;
    }
    if (*p).FreeList[i1 as usize] != 0 as libc::c_int as libc::c_uint {
        let mut ptr: *mut libc::c_void = RemoveNode(p, i1);
        let mut d: *mut UInt32 = ptr as *mut UInt32;
        let mut z: *const UInt32 = oldPtr as *const UInt32;
        let mut n: UInt32 = newNU;
        loop {
            *d.offset(0 as libc::c_int as isize) = *z.offset(0 as libc::c_int as isize);
            *d.offset(1 as libc::c_int as isize) = *z.offset(1 as libc::c_int as isize);
            *d.offset(2 as libc::c_int as isize) = *z.offset(2 as libc::c_int as isize);
            z = z.offset(3 as libc::c_int as isize);
            d = d.offset(3 as libc::c_int as isize);
            n = n.wrapping_sub(1);
            if !(n != 0) {
                break;
            }
        }
        InsertNode(p, oldPtr, i0);
        return ptr;
    }
    SplitBlock(p, oldPtr, i0, i1);
    return oldPtr;
}
unsafe extern "C" fn FreeUnits(
    mut p: *mut CPpmd8,
    mut ptr: *mut libc::c_void,
    mut nu: libc::c_uint,
) {
    InsertNode(
        p,
        ptr,
        (*p).Units2Indx[(nu as size_t).wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
            as libc::c_uint,
    );
}
unsafe extern "C" fn SpecialFreeUnit(mut p: *mut CPpmd8, mut ptr: *mut libc::c_void) {
    if ptr as *mut Byte != (*p).UnitsStart {
        InsertNode(p, ptr, 0 as libc::c_int as libc::c_uint);
    } else {
        (*p).UnitsStart = (*p).UnitsStart.offset(12 as libc::c_int as isize)
    };
}
unsafe extern "C" fn MoveUnitsUp(
    mut p: *mut CPpmd8,
    mut oldPtr: *mut libc::c_void,
    mut nu: libc::c_uint,
) -> *mut libc::c_void {
    let mut indx: libc::c_uint = (*p).Units2Indx
        [(nu as size_t).wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
        as libc::c_uint;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if oldPtr as *mut Byte
        > (*p)
            .UnitsStart
            .offset((16 as libc::c_int * 1024 as libc::c_int) as isize)
        || (oldPtr as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32
            > (*p).FreeList[indx as usize]
    {
        return oldPtr;
    }
    ptr = RemoveNode(p, indx);
    let mut d: *mut UInt32 = ptr as *mut UInt32;
    let mut z: *const UInt32 = oldPtr as *const UInt32;
    let mut n: UInt32 = nu;
    loop {
        *d.offset(0 as libc::c_int as isize) = *z.offset(0 as libc::c_int as isize);
        *d.offset(1 as libc::c_int as isize) = *z.offset(1 as libc::c_int as isize);
        *d.offset(2 as libc::c_int as isize) = *z.offset(2 as libc::c_int as isize);
        z = z.offset(3 as libc::c_int as isize);
        d = d.offset(3 as libc::c_int as isize);
        n = n.wrapping_sub(1);
        if !(n != 0) {
            break;
        }
    }
    if oldPtr as *mut Byte != (*p).UnitsStart {
        InsertNode(p, oldPtr, indx);
    } else {
        (*p).UnitsStart = (*p).UnitsStart.offset(
            ((*p).Indx2Units[indx as usize] as UInt32)
                .wrapping_mul(12 as libc::c_int as libc::c_uint) as isize,
        )
    }
    return ptr;
}
unsafe extern "C" fn ExpandTextArea(mut p: *mut CPpmd8) {
    let mut count: [UInt32; 38] = [0; 38];
    let mut i: libc::c_uint = 0;
    memset(
        count.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[UInt32; 38]>() as libc::c_ulong,
    );
    if (*p).LoUnit != (*p).HiUnit {
        (*((*p).LoUnit as *mut CPpmd8_Node)).Stamp = 0 as libc::c_int as UInt32
    }
    let mut node: *mut CPpmd8_Node = (*p).UnitsStart as *mut CPpmd8_Node;
    while (*node).Stamp == 0xffffffff as libc::c_uint {
        (*node).Stamp = 0 as libc::c_int as UInt32;
        count[(*p).Units2Indx
            [((*node).NU as size_t).wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
            as usize] = count[(*p).Units2Indx
            [((*node).NU as size_t).wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
            as usize]
            .wrapping_add(1);
        node = node.offset((*node).NU as isize)
    }
    (*p).UnitsStart = node as *mut Byte;
    i = 0 as libc::c_int as libc::c_uint;
    while i
        < (4 as libc::c_int
            + 4 as libc::c_int
            + 4 as libc::c_int
            + (128 as libc::c_int + 3 as libc::c_int
                - 1 as libc::c_int * 4 as libc::c_int
                - 2 as libc::c_int * 4 as libc::c_int
                - 3 as libc::c_int * 4 as libc::c_int)
                / 4 as libc::c_int) as libc::c_uint
    {
        let mut next: *mut CPpmd8_Node_Ref = &mut *(*p).FreeList.as_mut_ptr().offset(i as isize)
            as *mut CPpmd_Void_Ref
            as *mut CPpmd8_Node_Ref;
        while count[i as usize] != 0 as libc::c_int as libc::c_uint {
            let mut node_0: *mut CPpmd8_Node = (*p).Base.offset(*next as isize) as *mut CPpmd8_Node;
            while (*node_0).Stamp == 0 as libc::c_int as libc::c_uint {
                *next = (*node_0).Next;
                node_0 = (*p).Base.offset(*next as isize) as *mut CPpmd8_Node;
                (*p).Stamps[i as usize] = (*p).Stamps[i as usize].wrapping_sub(1);
                count[i as usize] = count[i as usize].wrapping_sub(1);
                if count[i as usize] == 0 as libc::c_int as libc::c_uint {
                    break;
                }
            }
            next = &mut (*node_0).Next
        }
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn SetSuccessor(mut p: *mut CPpmd_State, mut v: CPpmd_Void_Ref) {
    (*p).SuccessorLow = (v & 0xffff as libc::c_int as libc::c_uint) as UInt16;
    (*p).SuccessorHigh = (v >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as UInt16;
}
unsafe extern "C" fn RestartModel(mut p: *mut CPpmd8) {
    let mut i: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut m: libc::c_uint = 0;
    let mut r: libc::c_uint = 0;
    memset(
        (*p).FreeList.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[CPpmd_Void_Ref; 38]>() as libc::c_ulong,
    );
    memset(
        (*p).Stamps.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[UInt32; 38]>() as libc::c_ulong,
    );
    (*p).Text = (*p)
        .Base
        .offset((*p).AlignOffset as isize)
        .offset(0 as libc::c_int as isize);
    (*p).HiUnit = (*p).Text.offset((*p).Size as isize);
    (*p).UnitsStart = (*p).HiUnit.offset(
        -((*p)
            .Size
            .wrapping_div(8 as libc::c_int as libc::c_uint)
            .wrapping_div(12 as libc::c_int as libc::c_uint)
            .wrapping_mul(7 as libc::c_int as libc::c_uint)
            .wrapping_mul(12 as libc::c_int as libc::c_uint) as isize),
    );
    (*p).LoUnit = (*p).UnitsStart;
    (*p).GlueCount = 0 as libc::c_int as UInt32;
    (*p).OrderFall = (*p).MaxOrder;
    (*p).InitRL = -((if (*p).MaxOrder < 12 as libc::c_int as libc::c_uint {
        (*p).MaxOrder
    } else {
        12 as libc::c_int as libc::c_uint
    }) as Int32)
        - 1 as libc::c_int;
    (*p).RunLength = (*p).InitRL;
    (*p).PrevSuccess = 0 as libc::c_int as libc::c_uint;
    (*p).HiUnit = (*p).HiUnit.offset(-(12 as libc::c_int as isize));
    (*p).MaxContext = (*p).HiUnit as CTX_PTR;
    (*p).MinContext = (*p).MaxContext;
    (*(*p).MinContext).Suffix = 0 as libc::c_int as CPpmd8_Context_Ref;
    (*(*p).MinContext).NumStats = 255 as libc::c_int as Byte;
    (*(*p).MinContext).Flags = 0 as libc::c_int as Byte;
    (*(*p).MinContext).SummFreq = (256 as libc::c_int + 1 as libc::c_int) as UInt16;
    (*p).FoundState = (*p).LoUnit as *mut CPpmd_State;
    (*p).LoUnit = (*p).LoUnit.offset(
        ((256 as libc::c_int / 2 as libc::c_int) as UInt32)
            .wrapping_mul(12 as libc::c_int as libc::c_uint) as isize,
    );
    (*(*p).MinContext).Stats =
        ((*p).FoundState as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 256 as libc::c_int as libc::c_uint {
        let mut s: *mut CPpmd_State = &mut *(*p).FoundState.offset(i as isize) as *mut CPpmd_State;
        (*s).Symbol = i as Byte;
        (*s).Freq = 1 as libc::c_int as Byte;
        SetSuccessor(s, 0 as libc::c_int as CPpmd_Void_Ref);
        i = i.wrapping_add(1)
    }
    m = 0 as libc::c_int as libc::c_uint;
    i = m;
    while m < 25 as libc::c_int as libc::c_uint {
        while (*p).NS2Indx[i as usize] as libc::c_uint == m {
            i = i.wrapping_add(1)
        }
        k = 0 as libc::c_int as libc::c_uint;
        while k < 8 as libc::c_int as libc::c_uint {
            let mut val: UInt16 = (((1 as libc::c_int) << 7 as libc::c_int + 7 as libc::c_int)
                as libc::c_uint)
                .wrapping_sub(
                    (kInitBinEsc[k as usize] as libc::c_uint)
                        .wrapping_div(i.wrapping_add(1 as libc::c_int as libc::c_uint)),
                ) as UInt16;
            let mut dest: *mut UInt16 = (*p).BinSumm[m as usize].as_mut_ptr().offset(k as isize);
            r = 0 as libc::c_int as libc::c_uint;
            while r < 64 as libc::c_int as libc::c_uint {
                *dest.offset(r as isize) = val;
                r = r.wrapping_add(8 as libc::c_int as libc::c_uint)
            }
            k = k.wrapping_add(1)
        }
        m = m.wrapping_add(1)
    }
    m = 0 as libc::c_int as libc::c_uint;
    i = m;
    while m < 24 as libc::c_int as libc::c_uint {
        while (*p).NS2Indx[(i as size_t).wrapping_add(3 as libc::c_int as libc::c_ulong) as usize]
            as libc::c_uint
            == m.wrapping_add(3 as libc::c_int as libc::c_uint)
        {
            i = i.wrapping_add(1)
        }
        k = 0 as libc::c_int as libc::c_uint;
        while k < 32 as libc::c_int as libc::c_uint {
            let mut s_0: *mut CPpmd_See = &mut *(*(*p).See.as_mut_ptr().offset(m as isize))
                .as_mut_ptr()
                .offset(k as isize) as *mut CPpmd_See;
            (*s_0).Shift = (7 as libc::c_int - 4 as libc::c_int) as Byte;
            (*s_0).Summ = ((2 as libc::c_int as libc::c_uint)
                .wrapping_mul(i)
                .wrapping_add(5 as libc::c_int as libc::c_uint)
                << (*s_0).Shift as libc::c_int) as UInt16;
            (*s_0).Count = 7 as libc::c_int as Byte;
            k = k.wrapping_add(1)
        }
        m = m.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Init(
    mut p: *mut CPpmd8,
    mut maxOrder: libc::c_uint,
    mut restoreMethod: libc::c_uint,
) {
    (*p).MaxOrder = maxOrder;
    (*p).RestoreMethod = restoreMethod;
    RestartModel(p);
    (*p).DummySee.Shift = 7 as libc::c_int as Byte;
    (*p).DummySee.Summ = 0 as libc::c_int as UInt16;
    (*p).DummySee.Count = 64 as libc::c_int as Byte;
    /* unused */
}
unsafe extern "C" fn Refresh(
    mut p: *mut CPpmd8,
    mut ctx: CTX_PTR,
    mut oldNU: libc::c_uint,
    mut scale: libc::c_uint,
) {
    let mut i: libc::c_uint = (*ctx).NumStats as libc::c_uint;
    let mut escFreq: libc::c_uint = 0;
    let mut sumFreq: libc::c_uint = 0;
    let mut flags: libc::c_uint = 0;
    let mut s: *mut CPpmd_State = ShrinkUnits(
        p,
        (*p).Base.offset((*ctx).Stats as isize) as *mut libc::c_void as *mut CPpmd_State
            as *mut libc::c_void,
        oldNU,
        i.wrapping_add(2 as libc::c_int as libc::c_uint) >> 1 as libc::c_int,
    ) as *mut CPpmd_State;
    (*ctx).Stats = (s as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
    flags = ((*ctx).Flags as libc::c_uint
        & (0x10 as libc::c_int as libc::c_uint)
            .wrapping_add((0x4 as libc::c_int as libc::c_uint).wrapping_mul(scale)))
    .wrapping_add(
        (0x8 as libc::c_int * ((*s).Symbol as libc::c_int >= 0x40 as libc::c_int) as libc::c_int)
            as libc::c_uint,
    );
    escFreq = ((*ctx).SummFreq as libc::c_int - (*s).Freq as libc::c_int) as libc::c_uint;
    (*s).Freq = (((*s).Freq as libc::c_uint).wrapping_add(scale) >> scale) as Byte;
    sumFreq = (*s).Freq as libc::c_uint;
    loop {
        s = s.offset(1);
        escFreq = escFreq.wrapping_sub((*s).Freq as libc::c_uint);
        (*s).Freq = (((*s).Freq as libc::c_uint).wrapping_add(scale) >> scale) as Byte;
        sumFreq = sumFreq.wrapping_add((*s).Freq as libc::c_uint);
        flags |= (0x8 as libc::c_int
            * ((*s).Symbol as libc::c_int >= 0x40 as libc::c_int) as libc::c_int)
            as libc::c_uint;
        i = i.wrapping_sub(1);
        if !(i != 0) {
            break;
        }
    }
    (*ctx).SummFreq = sumFreq.wrapping_add(escFreq.wrapping_add(scale) >> scale) as UInt16;
    (*ctx).Flags = flags as Byte;
}
unsafe extern "C" fn SwapStates(mut t1: *mut CPpmd_State, mut t2: *mut CPpmd_State) {
    let mut tmp: CPpmd_State = *t1;
    *t1 = *t2;
    *t2 = tmp;
}
unsafe extern "C" fn CutOff(
    mut p: *mut CPpmd8,
    mut ctx: CTX_PTR,
    mut order: libc::c_uint,
) -> CPpmd_Void_Ref {
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_uint = 0;
    let mut s: *mut CPpmd_State = 0 as *mut CPpmd_State;
    if (*ctx).NumStats == 0 {
        s = &mut (*ctx).SummFreq as *mut UInt16 as *mut CPpmd_State;
        if (*p).Base.offset(
            ((*s).SuccessorLow as libc::c_uint
                | ((*s).SuccessorHigh as UInt32) << 16 as libc::c_int) as isize,
        ) as *mut libc::c_void as *mut Byte
            >= (*p).UnitsStart
        {
            if order < (*p).MaxOrder {
                SetSuccessor(
                    s,
                    CutOff(
                        p,
                        (*p).Base.offset(
                            ((*s).SuccessorLow as libc::c_uint
                                | ((*s).SuccessorHigh as UInt32) << 16 as libc::c_int)
                                as isize,
                        ) as *mut libc::c_void as *mut CPpmd8_Context,
                        order.wrapping_add(1 as libc::c_int as libc::c_uint),
                    ),
                );
            } else {
                SetSuccessor(s, 0 as libc::c_int as CPpmd_Void_Ref);
            }
            if (*s).SuccessorLow as libc::c_uint
                | ((*s).SuccessorHigh as UInt32) << 16 as libc::c_int
                != 0
                || order <= 9 as libc::c_int as libc::c_uint
            {
                /* O_BOUND */
                return (ctx as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
            }
        }
        SpecialFreeUnit(p, ctx as *mut libc::c_void);
        return 0 as libc::c_int as CPpmd_Void_Ref;
    }
    tmp = ((*ctx).NumStats as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
        >> 1 as libc::c_int;
    (*ctx).Stats = (MoveUnitsUp(
        p,
        (*p).Base.offset((*ctx).Stats as isize) as *mut libc::c_void as *mut CPpmd_State
            as *mut libc::c_void,
        tmp,
    ) as *mut Byte)
        .offset_from((*p).Base) as libc::c_long as UInt32;
    i = (*ctx).NumStats as libc::c_int;
    s = ((*p).Base.offset((*ctx).Stats as isize) as *mut libc::c_void as *mut CPpmd_State)
        .offset(i as isize);
    while s >= (*p).Base.offset((*ctx).Stats as isize) as *mut libc::c_void as *mut CPpmd_State {
        if ((*p).Base.offset(
            ((*s).SuccessorLow as libc::c_uint
                | ((*s).SuccessorHigh as UInt32) << 16 as libc::c_int) as isize,
        ) as *mut libc::c_void as *mut Byte)
            < (*p).UnitsStart
        {
            let fresh1 = i;
            i = i - 1;
            let mut s2: *mut CPpmd_State =
                ((*p).Base.offset((*ctx).Stats as isize) as *mut libc::c_void as *mut CPpmd_State)
                    .offset(fresh1 as isize);
            SetSuccessor(s, 0 as libc::c_int as CPpmd_Void_Ref);
            SwapStates(s, s2);
        } else if order < (*p).MaxOrder {
            SetSuccessor(
                s,
                CutOff(
                    p,
                    (*p).Base.offset(
                        ((*s).SuccessorLow as libc::c_uint
                            | ((*s).SuccessorHigh as UInt32) << 16 as libc::c_int)
                            as isize,
                    ) as *mut libc::c_void as *mut CPpmd8_Context,
                    order.wrapping_add(1 as libc::c_int as libc::c_uint),
                ),
            );
        } else {
            SetSuccessor(s, 0 as libc::c_int as CPpmd_Void_Ref);
        }
        s = s.offset(-1)
    }
    if i != (*ctx).NumStats as libc::c_int && order != 0 {
        (*ctx).NumStats = i as Byte;
        s = (*p).Base.offset((*ctx).Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
        if i < 0 as libc::c_int {
            FreeUnits(p, s as *mut libc::c_void, tmp);
            SpecialFreeUnit(p, ctx as *mut libc::c_void);
            return 0 as libc::c_int as CPpmd_Void_Ref;
        }
        if i == 0 as libc::c_int {
            (*ctx).Flags = (((*ctx).Flags as libc::c_int & 0x10 as libc::c_int)
                + 0x8 as libc::c_int
                    * ((*s).Symbol as libc::c_int >= 0x40 as libc::c_int) as libc::c_int)
                as Byte;
            *(&mut (*ctx).SummFreq as *mut UInt16 as *mut CPpmd_State) = *s;
            FreeUnits(p, s as *mut libc::c_void, tmp);
            /* 9.31: the code was fixed. It's was not BUG, if Freq <= MAX_FREQ = 124 */
            (*(&mut (*ctx).SummFreq as *mut UInt16 as *mut CPpmd_State)).Freq =
                (((*(&mut (*ctx).SummFreq as *mut UInt16 as *mut CPpmd_State)).Freq
                    as libc::c_uint)
                    .wrapping_add(11 as libc::c_int as libc::c_uint)
                    >> 3 as libc::c_int) as Byte
        } else {
            Refresh(
                p,
                ctx,
                tmp,
                ((*ctx).SummFreq as libc::c_int > 16 as libc::c_int * i) as libc::c_int
                    as libc::c_uint,
            );
        }
    }
    return (ctx as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
}
unsafe extern "C" fn GetUsedMemory(mut p: *const CPpmd8) -> UInt32 {
    let mut v: UInt32 = 0 as libc::c_int as UInt32;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i
        < (4 as libc::c_int
            + 4 as libc::c_int
            + 4 as libc::c_int
            + (128 as libc::c_int + 3 as libc::c_int
                - 1 as libc::c_int * 4 as libc::c_int
                - 2 as libc::c_int * 4 as libc::c_int
                - 3 as libc::c_int * 4 as libc::c_int)
                / 4 as libc::c_int) as libc::c_uint
    {
        v = (v as libc::c_uint).wrapping_add(
            (*p).Stamps[i as usize].wrapping_mul((*p).Indx2Units[i as usize] as libc::c_uint),
        ) as UInt32 as UInt32;
        i = i.wrapping_add(1)
    }
    return (*p)
        .Size
        .wrapping_sub((*p).HiUnit.offset_from((*p).LoUnit) as libc::c_long as UInt32)
        .wrapping_sub((*p).UnitsStart.offset_from((*p).Text) as libc::c_long as UInt32)
        .wrapping_sub(v.wrapping_mul(12 as libc::c_int as libc::c_uint));
}
unsafe extern "C" fn RestoreModel(mut p: *mut CPpmd8, mut c1: CTX_PTR) {
    let mut c: CTX_PTR = 0 as *mut CPpmd8_Context;
    let mut s: *mut CPpmd_State = 0 as *mut CPpmd_State;
    (*p).Text = (*p)
        .Base
        .offset((*p).AlignOffset as isize)
        .offset(0 as libc::c_int as isize);
    c = (*p).MaxContext;
    while c != c1 {
        (*c).NumStats = (*c).NumStats.wrapping_sub(1);
        if (*c).NumStats as libc::c_int == 0 as libc::c_int {
            s = (*p).Base.offset((*c).Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
            (*c).Flags = (((*c).Flags as libc::c_int & 0x10 as libc::c_int)
                + 0x8 as libc::c_int
                    * ((*s).Symbol as libc::c_int >= 0x40 as libc::c_int) as libc::c_int)
                as Byte;
            *(&mut (*c).SummFreq as *mut UInt16 as *mut CPpmd_State) = *s;
            SpecialFreeUnit(p, s as *mut libc::c_void);
            (*(&mut (*c).SummFreq as *mut UInt16 as *mut CPpmd_State)).Freq =
                (((*(&mut (*c).SummFreq as *mut UInt16 as *mut CPpmd_State)).Freq as libc::c_uint)
                    .wrapping_add(11 as libc::c_int as libc::c_uint)
                    >> 3 as libc::c_int) as Byte
        } else {
            Refresh(
                p,
                c,
                ((*c).NumStats as libc::c_int + 3 as libc::c_int >> 1 as libc::c_int)
                    as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            );
        }
        c = (*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context
    }
    while c != (*p).MinContext {
        if (*c).NumStats == 0 {
            (*(&mut (*c).SummFreq as *mut UInt16 as *mut CPpmd_State)).Freq =
                ((*(&mut (*c).SummFreq as *mut UInt16 as *mut CPpmd_State)).Freq as libc::c_int
                    - ((*(&mut (*c).SummFreq as *mut UInt16 as *mut CPpmd_State)).Freq
                        as libc::c_int
                        >> 1 as libc::c_int)) as Byte
        } else {
            (*c).SummFreq = ((*c).SummFreq as libc::c_int + 4 as libc::c_int) as UInt16;
            if (*c).SummFreq as libc::c_int
                > 128 as libc::c_int + 4 as libc::c_int * (*c).NumStats as libc::c_int
            {
                Refresh(
                    p,
                    c,
                    ((*c).NumStats as libc::c_int + 2 as libc::c_int >> 1 as libc::c_int)
                        as libc::c_uint,
                    1 as libc::c_int as libc::c_uint,
                );
            }
        }
        c = (*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context
    }
    if (*p).RestoreMethod == PPMD8_RESTORE_METHOD_RESTART as libc::c_int as libc::c_uint
        || GetUsedMemory(p) < (*p).Size >> 1 as libc::c_int
    {
        RestartModel(p);
    } else {
        while (*(*p).MaxContext).Suffix != 0 {
            (*p).MaxContext = (*p).Base.offset((*(*p).MaxContext).Suffix as isize)
                as *mut libc::c_void as *mut CPpmd8_Context
        }
        loop {
            CutOff(p, (*p).MaxContext, 0 as libc::c_int as libc::c_uint);
            ExpandTextArea(p);
            if !(GetUsedMemory(p)
                > (3 as libc::c_int as libc::c_uint).wrapping_mul((*p).Size >> 2 as libc::c_int))
            {
                break;
            }
        }
        (*p).GlueCount = 0 as libc::c_int as UInt32;
        (*p).OrderFall = (*p).MaxOrder
    };
}
unsafe extern "C" fn CreateSuccessors(
    mut p: *mut CPpmd8,
    mut skip: Bool,
    mut s1: *mut CPpmd_State,
    mut c: CTX_PTR,
) -> CTX_PTR {
    let mut upState: CPpmd_State = CPpmd_State {
        Symbol: 0,
        Freq: 0,
        SuccessorLow: 0,
        SuccessorHigh: 0,
    };
    let mut flags: Byte = 0;
    let mut upBranch: CPpmd_Byte_Ref = (*(*p).FoundState).SuccessorLow as libc::c_uint
        | ((*(*p).FoundState).SuccessorHigh as UInt32) << 16 as libc::c_int;
    /* fixed over Shkarin's code. Maybe it could work without + 1 too. */
    let mut ps: [*mut CPpmd_State; 17] = [0 as *mut CPpmd_State; 17];
    let mut numPs: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if skip == 0 {
        let fresh2 = numPs;
        numPs = numPs.wrapping_add(1);
        ps[fresh2 as usize] = (*p).FoundState
    }
    while (*c).Suffix != 0 {
        let mut successor: CPpmd_Void_Ref = 0;
        let mut s: *mut CPpmd_State = 0 as *mut CPpmd_State;
        c = (*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context;
        if !s1.is_null() {
            s = s1;
            s1 = 0 as *mut CPpmd_State
        } else if (*c).NumStats as libc::c_int != 0 as libc::c_int {
            s = (*p).Base.offset((*c).Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
            while (*s).Symbol as libc::c_int != (*(*p).FoundState).Symbol as libc::c_int {
                s = s.offset(1)
            }
            if ((*s).Freq as libc::c_int) < 124 as libc::c_int - 9 as libc::c_int {
                (*s).Freq = (*s).Freq.wrapping_add(1);
                (*c).SummFreq = (*c).SummFreq.wrapping_add(1)
            }
        } else {
            s = &mut (*c).SummFreq as *mut UInt16 as *mut CPpmd_State;
            (*s).Freq = ((*s).Freq as libc::c_int
                + (((*((*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void
                    as *mut CPpmd8_Context))
                    .NumStats
                    == 0) as libc::c_int
                    & (((*s).Freq as libc::c_int) < 24 as libc::c_int) as libc::c_int))
                as Byte
        }
        successor =
            (*s).SuccessorLow as libc::c_uint | ((*s).SuccessorHigh as UInt32) << 16 as libc::c_int;
        if successor != upBranch {
            c = (*p).Base.offset(successor as isize) as *mut libc::c_void as *mut CPpmd8_Context;
            if numPs == 0 as libc::c_int as libc::c_uint {
                return c;
            }
            break;
        } else {
            let fresh3 = numPs;
            numPs = numPs.wrapping_add(1);
            ps[fresh3 as usize] = s
        }
    }
    upState.Symbol = *((*p).Base.offset(upBranch as isize) as *mut libc::c_void as *const Byte);
    SetSuccessor(
        &mut upState,
        upBranch.wrapping_add(1 as libc::c_int as libc::c_uint),
    );
    flags = (0x10 as libc::c_int
        * ((*(*p).FoundState).Symbol as libc::c_int >= 0x40 as libc::c_int) as libc::c_int
        + 0x8 as libc::c_int
            * (upState.Symbol as libc::c_int >= 0x40 as libc::c_int) as libc::c_int)
        as Byte;
    if (*c).NumStats as libc::c_int == 0 as libc::c_int {
        upState.Freq = (*(&mut (*c).SummFreq as *mut UInt16 as *mut CPpmd_State)).Freq
    } else {
        let mut cf: UInt32 = 0;
        let mut s0: UInt32 = 0;
        let mut s_0: *mut CPpmd_State = 0 as *mut CPpmd_State;
        s_0 = (*p).Base.offset((*c).Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
        while (*s_0).Symbol as libc::c_int != upState.Symbol as libc::c_int {
            s_0 = s_0.offset(1)
        }
        cf = ((*s_0).Freq as libc::c_int - 1 as libc::c_int) as UInt32;
        s0 = (((*c).SummFreq as libc::c_int - (*c).NumStats as libc::c_int) as libc::c_uint)
            .wrapping_sub(cf);
        upState.Freq = (1 as libc::c_int as libc::c_uint).wrapping_add(
            if (2 as libc::c_int as libc::c_uint).wrapping_mul(cf) <= s0 {
                ((5 as libc::c_int as libc::c_uint).wrapping_mul(cf) > s0) as libc::c_int
                    as libc::c_uint
            } else {
                cf.wrapping_add((2 as libc::c_int as libc::c_uint).wrapping_mul(s0))
                    .wrapping_sub(3 as libc::c_int as libc::c_uint)
                    .wrapping_div(s0)
            },
        ) as Byte
    }
    loop {
        /* Create Child */
        let mut c1: CTX_PTR = 0 as *mut CPpmd8_Context; /* = AllocContext(p); */
        if (*p).HiUnit != (*p).LoUnit {
            (*p).HiUnit = (*p).HiUnit.offset(-(12 as libc::c_int as isize)); /* check it */
            c1 = (*p).HiUnit as CTX_PTR
        } else if (*p).FreeList[0 as libc::c_int as usize] != 0 as libc::c_int as libc::c_uint {
            c1 = RemoveNode(p, 0 as libc::c_int as libc::c_uint) as CTX_PTR
        } else {
            c1 = AllocUnitsRare(p, 0 as libc::c_int as libc::c_uint) as CTX_PTR;
            if c1.is_null() {
                return 0 as CTX_PTR;
            }
        }
        (*c1).NumStats = 0 as libc::c_int as Byte;
        (*c1).Flags = flags;
        *(&mut (*c1).SummFreq as *mut UInt16 as *mut CPpmd_State) = upState;
        (*c1).Suffix = (c as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
        numPs = numPs.wrapping_sub(1);
        SetSuccessor(
            ps[numPs as usize],
            (c1 as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32,
        );
        c = c1;
        if !(numPs != 0 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    return c;
}
unsafe extern "C" fn ReduceOrder(
    mut p: *mut CPpmd8,
    mut s1: *mut CPpmd_State,
    mut c: CTX_PTR,
) -> CTX_PTR {
    let mut s: *mut CPpmd_State = 0 as *mut CPpmd_State;
    let mut c1: CTX_PTR = c;
    let mut upBranch: CPpmd_Void_Ref = (*p).Text.offset_from((*p).Base) as libc::c_long as UInt32;
    SetSuccessor((*p).FoundState, upBranch);
    (*p).OrderFall = (*p).OrderFall.wrapping_add(1);
    loop {
        if !s1.is_null() {
            c = (*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context;
            s = s1;
            s1 = 0 as *mut CPpmd_State
        } else {
            if (*c).Suffix == 0 {
                return c;
            }
            c = (*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context;
            if (*c).NumStats != 0 {
                s = (*p).Base.offset((*c).Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
                if (*s).Symbol as libc::c_int != (*(*p).FoundState).Symbol as libc::c_int {
                    loop {
                        s = s.offset(1);
                        if !((*s).Symbol as libc::c_int != (*(*p).FoundState).Symbol as libc::c_int)
                        {
                            break;
                        }
                    }
                }
                if ((*s).Freq as libc::c_int) < 124 as libc::c_int - 9 as libc::c_int {
                    (*s).Freq = ((*s).Freq as libc::c_int + 2 as libc::c_int) as Byte;
                    (*c).SummFreq = ((*c).SummFreq as libc::c_int + 2 as libc::c_int) as UInt16
                }
            } else {
                s = &mut (*c).SummFreq as *mut UInt16 as *mut CPpmd_State;
                (*s).Freq = ((*s).Freq as libc::c_int
                    + (((*s).Freq as libc::c_int) < 32 as libc::c_int) as libc::c_int)
                    as Byte
            }
        }
        if (*s).SuccessorLow as libc::c_uint | ((*s).SuccessorHigh as UInt32) << 16 as libc::c_int
            != 0
        {
            break;
        }
        SetSuccessor(s, upBranch);
        (*p).OrderFall = (*p).OrderFall.wrapping_add(1)
    }
    if (*s).SuccessorLow as libc::c_uint | ((*s).SuccessorHigh as UInt32) << 16 as libc::c_int
        <= upBranch
    {
        let mut successor: CTX_PTR = 0 as *mut CPpmd8_Context;
        let mut s2: *mut CPpmd_State = (*p).FoundState;
        (*p).FoundState = s;
        successor = CreateSuccessors(p, 0 as libc::c_int, 0 as *mut CPpmd_State, c);
        if successor.is_null() {
            SetSuccessor(s, 0 as libc::c_int as CPpmd_Void_Ref);
        } else {
            SetSuccessor(
                s,
                (successor as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32,
            );
        }
        (*p).FoundState = s2
    }
    if (*p).OrderFall == 1 as libc::c_int as libc::c_uint && c1 == (*p).MaxContext {
        SetSuccessor(
            (*p).FoundState,
            (*s).SuccessorLow as libc::c_uint | ((*s).SuccessorHigh as UInt32) << 16 as libc::c_int,
        );
        (*p).Text = (*p).Text.offset(-1)
    }
    if (*s).SuccessorLow as libc::c_uint | ((*s).SuccessorHigh as UInt32) << 16 as libc::c_int
        == 0 as libc::c_int as libc::c_uint
    {
        return 0 as CTX_PTR;
    }
    return (*p).Base.offset(
        ((*s).SuccessorLow as libc::c_uint | ((*s).SuccessorHigh as UInt32) << 16 as libc::c_int)
            as isize,
    ) as *mut libc::c_void as *mut CPpmd8_Context;
}
unsafe extern "C" fn UpdateModel(mut p: *mut CPpmd8) {
    let mut successor: CPpmd_Void_Ref = 0;
    let mut fSuccessor: CPpmd_Void_Ref = (*(*p).FoundState).SuccessorLow as libc::c_uint
        | ((*(*p).FoundState).SuccessorHigh as UInt32) << 16 as libc::c_int;
    let mut c: CTX_PTR = 0 as *mut CPpmd8_Context;
    let mut s0: libc::c_uint = 0;
    let mut ns: libc::c_uint = 0;
    let mut fFreq: libc::c_uint = (*(*p).FoundState).Freq as libc::c_uint;
    let mut flag: Byte = 0;
    let mut fSymbol: Byte = (*(*p).FoundState).Symbol;
    let mut s: *mut CPpmd_State = 0 as *mut CPpmd_State;
    if ((*(*p).FoundState).Freq as libc::c_int) < 124 as libc::c_int / 4 as libc::c_int
        && (*(*p).MinContext).Suffix != 0 as libc::c_int as libc::c_uint
    {
        c = (*p).Base.offset((*(*p).MinContext).Suffix as isize) as *mut libc::c_void
            as *mut CPpmd8_Context;
        if (*c).NumStats as libc::c_int == 0 as libc::c_int {
            s = &mut (*c).SummFreq as *mut UInt16 as *mut CPpmd_State;
            if ((*s).Freq as libc::c_int) < 32 as libc::c_int {
                (*s).Freq = (*s).Freq.wrapping_add(1)
            }
        } else {
            s = (*p).Base.offset((*c).Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
            if (*s).Symbol as libc::c_int != (*(*p).FoundState).Symbol as libc::c_int {
                loop {
                    s = s.offset(1);
                    if !((*s).Symbol as libc::c_int != (*(*p).FoundState).Symbol as libc::c_int) {
                        break;
                    }
                }
                if (*s.offset(0 as libc::c_int as isize)).Freq as libc::c_int
                    >= (*s.offset(-(1 as libc::c_int) as isize)).Freq as libc::c_int
                {
                    SwapStates(
                        &mut *s.offset(0 as libc::c_int as isize),
                        &mut *s.offset(-(1 as libc::c_int) as isize),
                    );
                    s = s.offset(-1)
                }
            }
            if ((*s).Freq as libc::c_int) < 124 as libc::c_int - 9 as libc::c_int {
                (*s).Freq = ((*s).Freq as libc::c_int + 2 as libc::c_int) as Byte;
                (*c).SummFreq = ((*c).SummFreq as libc::c_int + 2 as libc::c_int) as UInt16
            }
        }
    }
    c = (*p).MaxContext;
    if (*p).OrderFall == 0 as libc::c_int as libc::c_uint && fSuccessor != 0 {
        let mut cs: CTX_PTR = CreateSuccessors(p, 1 as libc::c_int, s, (*p).MinContext);
        if cs.is_null() {
            SetSuccessor((*p).FoundState, 0 as libc::c_int as CPpmd_Void_Ref);
            RestoreModel(p, c);
        } else {
            SetSuccessor(
                (*p).FoundState,
                (cs as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32,
            );
            (*p).MaxContext = cs
        }
        return;
    }
    let fresh4 = (*p).Text;
    (*p).Text = (*p).Text.offset(1);
    *fresh4 = (*(*p).FoundState).Symbol;
    successor = (*p).Text.offset_from((*p).Base) as libc::c_long as UInt32;
    if (*p).Text >= (*p).UnitsStart {
        RestoreModel(p, c);
        return;
    }
    if fSuccessor == 0 {
        let mut cs_0: CTX_PTR = ReduceOrder(p, s, (*p).MinContext);
        if cs_0.is_null() {
            RestoreModel(p, c);
            return;
        }
        fSuccessor = (cs_0 as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32
    } else if ((*p).Base.offset(fSuccessor as isize) as *mut libc::c_void as *mut Byte)
        < (*p).UnitsStart
    {
        let mut cs_1: CTX_PTR = CreateSuccessors(p, 0 as libc::c_int, s, (*p).MinContext);
        if cs_1.is_null() {
            RestoreModel(p, c);
            return;
        }
        fSuccessor = (cs_1 as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32
    }
    (*p).OrderFall = (*p).OrderFall.wrapping_sub(1);
    if (*p).OrderFall == 0 as libc::c_int as libc::c_uint {
        successor = fSuccessor;
        (*p).Text = (*p)
            .Text
            .offset(-(((*p).MaxContext != (*p).MinContext) as libc::c_int as isize))
    }
    ns = (*(*p).MinContext).NumStats as libc::c_uint;
    s0 = ((*(*p).MinContext).SummFreq as libc::c_uint)
        .wrapping_sub(ns)
        .wrapping_sub(fFreq);
    flag = (0x8 as libc::c_int * (fSymbol as libc::c_int >= 0x40 as libc::c_int) as libc::c_int)
        as Byte;
    while c != (*p).MinContext {
        let mut ns1: libc::c_uint = 0;
        let mut cf: UInt32 = 0;
        let mut sf: UInt32 = 0;
        ns1 = (*c).NumStats as libc::c_uint;
        if ns1 != 0 as libc::c_int as libc::c_uint {
            if ns1 & 1 as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint {
                /* Expand for one UNIT */
                let mut oldNU: libc::c_uint =
                    ns1.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1 as libc::c_int;
                let mut i: libc::c_uint = (*p).Units2Indx
                    [(oldNU as size_t).wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
                    as libc::c_uint;
                if i != (*p).Units2Indx[(oldNU as size_t)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as usize] as libc::c_uint
                {
                    let mut ptr: *mut libc::c_void =
                        AllocUnits(p, i.wrapping_add(1 as libc::c_int as libc::c_uint));
                    let mut oldPtr: *mut libc::c_void = 0 as *mut libc::c_void;
                    if ptr.is_null() {
                        RestoreModel(p, c);
                        return;
                    }
                    oldPtr = (*p).Base.offset((*c).Stats as isize) as *mut libc::c_void
                        as *mut CPpmd_State as *mut libc::c_void;
                    let mut d: *mut UInt32 = ptr as *mut UInt32;
                    let mut z: *const UInt32 = oldPtr as *const UInt32;
                    let mut n: UInt32 = oldNU;
                    loop {
                        *d.offset(0 as libc::c_int as isize) = *z.offset(0 as libc::c_int as isize);
                        *d.offset(1 as libc::c_int as isize) = *z.offset(1 as libc::c_int as isize);
                        *d.offset(2 as libc::c_int as isize) = *z.offset(2 as libc::c_int as isize);
                        z = z.offset(3 as libc::c_int as isize);
                        d = d.offset(3 as libc::c_int as isize);
                        n = n.wrapping_sub(1);
                        if !(n != 0) {
                            break;
                        }
                    }
                    InsertNode(p, oldPtr, i);
                    (*c).Stats = (ptr as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32
                }
            }
            (*c).SummFreq = ((*c).SummFreq as libc::c_int
                + ((3 as libc::c_int as libc::c_uint)
                    .wrapping_mul(ns1)
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    < ns) as libc::c_int) as UInt16
        } else {
            let mut s2: *mut CPpmd_State =
                AllocUnits(p, 0 as libc::c_int as libc::c_uint) as *mut CPpmd_State;
            if s2.is_null() {
                RestoreModel(p, c);
                return;
            }
            *s2 = *(&mut (*c).SummFreq as *mut UInt16 as *mut CPpmd_State);
            (*c).Stats = (s2 as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
            if ((*s2).Freq as libc::c_int)
                < 124 as libc::c_int / 4 as libc::c_int - 1 as libc::c_int
            {
                (*s2).Freq = (((*s2).Freq as libc::c_int) << 1 as libc::c_int) as Byte
            } else {
                (*s2).Freq = (124 as libc::c_int - 4 as libc::c_int) as Byte
            }
            (*c).SummFreq = ((*s2).Freq as libc::c_uint)
                .wrapping_add((*p).InitEsc)
                .wrapping_add(
                    (ns > 2 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_uint,
                ) as UInt16
        }
        cf = (2 as libc::c_int as libc::c_uint)
            .wrapping_mul(fFreq)
            .wrapping_mul(((*c).SummFreq as libc::c_int + 6 as libc::c_int) as libc::c_uint);
        sf = s0.wrapping_add((*c).SummFreq as libc::c_uint);
        if cf < (6 as libc::c_int as libc::c_uint).wrapping_mul(sf) {
            cf = (1 as libc::c_int
                + (cf > sf) as libc::c_int
                + (cf >= (4 as libc::c_int as libc::c_uint).wrapping_mul(sf)) as libc::c_int)
                as UInt32;
            (*c).SummFreq = ((*c).SummFreq as libc::c_int + 4 as libc::c_int) as UInt16
        } else {
            cf = (4 as libc::c_int
                + (cf > (9 as libc::c_int as libc::c_uint).wrapping_mul(sf)) as libc::c_int
                + (cf > (12 as libc::c_int as libc::c_uint).wrapping_mul(sf)) as libc::c_int
                + (cf > (15 as libc::c_int as libc::c_uint).wrapping_mul(sf)) as libc::c_int)
                as UInt32;
            (*c).SummFreq = ((*c).SummFreq as libc::c_uint).wrapping_add(cf) as UInt16
        }
        let mut s2_0: *mut CPpmd_State =
            ((*p).Base.offset((*c).Stats as isize) as *mut libc::c_void as *mut CPpmd_State)
                .offset(ns1 as isize)
                .offset(1 as libc::c_int as isize);
        SetSuccessor(s2_0, successor);
        (*s2_0).Symbol = fSymbol;
        (*s2_0).Freq = cf as Byte;
        (*c).Flags = ((*c).Flags as libc::c_int | flag as libc::c_int) as Byte;
        (*c).NumStats = ns1.wrapping_add(1 as libc::c_int as libc::c_uint) as Byte;
        c = (*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context
    }
    (*p).MinContext =
        (*p).Base.offset(fSuccessor as isize) as *mut libc::c_void as *mut CPpmd8_Context;
    (*p).MaxContext = (*p).MinContext;
}
unsafe extern "C" fn Rescale(mut p: *mut CPpmd8) {
    let mut i: libc::c_uint = 0;
    let mut adder: libc::c_uint = 0;
    let mut sumFreq: libc::c_uint = 0;
    let mut escFreq: libc::c_uint = 0;
    let mut stats: *mut CPpmd_State = (*p).Base.offset((*(*p).MinContext).Stats as isize)
        as *mut libc::c_void as *mut CPpmd_State;
    let mut s: *mut CPpmd_State = (*p).FoundState;
    let mut tmp: CPpmd_State = *s;
    while s != stats {
        *s.offset(0 as libc::c_int as isize) = *s.offset(-(1 as libc::c_int) as isize);
        s = s.offset(-1)
    }
    *s = tmp;
    escFreq =
        ((*(*p).MinContext).SummFreq as libc::c_int - (*s).Freq as libc::c_int) as libc::c_uint;
    (*s).Freq = ((*s).Freq as libc::c_int + 4 as libc::c_int) as Byte;
    adder = ((*p).OrderFall != 0 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_uint;
    (*s).Freq = (((*s).Freq as libc::c_uint).wrapping_add(adder) >> 1 as libc::c_int) as Byte;
    sumFreq = (*s).Freq as libc::c_uint;
    i = (*(*p).MinContext).NumStats as libc::c_uint;
    loop {
        s = s.offset(1);
        escFreq = escFreq.wrapping_sub((*s).Freq as libc::c_uint);
        (*s).Freq = (((*s).Freq as libc::c_uint).wrapping_add(adder) >> 1 as libc::c_int) as Byte;
        sumFreq = sumFreq.wrapping_add((*s).Freq as libc::c_uint);
        if (*s.offset(0 as libc::c_int as isize)).Freq as libc::c_int
            > (*s.offset(-(1 as libc::c_int) as isize)).Freq as libc::c_int
        {
            let mut s1: *mut CPpmd_State = s;
            let mut tmp_0: CPpmd_State = *s1;
            loop {
                *s1.offset(0 as libc::c_int as isize) = *s1.offset(-(1 as libc::c_int) as isize);
                s1 = s1.offset(-1);
                if !(s1 != stats
                    && tmp_0.Freq as libc::c_int
                        > (*s1.offset(-(1 as libc::c_int) as isize)).Freq as libc::c_int)
                {
                    break;
                }
            }
            *s1 = tmp_0
        }
        i = i.wrapping_sub(1);
        if !(i != 0) {
            break;
        }
    }
    if (*s).Freq as libc::c_int == 0 as libc::c_int {
        let mut numStats: libc::c_uint = (*(*p).MinContext).NumStats as libc::c_uint;
        let mut n0: libc::c_uint = 0;
        let mut n1: libc::c_uint = 0;
        loop {
            i = i.wrapping_add(1);
            s = s.offset(-1);
            if !((*s).Freq as libc::c_int == 0 as libc::c_int) {
                break;
            }
        }
        escFreq = escFreq.wrapping_add(i);
        (*(*p).MinContext).NumStats =
            ((*(*p).MinContext).NumStats as libc::c_uint).wrapping_sub(i) as Byte;
        if (*(*p).MinContext).NumStats as libc::c_int == 0 as libc::c_int {
            let mut tmp_1: CPpmd_State = *stats;
            tmp_1.Freq = ((2 as libc::c_int * tmp_1.Freq as libc::c_int) as libc::c_uint)
                .wrapping_add(escFreq)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_div(escFreq) as Byte;
            if tmp_1.Freq as libc::c_int > 124 as libc::c_int / 3 as libc::c_int {
                tmp_1.Freq = (124 as libc::c_int / 3 as libc::c_int) as Byte
            }
            InsertNode(
                p,
                stats as *mut libc::c_void,
                (*p).Units2Indx[((numStats.wrapping_add(2 as libc::c_int as libc::c_uint)
                    >> 1 as libc::c_int) as size_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as usize] as libc::c_uint,
            );
            (*(*p).MinContext).Flags = (((*(*p).MinContext).Flags as libc::c_int
                & 0x10 as libc::c_int)
                + 0x8 as libc::c_int
                    * (tmp_1.Symbol as libc::c_int >= 0x40 as libc::c_int) as libc::c_int)
                as Byte;
            (*p).FoundState = &mut (*(*p).MinContext).SummFreq as *mut UInt16 as *mut CPpmd_State;
            *(*p).FoundState = tmp_1;
            return;
        }
        n0 = numStats.wrapping_add(2 as libc::c_int as libc::c_uint) >> 1 as libc::c_int;
        n1 = ((*(*p).MinContext).NumStats as libc::c_int + 2 as libc::c_int >> 1 as libc::c_int)
            as libc::c_uint;
        if n0 != n1 {
            (*(*p).MinContext).Stats =
                (ShrinkUnits(p, stats as *mut libc::c_void, n0, n1) as *mut Byte)
                    .offset_from((*p).Base) as libc::c_long as UInt32
        }
        (*(*p).MinContext).Flags =
            ((*(*p).MinContext).Flags as libc::c_int & !(0x8 as libc::c_int)) as Byte;
        s = (*p).Base.offset((*(*p).MinContext).Stats as isize) as *mut libc::c_void
            as *mut CPpmd_State;
        (*(*p).MinContext).Flags = ((*(*p).MinContext).Flags as libc::c_int
            | 0x8 as libc::c_int
                * ((*s).Symbol as libc::c_int >= 0x40 as libc::c_int) as libc::c_int)
            as Byte;
        i = (*(*p).MinContext).NumStats as libc::c_uint;
        loop {
            s = s.offset(1);
            (*(*p).MinContext).Flags = ((*(*p).MinContext).Flags as libc::c_int
                | 0x8 as libc::c_int
                    * ((*s).Symbol as libc::c_int >= 0x40 as libc::c_int) as libc::c_int)
                as Byte;
            i = i.wrapping_sub(1);
            if !(i != 0) {
                break;
            }
        }
    }
    (*(*p).MinContext).SummFreq = sumFreq
        .wrapping_add(escFreq)
        .wrapping_sub(escFreq >> 1 as libc::c_int) as UInt16;
    (*(*p).MinContext).Flags =
        ((*(*p).MinContext).Flags as libc::c_int | 0x4 as libc::c_int) as Byte;
    (*p).FoundState = (*p).Base.offset((*(*p).MinContext).Stats as isize) as *mut libc::c_void
        as *mut CPpmd_State;
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_MakeEscFreq(
    mut p: *mut CPpmd8,
    mut numMasked1: libc::c_uint,
    mut escFreq: *mut UInt32,
) -> *mut CPpmd_See {
    let mut see: *mut CPpmd_See = 0 as *mut CPpmd_See;
    if (*(*p).MinContext).NumStats as libc::c_int != 0xff as libc::c_int {
        see = (*p).See[((*p).NS2Indx[((*(*p).MinContext).NumStats as libc::c_uint as size_t)
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            as usize] as libc::c_uint as size_t)
            .wrapping_sub(3 as libc::c_int as libc::c_ulong) as usize]
            .as_mut_ptr()
            .offset(
                ((*(*p).MinContext).SummFreq as libc::c_uint
                    > (11 as libc::c_int as libc::c_uint).wrapping_mul(
                        ((*(*p).MinContext).NumStats as libc::c_uint)
                            .wrapping_add(1 as libc::c_int as libc::c_uint),
                    )) as libc::c_int as isize,
            )
            .offset(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(
                    ((2 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*(*p).MinContext).NumStats as libc::c_uint)
                        < ((*((*p).Base.offset((*(*p).MinContext).Suffix as isize)
                            as *mut libc::c_void
                            as *mut CPpmd8_Context))
                            .NumStats as libc::c_uint)
                            .wrapping_add(numMasked1)) as libc::c_int
                        as libc::c_uint,
                ) as isize,
            )
            .offset((*(*p).MinContext).Flags as libc::c_int as isize);
        let mut r: libc::c_uint =
            ((*see).Summ as libc::c_int >> (*see).Shift as libc::c_int) as libc::c_uint;
        (*see).Summ = ((*see).Summ as libc::c_uint).wrapping_sub(r) as UInt16;
        *escFreq =
            r.wrapping_add((r == 0 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_uint)
    } else {
        see = &mut (*p).DummySee;
        *escFreq = 1 as libc::c_int as UInt32
    }
    return see;
}
unsafe extern "C" fn NextContext(mut p: *mut CPpmd8) {
    let mut c: CTX_PTR = (*p).Base.offset(
        ((*(*p).FoundState).SuccessorLow as libc::c_uint
            | ((*(*p).FoundState).SuccessorHigh as UInt32) << 16 as libc::c_int) as isize,
    ) as *mut libc::c_void as *mut CPpmd8_Context;
    if (*p).OrderFall == 0 as libc::c_int as libc::c_uint && c as *mut Byte >= (*p).UnitsStart {
        (*p).MaxContext = c;
        (*p).MinContext = (*p).MaxContext
    } else {
        UpdateModel(p);
        (*p).MinContext = (*p).MaxContext
    };
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Update1(mut p: *mut CPpmd8) {
    let mut s: *mut CPpmd_State = (*p).FoundState;
    (*s).Freq = ((*s).Freq as libc::c_int + 4 as libc::c_int) as Byte;
    (*(*p).MinContext).SummFreq =
        ((*(*p).MinContext).SummFreq as libc::c_int + 4 as libc::c_int) as UInt16;
    if (*s.offset(0 as libc::c_int as isize)).Freq as libc::c_int
        > (*s.offset(-(1 as libc::c_int) as isize)).Freq as libc::c_int
    {
        SwapStates(
            &mut *s.offset(0 as libc::c_int as isize),
            &mut *s.offset(-(1 as libc::c_int) as isize),
        );
        s = s.offset(-1);
        (*p).FoundState = s;
        if (*s).Freq as libc::c_int > 124 as libc::c_int {
            Rescale(p);
        }
    }
    NextContext(p);
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Update1_0(mut p: *mut CPpmd8) {
    (*p).PrevSuccess = (2 as libc::c_int * (*(*p).FoundState).Freq as libc::c_int
        >= (*(*p).MinContext).SummFreq as libc::c_int) as libc::c_int
        as libc::c_uint;
    (*p).RunLength =
        ((*p).RunLength as libc::c_uint).wrapping_add((*p).PrevSuccess) as Int32 as Int32;
    (*(*p).MinContext).SummFreq =
        ((*(*p).MinContext).SummFreq as libc::c_int + 4 as libc::c_int) as UInt16;
    (*(*p).FoundState).Freq = ((*(*p).FoundState).Freq as libc::c_int + 4 as libc::c_int) as Byte;
    if (*(*p).FoundState).Freq as libc::c_int > 124 as libc::c_int {
        Rescale(p);
    }
    NextContext(p);
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_UpdateBin(mut p: *mut CPpmd8) {
    (*(*p).FoundState).Freq = ((*(*p).FoundState).Freq as libc::c_int
        + (((*(*p).FoundState).Freq as libc::c_int) < 196 as libc::c_int) as libc::c_int)
        as Byte;
    (*p).PrevSuccess = 1 as libc::c_int as libc::c_uint;
    (*p).RunLength += 1;
    NextContext(p);
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Update2(mut p: *mut CPpmd8) {
    (*(*p).MinContext).SummFreq =
        ((*(*p).MinContext).SummFreq as libc::c_int + 4 as libc::c_int) as UInt16;
    (*(*p).FoundState).Freq = ((*(*p).FoundState).Freq as libc::c_int + 4 as libc::c_int) as Byte;
    if (*(*p).FoundState).Freq as libc::c_int > 124 as libc::c_int {
        Rescale(p);
    }
    (*p).RunLength = (*p).InitRL;
    UpdateModel(p);
    (*p).MinContext = (*p).MaxContext;
}
/* H->I changes:
  NS2Indx
  GlewCount, and Glue method
  BinSum
  See / EscFreq
  CreateSuccessors updates more suffix contexts
  UpdateModel consts.
  PrevSuccess Update
*/

#[no_mangle]
pub unsafe extern "C" fn Ppmd8_RangeDec_Init(mut p: *mut CPpmd8) -> Bool {
    let mut i: libc::c_uint = 0;
    (*p).Low = 0 as libc::c_int as UInt32;
    (*p).Range = 0xffffffff as libc::c_uint;
    (*p).Code = 0 as libc::c_int as UInt32;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 4 as libc::c_int as libc::c_uint {
        (*p).Code = (*p).Code << 8 as libc::c_int
            | (*(*p).Stream.In).Read.expect("non-null function pointer")((*p).Stream.In)
                as libc::c_uint;
        i = i.wrapping_add(1)
    }
    return ((*p).Code < 0xffffffff as libc::c_uint) as libc::c_int;
}

unsafe extern "C" fn pmalloc(mut ip: ISzAllocPtr, mut size: size_t) -> *mut libc::c_void {
    return malloc(size.try_into().unwrap()); /* EndMark */
}
unsafe extern "C" fn pfree(mut ip: ISzAllocPtr, mut addr: *mut libc::c_void) {
    free(addr);
}
pub static mut ialloc: ISzAlloc = unsafe {
    {
        let mut init = ISzAlloc {
            Alloc: Some(
                pmalloc as unsafe extern "C" fn(_: ISzAllocPtr, _: size_t) -> *mut libc::c_void,
            ),
            Free: Some(pfree as unsafe extern "C" fn(_: ISzAllocPtr, _: *mut libc::c_void) -> ()),
        };
        init
    }
};
