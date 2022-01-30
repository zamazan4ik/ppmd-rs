use libc::{free, malloc};

//pub type u64 = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct IByteIn {
    pub Read: Option<unsafe extern "C" fn(_: *const IByteIn) -> u8>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IByteOut {
    pub Write: Option<unsafe extern "C" fn(_: *const IByteOut, _: u8) -> ()>,
}
/* Returns: result. (result != SZ_OK) means break.
Value (UInt64)(Int64)-1 for size means unknown value. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ISzAlloc {
    pub Alloc: Option<unsafe extern "C" fn(_: ISzAllocPtr, _: u64) -> *mut libc::c_void>,
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
    pub Summ: u16,
    pub Shift: u8,
    pub Count: u8,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct CPpmd_State {
    pub Symbol: u8,
    pub Freq: u8,
    pub SuccessorLow: u16,
    pub SuccessorHigh: u16,
}
pub type CPpmd_State_Ref = u32;
pub type CPpmd_Void_Ref = u32;
pub type CPpmd_Byte_Ref = u32;
/* Ppmd8.h -- PPMdI codec
2017-04-03 : Igor Pavlov : Public domain
This code is based on:
  PPMd var.I (2002): Dmitry Shkarin : Public domain
  Carryless rangecoder (1999): Dmitry Subbotin : Public domain */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd8_Context_ {
    pub NumStats: u8,
    pub Flags: u8,
    pub SummFreq: u16,
    pub Stats: CPpmd_State_Ref,
    pub Suffix: CPpmd8_Context_Ref,
}
pub type CPpmd8_Context_Ref = u32;
pub type CPpmd8_Context = CPpmd8_Context_;
pub type C2RustUnnamed = u32;
pub const PPMD8_RESTORE_METHOD_FREEZE: C2RustUnnamed = 2;
pub const PPMD8_RESTORE_METHOD_CUT_OFF: C2RustUnnamed = 1;
pub const PPMD8_RESTORE_METHOD_RESTART: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd8 {
    pub MinContext: *mut CPpmd8_Context,
    pub MaxContext: *mut CPpmd8_Context,
    pub FoundState: *mut CPpmd_State,
    pub OrderFall: u32,
    pub InitEsc: u32,
    pub PrevSuccess: u32,
    pub MaxOrder: u32,
    pub RunLength: i32,
    pub InitRL: i32,
    pub Size: u32,
    pub GlueCount: u32,
    pub Base: *mut u8,
    pub LoUnit: *mut u8,
    pub HiUnit: *mut u8,
    pub Text: *mut u8,
    pub UnitsStart: *mut u8,
    pub AlignOffset: u32,
    pub RestoreMethod: u32,
    pub Range: u32,
    pub Code: u32,
    pub Low: u32,
    pub Stream: C2RustUnnamed_0,
    pub Indx2Units: [u8; 38],
    pub Units2Indx: [u8; 128],
    pub FreeList: [CPpmd_Void_Ref; 38],
    pub Stamps: [u32; 38],
    pub NS2BSIndx: [u8; 256],
    pub NS2Indx: [u8; 260],
    pub DummySee: CPpmd_See,
    pub See: [[CPpmd_See; 32]; 24],
    pub BinSumm: [[u16; 64]; 25],
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
    pub Stamp: u32,
    pub Next: CPpmd8_Node_Ref,
    pub NU: u32,
}
pub type CPpmd8_Node_Ref = u32;
/* Ppmd8.c -- PPMdI codec
2017-04-03 : Igor Pavlov : Public domain
This code is based on PPMd var.I (2002): Dmitry Shkarin : Public domain */
#[no_mangle]
pub static mut PPMD8_kExpEscape: [u8; 16] = [25, 14, 9, 7, 5, 5, 4, 4, 4, 3, 3, 3, 2, 2, 2, 2];
static mut kInitBinEsc: [u16; 8] = [
    0x3cdd, 0x1f3f, 0x59bf, 0x48f3, 0x64a1, 0x5abc, 0x6632, 0x6051,
];
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Construct(mut p: *mut CPpmd8) {
    let mut i: u32 = 0;
    let mut k: u32 = 0;
    let mut m: u32 = 0;
    (*p).Base = 0 as *mut u8;
    while i
        < (4 as i32
            + 4 as i32
            + 4 as i32
            + (128 as i32 + 3 as i32
                - 1 as i32 * 4 as i32
                - 2 as i32 * 4 as i32
                - 3 as i32 * 4 as i32)
                / 4 as i32) as u32
    {
        let mut step: u32 = if i >= 12 as i32 as u32 {
            4 as i32 as u32
        } else {
            (i >> 2 as i32).wrapping_add(1 as i32 as u32)
        };
        loop {
            let fresh0 = k;
            k = k.wrapping_add(1);
            (*p).Units2Indx[fresh0 as usize] = i as u8;
            step = step.wrapping_sub(1);
            if !(step != 0) {
                break;
            }
        }
        (*p).Indx2Units[i as usize] = k as u8;
        i = i.wrapping_add(1)
    }
    (*p).NS2BSIndx[0 as i32 as usize] = ((0 as i32) << 1 as i32) as u8;
    (*p).NS2BSIndx[1 as i32 as usize] = ((1 as i32) << 1 as i32) as u8;
    libc::memset(
        (*p).NS2BSIndx.as_mut_ptr().offset(2 as i32 as isize) as *mut libc::c_void,
        (2 as i32) << 1 as i32,
        9,
    );
    libc::memset(
        (*p).NS2BSIndx.as_mut_ptr().offset(11 as i32 as isize) as *mut libc::c_void,
        (3 as i32) << 1 as i32,
        256 - 11,
    );
    i = 0 as i32 as u32;
    while i < 5 as i32 as u32 {
        (*p).NS2Indx[i as usize] = i as u8;
        i = i.wrapping_add(1)
    }
    m = i;
    k = 1 as i32 as u32;
    while i < 260 as i32 as u32 {
        (*p).NS2Indx[i as usize] = m as u8;
        k = k.wrapping_sub(1);
        if k == 0 as i32 as u32 {
            m = m.wrapping_add(1);
            k = m.wrapping_sub(4 as i32 as u32)
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
pub unsafe extern "C" fn Ppmd8_Free(mut p: *mut CPpmd8, alloc: ISzAllocPtr) {
    (*alloc).Free.expect("non-null function pointer")(alloc, (*p).Base as *mut libc::c_void);
    (*p).Size = 0 as i32 as u32;
    (*p).Base = 0 as *mut u8;
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Alloc(mut p: *mut CPpmd8, size: u32, alloc: ISzAllocPtr) -> i32 {
    if (*p).Base.is_null() || (*p).Size != size {
        Ppmd8_Free(p, alloc);
        (*p).AlignOffset = (4 as i32 as u32).wrapping_sub(size & 3 as i32 as u32);
        (*p).Base = (*alloc).Alloc.expect("non-null function pointer")(
            alloc,
            (*p).AlignOffset.wrapping_add(size) as u64,
        ) as *mut u8;
        if (*p).Base.is_null() {
            return 0 as i32;
        }
        (*p).Size = size
    }
    return 1 as i32;
}
unsafe extern "C" fn InsertNode(mut p: *mut CPpmd8, node: *mut libc::c_void, indx: u32) {
    (*(node as *mut CPpmd8_Node)).Stamp = 0xffffffff as u32;
    (*(node as *mut CPpmd8_Node)).Next = (*p).FreeList[indx as usize];
    (*(node as *mut CPpmd8_Node)).NU = (*p).Indx2Units[indx as usize] as u32;
    (*p).FreeList[indx as usize] = (node as *mut u8).offset_from((*p).Base) as libc::c_long as u32;
    (*p).Stamps[indx as usize] = (*p).Stamps[indx as usize].wrapping_add(1);
}
unsafe extern "C" fn RemoveNode(mut p: *mut CPpmd8, indx: u32) -> *mut libc::c_void {
    let node: *mut CPpmd8_Node =
        (*p).Base.offset((*p).FreeList[indx as usize] as isize) as *mut CPpmd8_Node;
    (*p).FreeList[indx as usize] = (*node).Next;
    (*p).Stamps[indx as usize] = (*p).Stamps[indx as usize].wrapping_sub(1);
    return node as *mut libc::c_void;
}
unsafe extern "C" fn SplitBlock(
    p: *mut CPpmd8,
    mut ptr: *mut libc::c_void,
    oldIndx: u32,
    newIndx: u32,
) {
    let mut i: u32 = 0;
    let nu: u32 = ((*p).Indx2Units[oldIndx as usize] as i32
        - (*p).Indx2Units[newIndx as usize] as i32) as u32;
    ptr = (ptr as *mut u8)
        .offset(((*p).Indx2Units[newIndx as usize] as u32).wrapping_mul(12 as i32 as u32) as isize)
        as *mut libc::c_void;
    i = (*p).Units2Indx[(nu as u64).wrapping_sub(1 as i32 as u64) as usize] as u32;
    if (*p).Indx2Units[i as usize] as u32 != nu {
        i = i.wrapping_sub(1);
        let k: u32 = (*p).Indx2Units[i as usize] as u32;
        InsertNode(
            p,
            (ptr as *mut u8).offset(k.wrapping_mul(12 as i32 as u32) as isize) as *mut libc::c_void,
            nu.wrapping_sub(k).wrapping_sub(1 as i32 as u32),
        );
    }
    InsertNode(p, ptr, i);
}
unsafe extern "C" fn GlueFreeBlocks(mut p: *mut CPpmd8) {
    let mut head: CPpmd8_Node_Ref = 0 as i32 as CPpmd8_Node_Ref;
    let mut prev: *mut CPpmd8_Node_Ref = &mut head;
    let mut i: u32 = 0;
    (*p).GlueCount = ((1 as i32) << 13 as i32) as u32;
    libc::memset(
        (*p).Stamps.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<[u32; 38]>() as u64)
            .try_into()
            .unwrap(),
    );
    /* Order-0 context is always at top UNIT, so we don't need guard NODE at the end.
    All blocks up to p->LoUnit can be free, so we need guard NODE at LoUnit. */
    if (*p).LoUnit != (*p).HiUnit {
        (*((*p).LoUnit as *mut CPpmd8_Node)).Stamp = 0 as i32 as u32
    }
    /* Glue free blocks */
    i = 0 as i32 as u32;
    while i
        < (4 as i32
            + 4 as i32
            + 4 as i32
            + (128 as i32 + 3 as i32
                - 1 as i32 * 4 as i32
                - 2 as i32 * 4 as i32
                - 3 as i32 * 4 as i32)
                / 4 as i32) as u32
    {
        let mut next: CPpmd8_Node_Ref = (*p).FreeList[i as usize];
        (*p).FreeList[i as usize] = 0 as i32 as CPpmd_Void_Ref;
        while next != 0 as i32 as u32 {
            let mut node: *mut CPpmd8_Node = (*p).Base.offset(next as isize) as *mut CPpmd8_Node;
            if (*node).NU != 0 as i32 as u32 {
                let mut node2: *mut CPpmd8_Node = 0 as *mut CPpmd8_Node;
                *prev = next;
                prev = &mut (*node).Next;
                loop {
                    node2 = node.offset((*node).NU as isize);
                    if !((*node2).Stamp == 0xffffffff as u32) {
                        break;
                    }
                    (*node).NU = ((*node).NU as u32).wrapping_add((*node2).NU) as u32 as u32;
                    (*node2).NU = 0 as i32 as u32
                }
            }
            next = (*node).Next
        }
        i = i.wrapping_add(1)
    }
    *prev = 0 as i32 as CPpmd8_Node_Ref;
    /* Fill lists of free blocks */
    while head != 0 as i32 as u32 {
        let mut node_0: *mut CPpmd8_Node = (*p).Base.offset(head as isize) as *mut CPpmd8_Node; /* AllocContext(p); */
        let mut nu: u32 = 0; /* AllocUnits(p, PPMD_NUM_INDEXES - 1); */
        head = (*node_0).Next; /* unused */
        nu = (*node_0).NU;
        if nu == 0 as i32 as u32 {
            continue;
        }
        while nu > 128 as i32 as u32 {
            InsertNode(
                p,
                node_0 as *mut libc::c_void,
                (4 as i32
                    + 4 as i32
                    + 4 as i32
                    + (128 as i32 + 3 as i32
                        - 1 as i32 * 4 as i32
                        - 2 as i32 * 4 as i32
                        - 3 as i32 * 4 as i32)
                        / 4 as i32
                    - 1 as i32) as u32,
            );
            nu = nu.wrapping_sub(128 as i32 as u32);
            node_0 = node_0.offset(128 as i32 as isize)
        }
        i = (*p).Units2Indx[(nu as u64).wrapping_sub(1 as i32 as u64) as usize] as u32;
        if (*p).Indx2Units[i as usize] as u32 != nu {
            i = i.wrapping_sub(1);
            let k: u32 = (*p).Indx2Units[i as usize] as u32;
            InsertNode(
                p,
                node_0.offset(k as isize) as *mut libc::c_void,
                nu.wrapping_sub(k).wrapping_sub(1 as i32 as u32),
            );
        }
        InsertNode(p, node_0 as *mut libc::c_void, i);
    }
}
unsafe extern "C" fn AllocUnitsRare(mut p: *mut CPpmd8, indx: u32) -> *mut libc::c_void {
    let mut i: u32 = 0;
    let mut retVal: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*p).GlueCount == 0 as i32 as u32 {
        GlueFreeBlocks(p);
        if (*p).FreeList[indx as usize] != 0 as i32 as u32 {
            return RemoveNode(p, indx);
        }
    }
    i = indx;
    loop {
        i = i.wrapping_add(1);
        if i == (4 as i32
            + 4 as i32
            + 4 as i32
            + (128 as i32 + 3 as i32
                - 1 as i32 * 4 as i32
                - 2 as i32 * 4 as i32
                - 3 as i32 * 4 as i32)
                / 4 as i32) as u32
        {
            let numBytes: u32 =
                ((*p).Indx2Units[indx as usize] as u32).wrapping_mul(12 as i32 as u32);
            (*p).GlueCount = (*p).GlueCount.wrapping_sub(1);
            return if (*p).UnitsStart.offset_from((*p).Text) as libc::c_long as u32 > numBytes {
                (*p).UnitsStart = (*p).UnitsStart.offset(-(numBytes as isize));
                (*p).UnitsStart
            } else {
                0 as *mut u8
            } as *mut libc::c_void;
        }
        if !((*p).FreeList[i as usize] == 0 as i32 as u32) {
            break;
        }
    }
    retVal = RemoveNode(p, i);
    SplitBlock(p, retVal, i, indx);
    return retVal;
}
unsafe extern "C" fn AllocUnits(mut p: *mut CPpmd8, indx: u32) -> *mut libc::c_void {
    let mut numBytes: u32 = 0;
    if (*p).FreeList[indx as usize] != 0 as i32 as u32 {
        return RemoveNode(p, indx);
    }
    numBytes = ((*p).Indx2Units[indx as usize] as u32).wrapping_mul(12 as i32 as u32);
    if numBytes <= (*p).HiUnit.offset_from((*p).LoUnit) as libc::c_long as u32 {
        let retVal: *mut libc::c_void = (*p).LoUnit as *mut libc::c_void;
        (*p).LoUnit = (*p).LoUnit.offset(numBytes as isize);
        return retVal;
    }
    return AllocUnitsRare(p, indx);
}
unsafe extern "C" fn ShrinkUnits(
    p: *mut CPpmd8,
    oldPtr: *mut libc::c_void,
    oldNU: u32,
    newNU: u32,
) -> *mut libc::c_void {
    let i0: u32 = (*p).Units2Indx[(oldNU as u64).wrapping_sub(1 as i32 as u64) as usize] as u32;
    let i1: u32 = (*p).Units2Indx[(newNU as u64).wrapping_sub(1 as i32 as u64) as usize] as u32;
    if i0 == i1 {
        return oldPtr;
    }
    if (*p).FreeList[i1 as usize] != 0 as i32 as u32 {
        let ptr: *mut libc::c_void = RemoveNode(p, i1);
        let mut d: *mut u32 = ptr as *mut u32;
        let mut z: *const u32 = oldPtr as *const u32;
        let mut n: u32 = newNU;
        loop {
            *d.offset(0 as i32 as isize) = *z.offset(0 as i32 as isize);
            *d.offset(1 as i32 as isize) = *z.offset(1 as i32 as isize);
            *d.offset(2 as i32 as isize) = *z.offset(2 as i32 as isize);
            z = z.offset(3 as i32 as isize);
            d = d.offset(3 as i32 as isize);
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
unsafe extern "C" fn FreeUnits(p: *mut CPpmd8, ptr: *mut libc::c_void, nu: u32) {
    InsertNode(
        p,
        ptr,
        (*p).Units2Indx[(nu as u64).wrapping_sub(1 as i32 as u64) as usize] as u32,
    );
}
unsafe extern "C" fn SpecialFreeUnit(mut p: *mut CPpmd8, ptr: *mut libc::c_void) {
    if ptr as *mut u8 != (*p).UnitsStart {
        InsertNode(p, ptr, 0 as i32 as u32);
    } else {
        (*p).UnitsStart = (*p).UnitsStart.offset(12 as i32 as isize)
    };
}
unsafe extern "C" fn MoveUnitsUp(
    mut p: *mut CPpmd8,
    oldPtr: *mut libc::c_void,
    nu: u32,
) -> *mut libc::c_void {
    let indx: u32 = (*p).Units2Indx[(nu as u64).wrapping_sub(1 as i32 as u64) as usize] as u32;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if oldPtr as *mut u8 > (*p).UnitsStart.offset((16 as i32 * 1024 as i32) as isize)
        || (oldPtr as *mut u8).offset_from((*p).Base) as libc::c_long as u32
            > (*p).FreeList[indx as usize]
    {
        return oldPtr;
    }
    ptr = RemoveNode(p, indx);
    let mut d: *mut u32 = ptr as *mut u32;
    let mut z: *const u32 = oldPtr as *const u32;
    let mut n: u32 = nu;
    loop {
        *d.offset(0 as i32 as isize) = *z.offset(0 as i32 as isize);
        *d.offset(1 as i32 as isize) = *z.offset(1 as i32 as isize);
        *d.offset(2 as i32 as isize) = *z.offset(2 as i32 as isize);
        z = z.offset(3 as i32 as isize);
        d = d.offset(3 as i32 as isize);
        n = n.wrapping_sub(1);
        if !(n != 0) {
            break;
        }
    }
    if oldPtr as *mut u8 != (*p).UnitsStart {
        InsertNode(p, oldPtr, indx);
    } else {
        (*p).UnitsStart = (*p)
            .UnitsStart
            .offset(((*p).Indx2Units[indx as usize] as u32).wrapping_mul(12 as i32 as u32) as isize)
    }
    return ptr;
}
unsafe extern "C" fn ExpandTextArea(mut p: *mut CPpmd8) {
    let mut count: [u32; 38] = [0; 38];
    let mut i: u32 = 0;
    libc::memset(
        count.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<[u32; 38]>() as u64)
            .try_into()
            .unwrap(),
    );
    if (*p).LoUnit != (*p).HiUnit {
        (*((*p).LoUnit as *mut CPpmd8_Node)).Stamp = 0 as i32 as u32
    }
    let mut node: *mut CPpmd8_Node = (*p).UnitsStart as *mut CPpmd8_Node;
    while (*node).Stamp == 0xffffffff as u32 {
        (*node).Stamp = 0 as i32 as u32;
        count[(*p).Units2Indx[((*node).NU as u64).wrapping_sub(1 as i32 as u64) as usize]
            as usize] = count
            [(*p).Units2Indx[((*node).NU as u64).wrapping_sub(1 as i32 as u64) as usize] as usize]
            .wrapping_add(1);
        node = node.offset((*node).NU as isize)
    }
    (*p).UnitsStart = node as *mut u8;
    i = 0 as i32 as u32;
    while i
        < (4 as i32
            + 4 as i32
            + 4 as i32
            + (128 as i32 + 3 as i32
                - 1 as i32 * 4 as i32
                - 2 as i32 * 4 as i32
                - 3 as i32 * 4 as i32)
                / 4 as i32) as u32
    {
        let mut next: *mut CPpmd8_Node_Ref = &mut *(*p).FreeList.as_mut_ptr().offset(i as isize)
            as *mut CPpmd_Void_Ref
            as *mut CPpmd8_Node_Ref;
        while count[i as usize] != 0 as i32 as u32 {
            let mut node_0: *mut CPpmd8_Node = (*p).Base.offset(*next as isize) as *mut CPpmd8_Node;
            while (*node_0).Stamp == 0 as i32 as u32 {
                *next = (*node_0).Next;
                node_0 = (*p).Base.offset(*next as isize) as *mut CPpmd8_Node;
                (*p).Stamps[i as usize] = (*p).Stamps[i as usize].wrapping_sub(1);
                count[i as usize] = count[i as usize].wrapping_sub(1);
                if count[i as usize] == 0 as i32 as u32 {
                    break;
                }
            }
            next = &mut (*node_0).Next
        }
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn SetSuccessor(mut p: *mut CPpmd_State, v: CPpmd_Void_Ref) {
    (*p).SuccessorLow = (v & 0xffff as i32 as u32) as u16;
    (*p).SuccessorHigh = (v >> 16 as i32 & 0xffff as i32 as u32) as u16;
}
unsafe extern "C" fn RestartModel(mut p: *mut CPpmd8) {
    let mut i: u32 = 0;
    let mut k: u32 = 0;
    let mut m: u32 = 0;
    let mut r: u32 = 0;
    libc::memset(
        (*p).FreeList.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<[CPpmd_Void_Ref; 38]>() as u64)
            .try_into()
            .unwrap(),
    );
    libc::memset(
        (*p).Stamps.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<[u32; 38]>() as u64)
            .try_into()
            .unwrap(),
    );
    (*p).Text = (*p)
        .Base
        .offset((*p).AlignOffset as isize)
        .offset(0 as i32 as isize);
    (*p).HiUnit = (*p).Text.offset((*p).Size as isize);
    (*p).UnitsStart = (*p).HiUnit.offset(
        -((*p)
            .Size
            .wrapping_div(8 as i32 as u32)
            .wrapping_div(12 as i32 as u32)
            .wrapping_mul(7 as i32 as u32)
            .wrapping_mul(12 as i32 as u32) as isize),
    );
    (*p).LoUnit = (*p).UnitsStart;
    (*p).GlueCount = 0 as i32 as u32;
    (*p).OrderFall = (*p).MaxOrder;
    (*p).InitRL = -((if (*p).MaxOrder < 12 as i32 as u32 {
        (*p).MaxOrder
    } else {
        12 as i32 as u32
    }) as i32)
        - 1 as i32;
    (*p).RunLength = (*p).InitRL;
    (*p).PrevSuccess = 0 as i32 as u32;
    (*p).HiUnit = (*p).HiUnit.offset(-(12 as i32 as isize));
    (*p).MaxContext = (*p).HiUnit as CTX_PTR;
    (*p).MinContext = (*p).MaxContext;
    (*(*p).MinContext).Suffix = 0 as i32 as CPpmd8_Context_Ref;
    (*(*p).MinContext).NumStats = 255 as i32 as u8;
    (*(*p).MinContext).Flags = 0 as i32 as u8;
    (*(*p).MinContext).SummFreq = (256 as i32 + 1 as i32) as u16;
    (*p).FoundState = (*p).LoUnit as *mut CPpmd_State;
    (*p).LoUnit = (*p)
        .LoUnit
        .offset(((256 as i32 / 2 as i32) as u32).wrapping_mul(12 as i32 as u32) as isize);
    (*(*p).MinContext).Stats =
        ((*p).FoundState as *mut u8).offset_from((*p).Base) as libc::c_long as u32;
    i = 0 as i32 as u32;
    while i < 256 as i32 as u32 {
        let mut s: *mut CPpmd_State = &mut *(*p).FoundState.offset(i as isize) as *mut CPpmd_State;
        (*s).Symbol = i as u8;
        (*s).Freq = 1 as i32 as u8;
        SetSuccessor(s, 0 as i32 as CPpmd_Void_Ref);
        i = i.wrapping_add(1)
    }
    m = 0 as i32 as u32;
    i = m;
    while m < 25 as i32 as u32 {
        while (*p).NS2Indx[i as usize] as u32 == m {
            i = i.wrapping_add(1)
        }
        k = 0 as i32 as u32;
        while k < 8 as i32 as u32 {
            let val: u16 = (((1 as i32) << 7 as i32 + 7 as i32) as u32).wrapping_sub(
                (kInitBinEsc[k as usize] as u32).wrapping_div(i.wrapping_add(1 as i32 as u32)),
            ) as u16;
            let dest: *mut u16 = (*p).BinSumm[m as usize].as_mut_ptr().offset(k as isize);
            r = 0 as i32 as u32;
            while r < 64 as i32 as u32 {
                *dest.offset(r as isize) = val;
                r = r.wrapping_add(8 as i32 as u32)
            }
            k = k.wrapping_add(1)
        }
        m = m.wrapping_add(1)
    }
    m = 0 as i32 as u32;
    i = m;
    while m < 24 as i32 as u32 {
        while (*p).NS2Indx[(i as u64).wrapping_add(3 as i32 as u64) as usize] as u32
            == m.wrapping_add(3 as i32 as u32)
        {
            i = i.wrapping_add(1)
        }
        k = 0 as i32 as u32;
        while k < 32 as i32 as u32 {
            let mut s_0: *mut CPpmd_See = &mut *(*(*p).See.as_mut_ptr().offset(m as isize))
                .as_mut_ptr()
                .offset(k as isize) as *mut CPpmd_See;
            (*s_0).Shift = (7 as i32 - 4 as i32) as u8;
            (*s_0).Summ = ((2 as i32 as u32)
                .wrapping_mul(i)
                .wrapping_add(5 as i32 as u32)
                << (*s_0).Shift as i32) as u16;
            (*s_0).Count = 7 as i32 as u8;
            k = k.wrapping_add(1)
        }
        m = m.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Init(mut p: *mut CPpmd8, maxOrder: u32, restoreMethod: u32) {
    (*p).MaxOrder = maxOrder;
    (*p).RestoreMethod = restoreMethod;
    RestartModel(p);
    (*p).DummySee.Shift = 7 as i32 as u8;
    (*p).DummySee.Summ = 0 as i32 as u16;
    (*p).DummySee.Count = 64 as i32 as u8;
    /* unused */
}
unsafe extern "C" fn Refresh(p: *mut CPpmd8, mut ctx: CTX_PTR, oldNU: u32, scale: u32) {
    let mut i: u32 = (*ctx).NumStats as u32;
    let mut escFreq: u32 = 0;
    let mut sumFreq: u32 = 0;
    let mut flags: u32 = 0;
    let mut s: *mut CPpmd_State = ShrinkUnits(
        p,
        (*p).Base.offset((*ctx).Stats as isize) as *mut libc::c_void as *mut CPpmd_State
            as *mut libc::c_void,
        oldNU,
        i.wrapping_add(2 as i32 as u32) >> 1 as i32,
    ) as *mut CPpmd_State;
    (*ctx).Stats = (s as *mut u8).offset_from((*p).Base) as libc::c_long as u32;
    flags = ((*ctx).Flags as u32
        & (0x10 as i32 as u32).wrapping_add((0x4 as i32 as u32).wrapping_mul(scale)))
    .wrapping_add((0x8 as i32 * ((*s).Symbol as i32 >= 0x40 as i32) as i32) as u32);
    escFreq = ((*ctx).SummFreq as i32 - (*s).Freq as i32) as u32;
    (*s).Freq = (((*s).Freq as u32).wrapping_add(scale) >> scale) as u8;
    sumFreq = (*s).Freq as u32;
    loop {
        s = s.offset(1);
        escFreq = escFreq.wrapping_sub((*s).Freq as u32);
        (*s).Freq = (((*s).Freq as u32).wrapping_add(scale) >> scale) as u8;
        sumFreq = sumFreq.wrapping_add((*s).Freq as u32);
        flags |= (0x8 as i32 * ((*s).Symbol as i32 >= 0x40 as i32) as i32) as u32;
        i = i.wrapping_sub(1);
        if !(i != 0) {
            break;
        }
    }
    (*ctx).SummFreq = sumFreq.wrapping_add(escFreq.wrapping_add(scale) >> scale) as u16;
    (*ctx).Flags = flags as u8;
}
unsafe extern "C" fn SwapStates(t1: *mut CPpmd_State, t2: *mut CPpmd_State) {
    let tmp: CPpmd_State = *t1;
    *t1 = *t2;
    *t2 = tmp;
}
unsafe extern "C" fn CutOff(p: *mut CPpmd8, mut ctx: CTX_PTR, order: u32) -> CPpmd_Void_Ref {
    let mut i: i32 = 0;
    let mut tmp: u32 = 0;
    let mut s: *mut CPpmd_State = 0 as *mut CPpmd_State;
    if (*ctx).NumStats == 0 {
        s = &mut (*ctx).SummFreq as *mut u16 as *mut CPpmd_State;
        if (*p)
            .Base
            .offset(((*s).SuccessorLow as u32 | ((*s).SuccessorHigh as u32) << 16 as i32) as isize)
            as *mut libc::c_void as *mut u8
            >= (*p).UnitsStart
        {
            if order < (*p).MaxOrder {
                SetSuccessor(
                    s,
                    CutOff(
                        p,
                        (*p).Base.offset(
                            ((*s).SuccessorLow as u32 | ((*s).SuccessorHigh as u32) << 16 as i32)
                                as isize,
                        ) as *mut libc::c_void as *mut CPpmd8_Context,
                        order.wrapping_add(1 as i32 as u32),
                    ),
                );
            } else {
                SetSuccessor(s, 0 as i32 as CPpmd_Void_Ref);
            }
            if (*s).SuccessorLow as u32 | ((*s).SuccessorHigh as u32) << 16 as i32 != 0
                || order <= 9 as i32 as u32
            {
                /* O_BOUND */
                return (ctx as *mut u8).offset_from((*p).Base) as libc::c_long as u32;
            }
        }
        SpecialFreeUnit(p, ctx as *mut libc::c_void);
        return 0 as i32 as CPpmd_Void_Ref;
    }
    tmp = ((*ctx).NumStats as u32).wrapping_add(2 as i32 as u32) >> 1 as i32;
    (*ctx).Stats = (MoveUnitsUp(
        p,
        (*p).Base.offset((*ctx).Stats as isize) as *mut libc::c_void as *mut CPpmd_State
            as *mut libc::c_void,
        tmp,
    ) as *mut u8)
        .offset_from((*p).Base) as libc::c_long as u32;
    i = (*ctx).NumStats as i32;
    s = ((*p).Base.offset((*ctx).Stats as isize) as *mut libc::c_void as *mut CPpmd_State)
        .offset(i as isize);
    while s >= (*p).Base.offset((*ctx).Stats as isize) as *mut libc::c_void as *mut CPpmd_State {
        if ((*p)
            .Base
            .offset(((*s).SuccessorLow as u32 | ((*s).SuccessorHigh as u32) << 16 as i32) as isize)
            as *mut libc::c_void as *mut u8)
            < (*p).UnitsStart
        {
            let fresh1 = i;
            i = i - 1;
            let s2: *mut CPpmd_State =
                ((*p).Base.offset((*ctx).Stats as isize) as *mut libc::c_void as *mut CPpmd_State)
                    .offset(fresh1 as isize);
            SetSuccessor(s, 0 as i32 as CPpmd_Void_Ref);
            SwapStates(s, s2);
        } else if order < (*p).MaxOrder {
            SetSuccessor(
                s,
                CutOff(
                    p,
                    (*p).Base.offset(
                        ((*s).SuccessorLow as u32 | ((*s).SuccessorHigh as u32) << 16 as i32)
                            as isize,
                    ) as *mut libc::c_void as *mut CPpmd8_Context,
                    order.wrapping_add(1 as i32 as u32),
                ),
            );
        } else {
            SetSuccessor(s, 0 as i32 as CPpmd_Void_Ref);
        }
        s = s.offset(-1)
    }
    if i != (*ctx).NumStats as i32 && order != 0 {
        (*ctx).NumStats = i as u8;
        s = (*p).Base.offset((*ctx).Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
        if i < 0 as i32 {
            FreeUnits(p, s as *mut libc::c_void, tmp);
            SpecialFreeUnit(p, ctx as *mut libc::c_void);
            return 0 as i32 as CPpmd_Void_Ref;
        }
        if i == 0 as i32 {
            (*ctx).Flags = (((*ctx).Flags as i32 & 0x10 as i32)
                + 0x8 as i32 * ((*s).Symbol as i32 >= 0x40 as i32) as i32)
                as u8;
            *(&mut (*ctx).SummFreq as *mut u16 as *mut CPpmd_State) = *s;
            FreeUnits(p, s as *mut libc::c_void, tmp);
            /* 9.31: the code was fixed. It's was not BUG, if Freq <= MAX_FREQ = 124 */
            (*(&mut (*ctx).SummFreq as *mut u16 as *mut CPpmd_State)).Freq =
                (((*(&mut (*ctx).SummFreq as *mut u16 as *mut CPpmd_State)).Freq as u32)
                    .wrapping_add(11 as i32 as u32)
                    >> 3 as i32) as u8
        } else {
            Refresh(
                p,
                ctx,
                tmp,
                ((*ctx).SummFreq as i32 > 16 as i32 * i) as i32 as u32,
            );
        }
    }
    return (ctx as *mut u8).offset_from((*p).Base) as libc::c_long as u32;
}
unsafe extern "C" fn GetUsedMemory(p: *const CPpmd8) -> u32 {
    let mut v: u32 = 0 as i32 as u32;
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while i
        < (4 as i32
            + 4 as i32
            + 4 as i32
            + (128 as i32 + 3 as i32
                - 1 as i32 * 4 as i32
                - 2 as i32 * 4 as i32
                - 3 as i32 * 4 as i32)
                / 4 as i32) as u32
    {
        v = (v as u32)
            .wrapping_add((*p).Stamps[i as usize].wrapping_mul((*p).Indx2Units[i as usize] as u32))
            as u32 as u32;
        i = i.wrapping_add(1)
    }
    return (*p)
        .Size
        .wrapping_sub((*p).HiUnit.offset_from((*p).LoUnit) as libc::c_long as u32)
        .wrapping_sub((*p).UnitsStart.offset_from((*p).Text) as libc::c_long as u32)
        .wrapping_sub(v.wrapping_mul(12 as i32 as u32));
}
unsafe extern "C" fn RestoreModel(mut p: *mut CPpmd8, c1: CTX_PTR) {
    let mut c: CTX_PTR = 0 as *mut CPpmd8_Context;
    let mut s: *mut CPpmd_State = 0 as *mut CPpmd_State;
    (*p).Text = (*p)
        .Base
        .offset((*p).AlignOffset as isize)
        .offset(0 as i32 as isize);
    c = (*p).MaxContext;
    while c != c1 {
        (*c).NumStats = (*c).NumStats.wrapping_sub(1);
        if (*c).NumStats as i32 == 0 as i32 {
            s = (*p).Base.offset((*c).Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
            (*c).Flags = (((*c).Flags as i32 & 0x10 as i32)
                + 0x8 as i32 * ((*s).Symbol as i32 >= 0x40 as i32) as i32)
                as u8;
            *(&mut (*c).SummFreq as *mut u16 as *mut CPpmd_State) = *s;
            SpecialFreeUnit(p, s as *mut libc::c_void);
            (*(&mut (*c).SummFreq as *mut u16 as *mut CPpmd_State)).Freq =
                (((*(&mut (*c).SummFreq as *mut u16 as *mut CPpmd_State)).Freq as u32)
                    .wrapping_add(11 as i32 as u32)
                    >> 3 as i32) as u8
        } else {
            Refresh(
                p,
                c,
                ((*c).NumStats as i32 + 3 as i32 >> 1 as i32) as u32,
                0 as i32 as u32,
            );
        }
        c = (*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context
    }
    while c != (*p).MinContext {
        if (*c).NumStats == 0 {
            (*(&mut (*c).SummFreq as *mut u16 as *mut CPpmd_State)).Freq =
                ((*(&mut (*c).SummFreq as *mut u16 as *mut CPpmd_State)).Freq as i32
                    - ((*(&mut (*c).SummFreq as *mut u16 as *mut CPpmd_State)).Freq as i32
                        >> 1 as i32)) as u8
        } else {
            (*c).SummFreq = ((*c).SummFreq as i32 + 4 as i32) as u16;
            if (*c).SummFreq as i32 > 128 as i32 + 4 as i32 * (*c).NumStats as i32 {
                Refresh(
                    p,
                    c,
                    ((*c).NumStats as i32 + 2 as i32 >> 1 as i32) as u32,
                    1 as i32 as u32,
                );
            }
        }
        c = (*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context
    }
    if (*p).RestoreMethod == PPMD8_RESTORE_METHOD_RESTART as i32 as u32
        || GetUsedMemory(p) < (*p).Size >> 1 as i32
    {
        RestartModel(p);
    } else {
        while (*(*p).MaxContext).Suffix != 0 {
            (*p).MaxContext = (*p).Base.offset((*(*p).MaxContext).Suffix as isize)
                as *mut libc::c_void as *mut CPpmd8_Context
        }
        loop {
            CutOff(p, (*p).MaxContext, 0 as i32 as u32);
            ExpandTextArea(p);
            if !(GetUsedMemory(p) > (3 as i32 as u32).wrapping_mul((*p).Size >> 2 as i32)) {
                break;
            }
        }
        (*p).GlueCount = 0 as i32 as u32;
        (*p).OrderFall = (*p).MaxOrder
    };
}
unsafe extern "C" fn CreateSuccessors(
    mut p: *mut CPpmd8,
    skip: i32,
    mut s1: *mut CPpmd_State,
    mut c: CTX_PTR,
) -> CTX_PTR {
    let mut upState: CPpmd_State = CPpmd_State {
        Symbol: 0,
        Freq: 0,
        SuccessorLow: 0,
        SuccessorHigh: 0,
    };
    let mut flags: u8 = 0;
    let upBranch: CPpmd_Byte_Ref = (*(*p).FoundState).SuccessorLow as u32
        | ((*(*p).FoundState).SuccessorHigh as u32) << 16 as i32;
    /* fixed over Shkarin's code. Maybe it could work without + 1 too. */
    let mut ps: [*mut CPpmd_State; 17] = [0 as *mut CPpmd_State; 17];
    let mut numPs: u32 = 0 as i32 as u32;
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
        } else if (*c).NumStats as i32 != 0 as i32 {
            s = (*p).Base.offset((*c).Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
            while (*s).Symbol as i32 != (*(*p).FoundState).Symbol as i32 {
                s = s.offset(1)
            }
            if ((*s).Freq as i32) < 124 as i32 - 9 as i32 {
                (*s).Freq = (*s).Freq.wrapping_add(1);
                (*c).SummFreq = (*c).SummFreq.wrapping_add(1)
            }
        } else {
            s = &mut (*c).SummFreq as *mut u16 as *mut CPpmd_State;
            (*s).Freq = ((*s).Freq as i32
                + (((*((*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void
                    as *mut CPpmd8_Context))
                    .NumStats
                    == 0) as i32
                    & (((*s).Freq as i32) < 24 as i32) as i32)) as u8
        }
        successor = (*s).SuccessorLow as u32 | ((*s).SuccessorHigh as u32) << 16 as i32;
        if successor != upBranch {
            c = (*p).Base.offset(successor as isize) as *mut libc::c_void as *mut CPpmd8_Context;
            if numPs == 0 as i32 as u32 {
                return c;
            }
            break;
        } else {
            let fresh3 = numPs;
            numPs = numPs.wrapping_add(1);
            ps[fresh3 as usize] = s
        }
    }
    upState.Symbol = *((*p).Base.offset(upBranch as isize) as *mut libc::c_void as *const u8);
    SetSuccessor(&mut upState, upBranch.wrapping_add(1 as i32 as u32));
    flags = (0x10 as i32 * ((*(*p).FoundState).Symbol as i32 >= 0x40 as i32) as i32
        + 0x8 as i32 * (upState.Symbol as i32 >= 0x40 as i32) as i32) as u8;
    if (*c).NumStats as i32 == 0 as i32 {
        upState.Freq = (*(&mut (*c).SummFreq as *mut u16 as *mut CPpmd_State)).Freq
    } else {
        let mut cf: u32 = 0;
        let mut s0: u32 = 0;
        let mut s_0: *mut CPpmd_State = 0 as *mut CPpmd_State;
        s_0 = (*p).Base.offset((*c).Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
        while (*s_0).Symbol as i32 != upState.Symbol as i32 {
            s_0 = s_0.offset(1)
        }
        cf = ((*s_0).Freq as i32 - 1 as i32) as u32;
        s0 = (((*c).SummFreq as i32 - (*c).NumStats as i32) as u32).wrapping_sub(cf);
        upState.Freq = (1 as i32 as u32).wrapping_add(if (2 as i32 as u32).wrapping_mul(cf) <= s0 {
            ((5 as i32 as u32).wrapping_mul(cf) > s0) as i32 as u32
        } else {
            cf.wrapping_add((2 as i32 as u32).wrapping_mul(s0))
                .wrapping_sub(3 as i32 as u32)
                .wrapping_div(s0)
        }) as u8
    }
    loop {
        /* Create Child */
        let mut c1: CTX_PTR = 0 as *mut CPpmd8_Context; /* = AllocContext(p); */
        if (*p).HiUnit != (*p).LoUnit {
            (*p).HiUnit = (*p).HiUnit.offset(-(12 as i32 as isize)); /* check it */
            c1 = (*p).HiUnit as CTX_PTR
        } else if (*p).FreeList[0 as i32 as usize] != 0 as i32 as u32 {
            c1 = RemoveNode(p, 0 as i32 as u32) as CTX_PTR
        } else {
            c1 = AllocUnitsRare(p, 0 as i32 as u32) as CTX_PTR;
            if c1.is_null() {
                return 0 as CTX_PTR;
            }
        }
        (*c1).NumStats = 0 as i32 as u8;
        (*c1).Flags = flags;
        *(&mut (*c1).SummFreq as *mut u16 as *mut CPpmd_State) = upState;
        (*c1).Suffix = (c as *mut u8).offset_from((*p).Base) as libc::c_long as u32;
        numPs = numPs.wrapping_sub(1);
        SetSuccessor(
            ps[numPs as usize],
            (c1 as *mut u8).offset_from((*p).Base) as libc::c_long as u32,
        );
        c = c1;
        if !(numPs != 0 as i32 as u32) {
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
    let c1: CTX_PTR = c;
    let upBranch: CPpmd_Void_Ref = (*p).Text.offset_from((*p).Base) as libc::c_long as u32;
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
                if (*s).Symbol as i32 != (*(*p).FoundState).Symbol as i32 {
                    loop {
                        s = s.offset(1);
                        if !((*s).Symbol as i32 != (*(*p).FoundState).Symbol as i32) {
                            break;
                        }
                    }
                }
                if ((*s).Freq as i32) < 124 as i32 - 9 as i32 {
                    (*s).Freq = ((*s).Freq as i32 + 2 as i32) as u8;
                    (*c).SummFreq = ((*c).SummFreq as i32 + 2 as i32) as u16
                }
            } else {
                s = &mut (*c).SummFreq as *mut u16 as *mut CPpmd_State;
                (*s).Freq = ((*s).Freq as i32 + (((*s).Freq as i32) < 32 as i32) as i32) as u8
            }
        }
        if (*s).SuccessorLow as u32 | ((*s).SuccessorHigh as u32) << 16 as i32 != 0 {
            break;
        }
        SetSuccessor(s, upBranch);
        (*p).OrderFall = (*p).OrderFall.wrapping_add(1)
    }
    if (*s).SuccessorLow as u32 | ((*s).SuccessorHigh as u32) << 16 as i32 <= upBranch {
        let mut successor: CTX_PTR = 0 as *mut CPpmd8_Context;
        let s2: *mut CPpmd_State = (*p).FoundState;
        (*p).FoundState = s;
        successor = CreateSuccessors(p, 0 as i32, 0 as *mut CPpmd_State, c);
        if successor.is_null() {
            SetSuccessor(s, 0 as i32 as CPpmd_Void_Ref);
        } else {
            SetSuccessor(
                s,
                (successor as *mut u8).offset_from((*p).Base) as libc::c_long as u32,
            );
        }
        (*p).FoundState = s2
    }
    if (*p).OrderFall == 1 as i32 as u32 && c1 == (*p).MaxContext {
        SetSuccessor(
            (*p).FoundState,
            (*s).SuccessorLow as u32 | ((*s).SuccessorHigh as u32) << 16 as i32,
        );
        (*p).Text = (*p).Text.offset(-1)
    }
    if (*s).SuccessorLow as u32 | ((*s).SuccessorHigh as u32) << 16 as i32 == 0 as i32 as u32 {
        return 0 as CTX_PTR;
    }
    return (*p)
        .Base
        .offset(((*s).SuccessorLow as u32 | ((*s).SuccessorHigh as u32) << 16 as i32) as isize)
        as *mut libc::c_void as *mut CPpmd8_Context;
}
unsafe extern "C" fn UpdateModel(mut p: *mut CPpmd8) {
    let mut successor: CPpmd_Void_Ref = 0;
    let mut fSuccessor: CPpmd_Void_Ref = (*(*p).FoundState).SuccessorLow as u32
        | ((*(*p).FoundState).SuccessorHigh as u32) << 16 as i32;
    let mut c: CTX_PTR = 0 as *mut CPpmd8_Context;
    let mut s0: u32 = 0;
    let mut ns: u32 = 0;
    let fFreq: u32 = (*(*p).FoundState).Freq as u32;
    let mut flag: u8 = 0;
    let fSymbol: u8 = (*(*p).FoundState).Symbol;
    let mut s: *mut CPpmd_State = 0 as *mut CPpmd_State;
    if ((*(*p).FoundState).Freq as i32) < 124 as i32 / 4 as i32
        && (*(*p).MinContext).Suffix != 0 as i32 as u32
    {
        c = (*p).Base.offset((*(*p).MinContext).Suffix as isize) as *mut libc::c_void
            as *mut CPpmd8_Context;
        if (*c).NumStats as i32 == 0 as i32 {
            s = &mut (*c).SummFreq as *mut u16 as *mut CPpmd_State;
            if ((*s).Freq as i32) < 32 as i32 {
                (*s).Freq = (*s).Freq.wrapping_add(1)
            }
        } else {
            s = (*p).Base.offset((*c).Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
            if (*s).Symbol as i32 != (*(*p).FoundState).Symbol as i32 {
                loop {
                    s = s.offset(1);
                    if !((*s).Symbol as i32 != (*(*p).FoundState).Symbol as i32) {
                        break;
                    }
                }
                if (*s.offset(0 as i32 as isize)).Freq as i32
                    >= (*s.offset(-(1 as i32) as isize)).Freq as i32
                {
                    SwapStates(
                        &mut *s.offset(0 as i32 as isize),
                        &mut *s.offset(-(1 as i32) as isize),
                    );
                    s = s.offset(-1)
                }
            }
            if ((*s).Freq as i32) < 124 as i32 - 9 as i32 {
                (*s).Freq = ((*s).Freq as i32 + 2 as i32) as u8;
                (*c).SummFreq = ((*c).SummFreq as i32 + 2 as i32) as u16
            }
        }
    }
    c = (*p).MaxContext;
    if (*p).OrderFall == 0 as i32 as u32 && fSuccessor != 0 {
        let cs: CTX_PTR = CreateSuccessors(p, 1 as i32, s, (*p).MinContext);
        if cs.is_null() {
            SetSuccessor((*p).FoundState, 0 as i32 as CPpmd_Void_Ref);
            RestoreModel(p, c);
        } else {
            SetSuccessor(
                (*p).FoundState,
                (cs as *mut u8).offset_from((*p).Base) as libc::c_long as u32,
            );
            (*p).MaxContext = cs
        }
        return;
    }
    let fresh4 = (*p).Text;
    (*p).Text = (*p).Text.offset(1);
    *fresh4 = (*(*p).FoundState).Symbol;
    successor = (*p).Text.offset_from((*p).Base) as libc::c_long as u32;
    if (*p).Text >= (*p).UnitsStart {
        RestoreModel(p, c);
        return;
    }
    if fSuccessor == 0 {
        let cs_0: CTX_PTR = ReduceOrder(p, s, (*p).MinContext);
        if cs_0.is_null() {
            RestoreModel(p, c);
            return;
        }
        fSuccessor = (cs_0 as *mut u8).offset_from((*p).Base) as libc::c_long as u32
    } else if ((*p).Base.offset(fSuccessor as isize) as *mut libc::c_void as *mut u8)
        < (*p).UnitsStart
    {
        let cs_1: CTX_PTR = CreateSuccessors(p, 0 as i32, s, (*p).MinContext);
        if cs_1.is_null() {
            RestoreModel(p, c);
            return;
        }
        fSuccessor = (cs_1 as *mut u8).offset_from((*p).Base) as libc::c_long as u32
    }
    (*p).OrderFall = (*p).OrderFall.wrapping_sub(1);
    if (*p).OrderFall == 0 {
        successor = fSuccessor;
        (*p).Text = (*p)
            .Text
            .offset(-(((*p).MaxContext != (*p).MinContext) as i32 as isize))
    }
    ns = (*(*p).MinContext).NumStats as u32;
    s0 = ((*(*p).MinContext).SummFreq as u32)
        .wrapping_sub(ns)
        .wrapping_sub(fFreq);
    flag = (0x8 as i32 * (fSymbol as i32 >= 0x40 as i32) as i32) as u8;
    while c != (*p).MinContext {
        let mut ns1: u32 = 0;
        let mut cf: u32 = 0;
        let mut sf: u32 = 0;
        ns1 = (*c).NumStats as u32;
        if ns1 != 0 as i32 as u32 {
            if ns1 & 1 as i32 as u32 != 0 as i32 as u32 {
                /* Expand for one UNIT */
                let oldNU: u32 = ns1.wrapping_add(1 as i32 as u32) >> 1 as i32;
                let i: u32 =
                    (*p).Units2Indx[(oldNU as u64).wrapping_sub(1 as i32 as u64) as usize] as u32;
                if i != (*p).Units2Indx[(oldNU as u64)
                    .wrapping_add(1 as i32 as u64)
                    .wrapping_sub(1 as i32 as u64) as usize] as u32
                {
                    let ptr: *mut libc::c_void = AllocUnits(p, i.wrapping_add(1 as i32 as u32));
                    let mut oldPtr: *mut libc::c_void = 0 as *mut libc::c_void;
                    if ptr.is_null() {
                        RestoreModel(p, c);
                        return;
                    }
                    oldPtr = (*p).Base.offset((*c).Stats as isize) as *mut libc::c_void
                        as *mut CPpmd_State as *mut libc::c_void;
                    let mut d: *mut u32 = ptr as *mut u32;
                    let mut z: *const u32 = oldPtr as *const u32;
                    let mut n: u32 = oldNU;
                    loop {
                        *d.offset(0 as i32 as isize) = *z.offset(0 as i32 as isize);
                        *d.offset(1 as i32 as isize) = *z.offset(1 as i32 as isize);
                        *d.offset(2 as i32 as isize) = *z.offset(2 as i32 as isize);
                        z = z.offset(3 as i32 as isize);
                        d = d.offset(3 as i32 as isize);
                        n = n.wrapping_sub(1);
                        if !(n != 0) {
                            break;
                        }
                    }
                    InsertNode(p, oldPtr, i);
                    (*c).Stats = (ptr as *mut u8).offset_from((*p).Base) as libc::c_long as u32
                }
            }
            (*c).SummFreq = ((*c).SummFreq as i32
                + ((3 as i32 as u32)
                    .wrapping_mul(ns1)
                    .wrapping_add(1 as i32 as u32)
                    < ns) as i32) as u16
        } else {
            let mut s2: *mut CPpmd_State = AllocUnits(p, 0 as i32 as u32) as *mut CPpmd_State;
            if s2.is_null() {
                RestoreModel(p, c);
                return;
            }
            *s2 = *(&mut (*c).SummFreq as *mut u16 as *mut CPpmd_State);
            (*c).Stats = (s2 as *mut u8).offset_from((*p).Base) as libc::c_long as u32;
            if ((*s2).Freq as i32) < 124 as i32 / 4 as i32 - 1 as i32 {
                (*s2).Freq = (((*s2).Freq as i32) << 1 as i32) as u8
            } else {
                (*s2).Freq = (124 as i32 - 4 as i32) as u8
            }
            (*c).SummFreq = ((*s2).Freq as u32)
                .wrapping_add((*p).InitEsc)
                .wrapping_add((ns > 2 as i32 as u32) as i32 as u32)
                as u16
        }
        cf = (2 as i32 as u32)
            .wrapping_mul(fFreq)
            .wrapping_mul(((*c).SummFreq as i32 + 6 as i32) as u32);
        sf = s0.wrapping_add((*c).SummFreq as u32);
        if cf < (6 as i32 as u32).wrapping_mul(sf) {
            cf = (1 as i32 + (cf > sf) as i32 + (cf >= (4 as i32 as u32).wrapping_mul(sf)) as i32)
                as u32;
            (*c).SummFreq = ((*c).SummFreq as i32 + 4 as i32) as u16
        } else {
            cf = (4 as i32
                + (cf > (9 as i32 as u32).wrapping_mul(sf)) as i32
                + (cf > (12 as i32 as u32).wrapping_mul(sf)) as i32
                + (cf > (15 as i32 as u32).wrapping_mul(sf)) as i32) as u32;
            (*c).SummFreq = ((*c).SummFreq as u32).wrapping_add(cf) as u16
        }
        let mut s2_0: *mut CPpmd_State =
            ((*p).Base.offset((*c).Stats as isize) as *mut libc::c_void as *mut CPpmd_State)
                .offset(ns1 as isize)
                .offset(1 as i32 as isize);
        SetSuccessor(s2_0, successor);
        (*s2_0).Symbol = fSymbol;
        (*s2_0).Freq = cf as u8;
        (*c).Flags = ((*c).Flags as i32 | flag as i32) as u8;
        (*c).NumStats = ns1.wrapping_add(1 as i32 as u32) as u8;
        c = (*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context
    }
    (*p).MinContext =
        (*p).Base.offset(fSuccessor as isize) as *mut libc::c_void as *mut CPpmd8_Context;
    (*p).MaxContext = (*p).MinContext;
}
unsafe extern "C" fn Rescale(mut p: *mut CPpmd8) {
    let mut i: u32 = 0;
    let mut adder: u32 = 0;
    let mut sumFreq: u32 = 0;
    let mut escFreq: u32 = 0;
    let stats: *mut CPpmd_State = (*p).Base.offset((*(*p).MinContext).Stats as isize)
        as *mut libc::c_void as *mut CPpmd_State;
    let mut s: *mut CPpmd_State = (*p).FoundState;
    let tmp: CPpmd_State = *s;
    while s != stats {
        *s.offset(0 as i32 as isize) = *s.offset(-(1 as i32) as isize);
        s = s.offset(-1)
    }
    *s = tmp;
    escFreq = ((*(*p).MinContext).SummFreq as i32 - (*s).Freq as i32) as u32;
    (*s).Freq = ((*s).Freq as i32 + 4 as i32) as u8;
    adder = ((*p).OrderFall != 0 as i32 as u32) as i32 as u32;
    (*s).Freq = (((*s).Freq as u32).wrapping_add(adder) >> 1 as i32) as u8;
    sumFreq = (*s).Freq as u32;
    i = (*(*p).MinContext).NumStats as u32;
    loop {
        s = s.offset(1);
        escFreq = escFreq.wrapping_sub((*s).Freq as u32);
        (*s).Freq = (((*s).Freq as u32).wrapping_add(adder) >> 1 as i32) as u8;
        sumFreq = sumFreq.wrapping_add((*s).Freq as u32);
        if (*s.offset(0 as i32 as isize)).Freq as i32
            > (*s.offset(-(1 as i32) as isize)).Freq as i32
        {
            let mut s1: *mut CPpmd_State = s;
            let tmp_0: CPpmd_State = *s1;
            loop {
                *s1.offset(0 as i32 as isize) = *s1.offset(-(1 as i32) as isize);
                s1 = s1.offset(-1);
                if !(s1 != stats
                    && tmp_0.Freq as i32 > (*s1.offset(-(1 as i32) as isize)).Freq as i32)
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
    if (*s).Freq as i32 == 0 as i32 {
        let numStats: u32 = (*(*p).MinContext).NumStats as u32;
        let mut n0: u32 = 0;
        let mut n1: u32 = 0;
        loop {
            i = i.wrapping_add(1);
            s = s.offset(-1);
            if !((*s).Freq as i32 == 0 as i32) {
                break;
            }
        }
        escFreq = escFreq.wrapping_add(i);
        (*(*p).MinContext).NumStats = ((*(*p).MinContext).NumStats as u32).wrapping_sub(i) as u8;
        if (*(*p).MinContext).NumStats as i32 == 0 as i32 {
            let mut tmp_1: CPpmd_State = *stats;
            tmp_1.Freq = ((2 as i32 * tmp_1.Freq as i32) as u32)
                .wrapping_add(escFreq)
                .wrapping_sub(1 as i32 as u32)
                .wrapping_div(escFreq) as u8;
            if tmp_1.Freq as i32 > 124 as i32 / 3 as i32 {
                tmp_1.Freq = (124 as i32 / 3 as i32) as u8
            }
            InsertNode(
                p,
                stats as *mut libc::c_void,
                (*p).Units2Indx[((numStats.wrapping_add(2 as i32 as u32) >> 1 as i32) as u64)
                    .wrapping_sub(1 as i32 as u64) as usize] as u32,
            );
            (*(*p).MinContext).Flags = (((*(*p).MinContext).Flags as i32 & 0x10 as i32)
                + 0x8 as i32 * (tmp_1.Symbol as i32 >= 0x40 as i32) as i32)
                as u8;
            (*p).FoundState = &mut (*(*p).MinContext).SummFreq as *mut u16 as *mut CPpmd_State;
            *(*p).FoundState = tmp_1;
            return;
        }
        n0 = numStats.wrapping_add(2 as i32 as u32) >> 1 as i32;
        n1 = ((*(*p).MinContext).NumStats as i32 + 2 as i32 >> 1 as i32) as u32;
        if n0 != n1 {
            (*(*p).MinContext).Stats =
                (ShrinkUnits(p, stats as *mut libc::c_void, n0, n1) as *mut u8)
                    .offset_from((*p).Base) as libc::c_long as u32
        }
        (*(*p).MinContext).Flags = ((*(*p).MinContext).Flags as i32 & !(0x8 as i32)) as u8;
        s = (*p).Base.offset((*(*p).MinContext).Stats as isize) as *mut libc::c_void
            as *mut CPpmd_State;
        (*(*p).MinContext).Flags = ((*(*p).MinContext).Flags as i32
            | 0x8 as i32 * ((*s).Symbol as i32 >= 0x40 as i32) as i32)
            as u8;
        i = (*(*p).MinContext).NumStats as u32;
        loop {
            s = s.offset(1);
            (*(*p).MinContext).Flags = ((*(*p).MinContext).Flags as i32
                | 0x8 as i32 * ((*s).Symbol as i32 >= 0x40 as i32) as i32)
                as u8;
            i = i.wrapping_sub(1);
            if !(i != 0) {
                break;
            }
        }
    }
    (*(*p).MinContext).SummFreq = sumFreq
        .wrapping_add(escFreq)
        .wrapping_sub(escFreq >> 1 as i32) as u16;
    (*(*p).MinContext).Flags = ((*(*p).MinContext).Flags as i32 | 0x4 as i32) as u8;
    (*p).FoundState = (*p).Base.offset((*(*p).MinContext).Stats as isize) as *mut libc::c_void
        as *mut CPpmd_State;
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_MakeEscFreq(
    p: *mut CPpmd8,
    numMasked1: u32,
    escFreq: *mut u32,
) -> *mut CPpmd_See {
    let mut see: *mut CPpmd_See = 0 as *mut CPpmd_See;
    if (*(*p).MinContext).NumStats as i32 != 0xff as i32 {
        see = (*p).See[((*p).NS2Indx
            [((*(*p).MinContext).NumStats as u32 as u64).wrapping_add(2 as i32 as u64) as usize]
            as u32 as u64)
            .wrapping_sub(3 as i32 as u64) as usize]
            .as_mut_ptr()
            .offset(
                ((*(*p).MinContext).SummFreq as u32
                    > (11 as i32 as u32).wrapping_mul(
                        ((*(*p).MinContext).NumStats as u32).wrapping_add(1 as i32 as u32),
                    )) as i32 as isize,
            )
            .offset(
                (2 as i32 as u32).wrapping_mul(
                    ((2 as i32 as u32).wrapping_mul((*(*p).MinContext).NumStats as u32)
                        < ((*((*p).Base.offset((*(*p).MinContext).Suffix as isize)
                            as *mut libc::c_void
                            as *mut CPpmd8_Context))
                            .NumStats as u32)
                            .wrapping_add(numMasked1)) as i32 as u32,
                ) as isize,
            )
            .offset((*(*p).MinContext).Flags as i32 as isize);
        let r: u32 = ((*see).Summ as i32 >> (*see).Shift as i32) as u32;
        (*see).Summ = ((*see).Summ as u32).wrapping_sub(r) as u16;
        *escFreq = r.wrapping_add((r == 0 as i32 as u32) as i32 as u32)
    } else {
        see = &mut (*p).DummySee;
        *escFreq = 1
    }
    return see;
}
unsafe extern "C" fn NextContext(mut p: *mut CPpmd8) {
    let c: CTX_PTR = (*p).Base.offset(
        ((*(*p).FoundState).SuccessorLow as u32
            | ((*(*p).FoundState).SuccessorHigh as u32) << 16 as i32) as isize,
    ) as *mut libc::c_void as *mut CPpmd8_Context;
    if (*p).OrderFall == 0 as i32 as u32 && c as *mut u8 >= (*p).UnitsStart {
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
    (*s).Freq = ((*s).Freq as i32 + 4 as i32) as u8;
    (*(*p).MinContext).SummFreq = ((*(*p).MinContext).SummFreq as i32 + 4 as i32) as u16;
    if (*s.offset(0 as i32 as isize)).Freq as i32 > (*s.offset(-(1 as i32) as isize)).Freq as i32 {
        SwapStates(
            &mut *s.offset(0 as i32 as isize),
            &mut *s.offset(-(1 as i32) as isize),
        );
        s = s.offset(-1);
        (*p).FoundState = s;
        if (*s).Freq as i32 > 124 as i32 {
            Rescale(p);
        }
    }
    NextContext(p);
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Update1_0(mut p: *mut CPpmd8) {
    (*p).PrevSuccess = (2 as i32 * (*(*p).FoundState).Freq as i32
        >= (*(*p).MinContext).SummFreq as i32) as i32 as u32;
    (*p).RunLength = ((*p).RunLength as u32).wrapping_add((*p).PrevSuccess) as i32;
    (*(*p).MinContext).SummFreq = ((*(*p).MinContext).SummFreq as i32 + 4 as i32) as u16;
    (*(*p).FoundState).Freq = ((*(*p).FoundState).Freq as i32 + 4 as i32) as u8;
    if (*(*p).FoundState).Freq as i32 > 124 as i32 {
        Rescale(p);
    }
    NextContext(p);
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_UpdateBin(mut p: *mut CPpmd8) {
    (*(*p).FoundState).Freq = ((*(*p).FoundState).Freq as i32
        + (((*(*p).FoundState).Freq as i32) < 196 as i32) as i32)
        as u8;
    (*p).PrevSuccess = 1 as i32 as u32;
    (*p).RunLength += 1;
    NextContext(p);
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Update2(mut p: *mut CPpmd8) {
    (*(*p).MinContext).SummFreq = ((*(*p).MinContext).SummFreq as i32 + 4 as i32) as u16;
    (*(*p).FoundState).Freq = ((*(*p).FoundState).Freq as i32 + 4 as i32) as u8;
    if (*(*p).FoundState).Freq as i32 > 124 as i32 {
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
pub unsafe extern "C" fn Ppmd8_RangeDec_Init(mut p: *mut CPpmd8) -> i32 {
    let mut i: u32 = 0;
    (*p).Low = 0 as i32 as u32;
    (*p).Range = 0xffffffff as u32;
    (*p).Code = 0 as i32 as u32;
    while i < 4 as i32 as u32 {
        (*p).Code = (*p).Code << 8 as i32
            | (*(*p).Stream.In).Read.expect("non-null function pointer")((*p).Stream.In) as u32;
        i = i.wrapping_add(1)
    }
    return ((*p).Code < 0xffffffff as u32) as i32;
}

unsafe extern "C" fn pmalloc(_: ISzAllocPtr, size: u64) -> *mut libc::c_void {
    return malloc(size.try_into().unwrap()); /* EndMark */
}
unsafe extern "C" fn pfree(_: ISzAllocPtr, addr: *mut libc::c_void) {
    free(addr);
}
pub static mut ialloc: ISzAlloc = {
    {
        let init = ISzAlloc {
            Alloc: Some(
                pmalloc as unsafe extern "C" fn(_: ISzAllocPtr, _: u64) -> *mut libc::c_void,
            ),
            Free: Some(pfree as unsafe extern "C" fn(_: ISzAllocPtr, _: *mut libc::c_void) -> ()),
        };
        init
    }
};
