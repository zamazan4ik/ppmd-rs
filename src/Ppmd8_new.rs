extern "C" {

    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
/* trick for Unix */
pub type Byte = libc::c_uchar;
pub type UInt16 = libc::c_ushort;
pub type Int32 = libc::c_int;
pub type UInt32 = libc::c_uint;
pub type BoolInt = libc::c_int;
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
2021-04-13 : Igor Pavlov : Public domain
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
    pub Successor_0: UInt16,
    pub Successor_1: UInt16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct CPpmd_State2_ {
    pub Symbol: Byte,
    pub Freq: Byte,
}
pub type CPpmd_State2 = CPpmd_State2_;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct CPpmd_State4_ {
    pub Successor_0: UInt16,
    pub Successor_1: UInt16,
}
pub type CPpmd_State4 = CPpmd_State4_;
/*
   PPMD code can write full CPpmd_State structure data to CPpmd*_Context
      at (byte offset = 2) instead of some fields of original CPpmd*_Context structure.

   If we use pointers to different types, but that point to shared
   memory space, we can have aliasing problem (strict aliasing).

   XLC compiler in -O2 mode can change the order of memory write instructions
   in relation to read instructions, if we have use pointers to different types.

   To solve that aliasing problem we use combined CPpmd*_Context structure
   with unions that contain the fields from both structures:
   the original CPpmd*_Context and CPpmd_State.
   So we can access the fields from both structures via one pointer,
   and the compiler doesn't change the order of write instructions
   in relation to read instructions.

   If we don't use memory write instructions to shared memory in
   some local code, and we use only reading instructions (read only),
   then probably it's safe to use pointers to different types for reading.
*/
// PPMD_32BIT
pub type CPpmd_State_Ref = UInt32;
pub type CPpmd_Void_Ref = UInt32;
pub type CPpmd_Byte_Ref = UInt32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd8_Context_ {
    pub NumStats: Byte,
    pub Flags: Byte,
    pub Union2: C2RustUnnamed_0,
    pub Union4: C2RustUnnamed,
    pub Suffix: CPpmd8_Context_Ref,
}
pub type CPpmd8_Context_Ref = UInt32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub Stats: CPpmd_State_Ref,
    pub State4: CPpmd_State4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub SummFreq: UInt16,
    pub State2: CPpmd_State2,
}
/* Ppmd8.h -- Ppmd8 (PPMdI) compression codec
2021-04-13 : Igor Pavlov : Public domain
This code is based on:
  PPMd var.I (2002): Dmitry Shkarin : Public domain
  Carryless rangecoder (1999): Dmitry Subbotin : Public domain */
// MY_CPU_pragma_pack_push_1
pub type CPpmd8_Context = CPpmd8_Context_;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const PPMD8_RESTORE_METHOD_UNSUPPPORTED: C2RustUnnamed_1 = 2;
pub const PPMD8_RESTORE_METHOD_CUT_OFF: C2RustUnnamed_1 = 1;
pub const PPMD8_RESTORE_METHOD_RESTART: C2RustUnnamed_1 = 0;
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
    pub RestoreMethod: libc::c_uint,
    pub RunLength: Int32,
    pub InitRL: Int32,
    pub Size: UInt32,
    pub GlueCount: UInt32,
    pub AlignOffset: UInt32,
    pub Base: *mut Byte,
    pub LoUnit: *mut Byte,
    pub HiUnit: *mut Byte,
    pub Text: *mut Byte,
    pub UnitsStart: *mut Byte,
    pub Range: UInt32,
    pub Code: UInt32,
    pub Low: UInt32,
    pub Stream: C2RustUnnamed_2,
    pub Indx2Units: [Byte; 40],
    pub Units2Indx: [Byte; 128],
    pub FreeList: [CPpmd_Void_Ref; 38],
    pub Stamps: [UInt32; 38],
    pub NS2BSIndx: [Byte; 256],
    pub NS2Indx: [Byte; 260],
    pub ExpEscape: [Byte; 16],
    pub DummySee: CPpmd_See,
    pub See: [[CPpmd_See; 32]; 24],
    pub BinSumm: [[UInt16; 64]; 25],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
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
2021-04-13 : Igor Pavlov : Public domain
This code is based on PPMd var.I (2002): Dmitry Shkarin : Public domain */
static mut PPMD8_kExpEscape: [Byte; 16] = [
    25_i32 as Byte,
    14_i32 as Byte,
    9_i32 as Byte,
    7_i32 as Byte,
    5_i32 as Byte,
    5_i32 as Byte,
    4_i32 as Byte,
    4_i32 as Byte,
    4_i32 as Byte,
    3_i32 as Byte,
    3_i32 as Byte,
    3_i32 as Byte,
    2_i32 as Byte,
    2_i32 as Byte,
    2_i32 as Byte,
    2_i32 as Byte,
];
static mut kInitBinEsc: [UInt16; 8] = [
    0x3cdd_i32 as UInt16,
    0x1f3f_i32 as UInt16,
    0x59bf_i32 as UInt16,
    0x48f3_i32 as UInt16,
    0x64a1_i32 as UInt16,
    0x5abc_i32 as UInt16,
    0x6632_i32 as UInt16,
    0x6051_i32 as UInt16,
];
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Construct(mut p: *mut CPpmd8) {
    let mut i: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut m: libc::c_uint = 0;
    (*p).Base = std::ptr::null_mut::<Byte>();
    i = 0_i32 as libc::c_uint;
    k = 0_i32 as libc::c_uint;
    while i
        < (4_i32
            + 4_i32
            + 4_i32
            + (128_i32 + 3_i32 - 1_i32 * 4_i32 - 2_i32 * 4_i32 - 3_i32 * 4_i32) / 4_i32)
            as libc::c_uint
    {
        let mut step: libc::c_uint = if i >= 12_i32 as libc::c_uint {
            4_i32 as libc::c_uint
        } else {
            (i >> 2_i32).wrapping_add(1_i32 as libc::c_uint)
        };
        loop {
            let fresh0 = k;
            k = k.wrapping_add(1);
            (*p).Units2Indx[fresh0 as usize] = i as Byte;
            step = step.wrapping_sub(1);
            if step == 0 {
                break;
            }
        }
        (*p).Indx2Units[i as usize] = k as Byte;
        i = i.wrapping_add(1)
    }
    (*p).NS2BSIndx[0_i32 as usize] = (0_i32 << 1_i32) as Byte;
    (*p).NS2BSIndx[1_i32 as usize] = (1_i32 << 1_i32) as Byte;
    memset(
        (*p).NS2BSIndx.as_mut_ptr().offset(2_i32 as isize) as *mut libc::c_void,
        2_i32 << 1_i32,
        9_i32 as libc::c_ulong,
    );
    memset(
        (*p).NS2BSIndx.as_mut_ptr().offset(11_i32 as isize) as *mut libc::c_void,
        3_i32 << 1_i32,
        (256_i32 - 11_i32) as libc::c_ulong,
    );
    i = 0_i32 as libc::c_uint;
    while i < 5_i32 as libc::c_uint {
        (*p).NS2Indx[i as usize] = i as Byte;
        i = i.wrapping_add(1)
    }
    m = i;
    k = 1_i32 as libc::c_uint;
    while i < 260_i32 as libc::c_uint {
        (*p).NS2Indx[i as usize] = m as Byte;
        k = k.wrapping_sub(1);
        if k == 0_i32 as libc::c_uint {
            m = m.wrapping_add(1);
            k = m.wrapping_sub(4_i32 as libc::c_uint)
        }
        i = i.wrapping_add(1)
    }
    memcpy(
        (*p).ExpEscape.as_mut_ptr() as *mut libc::c_void,
        PPMD8_kExpEscape.as_ptr() as *const libc::c_void,
        16_i32 as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Free(mut p: *mut CPpmd8, alloc: ISzAllocPtr) {
    (*alloc).Free.expect("non-null function pointer")(alloc, (*p).Base as *mut libc::c_void);
    (*p).Size = 0_i32 as UInt32;
    (*p).Base = std::ptr::null_mut::<Byte>();
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Alloc(
    mut p: *mut CPpmd8,
    size: UInt32,
    alloc: ISzAllocPtr,
) -> BoolInt {
    if (*p).Base.is_null() || (*p).Size != size {
        Ppmd8_Free(p, alloc);
        (*p).AlignOffset = (4_i32 as libc::c_uint).wrapping_sub(size) & 3_i32 as libc::c_uint;
        (*p).Base = (*alloc).Alloc.expect("non-null function pointer")(
            alloc,
            (*p).AlignOffset.wrapping_add(size) as size_t,
        ) as *mut Byte;
        if (*p).Base.is_null() {
            return 0_i32;
        }
        (*p).Size = size
    }
    1_i32
}
unsafe extern "C" fn InsertNode(
    mut p: *mut CPpmd8,
    node: *mut libc::c_void,
    indx: libc::c_uint,
) {
    (*(node as *mut CPpmd8_Node)).Stamp = 0xffffffff_u32;
    (*(node as *mut CPpmd8_Node)).Next = (*p).FreeList[indx as usize];
    (*(node as *mut CPpmd8_Node)).NU = (*p).Indx2Units[indx as usize] as libc::c_uint;
    (*p).FreeList[indx as usize] =
        (node as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
    (*p).Stamps[indx as usize] = (*p).Stamps[indx as usize].wrapping_add(1);
}
unsafe extern "C" fn RemoveNode(mut p: *mut CPpmd8, indx: libc::c_uint) -> *mut libc::c_void {
    let node: *mut CPpmd8_Node = (*p).Base.offset((*p).FreeList[indx as usize] as isize)
        as *mut libc::c_void as *mut CPpmd8_Node;
    (*p).FreeList[indx as usize] = (*node).Next;
    (*p).Stamps[indx as usize] = (*p).Stamps[indx as usize].wrapping_sub(1);
    node as *mut libc::c_void
}
unsafe extern "C" fn SplitBlock(
    p: *mut CPpmd8,
    mut ptr: *mut libc::c_void,
    oldIndx: libc::c_uint,
    newIndx: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    let nu: libc::c_uint = ((*p).Indx2Units[oldIndx as usize] as libc::c_uint)
        .wrapping_sub((*p).Indx2Units[newIndx as usize] as libc::c_uint);
    ptr = (ptr as *mut Byte).offset(
        ((*p).Indx2Units[newIndx as usize] as libc::c_uint).wrapping_mul(12_i32 as libc::c_uint)
            as isize,
    ) as *mut libc::c_void;
    i = (*p).Units2Indx[(nu as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
        as libc::c_uint;
    if (*p).Indx2Units[i as usize] as libc::c_uint != nu {
        i = i.wrapping_sub(1);
        let k: libc::c_uint = (*p).Indx2Units[i as usize] as libc::c_uint;
        InsertNode(
            p,
            (ptr as *mut Byte).offset(k.wrapping_mul(12_i32 as libc::c_uint) as isize)
                as *mut libc::c_void,
            nu.wrapping_sub(k).wrapping_sub(1_i32 as libc::c_uint),
        );
    }
    InsertNode(p, ptr, i);
}
unsafe extern "C" fn GlueFreeBlocks(mut p: *mut CPpmd8) {
    /*
    we use first UInt32 field of 12-bytes UNITs as record type stamp
      CPpmd_State    { Byte Symbol; Byte Freq; : Freq != 0xFF
      CPpmd8_Context { Byte NumStats; Byte Flags; UInt16 SummFreq;  : Flags != 0xFF ???
      CPpmd8_Node    { UInt32 Stamp            : Stamp == 0xFFFFFFFF for free record
                                               : Stamp == 0 for guard
      Last 12-bytes UNIT in array is always contains 12-bytes order-0 CPpmd8_Context record
    */
    let mut n: CPpmd8_Node_Ref = 0;
    (*p).GlueCount = (1_i32 << 13_i32) as UInt32;
    memset(
        (*p).Stamps.as_mut_ptr() as *mut libc::c_void,
        0_i32,
        ::std::mem::size_of::<[UInt32; 38]>() as libc::c_ulong,
    );
    /* we set guard NODE at LoUnit */
    if (*p).LoUnit != (*p).HiUnit {
        (*((*p).LoUnit as *mut libc::c_void as *mut CPpmd8_Node)).Stamp = 0_i32 as UInt32
    }
    /* Glue free blocks */
    let mut prev: *mut CPpmd8_Node_Ref = &mut n;
    let mut i: libc::c_uint = 0;
    i = 0_i32 as libc::c_uint;
    while i
        < (4_i32
            + 4_i32
            + 4_i32
            + (128_i32 + 3_i32 - 1_i32 * 4_i32 - 2_i32 * 4_i32 - 3_i32 * 4_i32) / 4_i32)
            as libc::c_uint
    {
        let mut next: CPpmd8_Node_Ref = (*p).FreeList[i as usize];
        (*p).FreeList[i as usize] = 0_i32 as CPpmd_Void_Ref;
        while next != 0_i32 as libc::c_uint {
            let mut node: *mut CPpmd8_Node =
                (*p).Base.offset(next as isize) as *mut libc::c_void as *mut CPpmd8_Node;
            let mut nu: UInt32 = (*node).NU;
            *prev = next;
            next = (*node).Next;
            if nu != 0_i32 as libc::c_uint {
                let mut node2: *mut CPpmd8_Node = std::ptr::null_mut::<CPpmd8_Node>();
                prev = &mut (*node).Next;
                loop {
                    node2 = node.offset(nu as isize);
                    if (*node2).Stamp != 0xffffffff_u32 {
                        break;
                    }
                    nu = (nu as libc::c_uint).wrapping_add((*node2).NU) as UInt32 as UInt32;
                    (*node2).NU = 0_i32 as UInt32;
                    (*node).NU = nu
                }
            }
        }
        i = i.wrapping_add(1)
    }
    *prev = 0_i32 as CPpmd8_Node_Ref;
    /* Fill lists of free blocks */
    while n != 0_i32 as libc::c_uint {
        let mut node_0: *mut CPpmd8_Node =
            (*p).Base.offset(n as isize) as *mut libc::c_void as *mut CPpmd8_Node;
        let mut nu_0: UInt32 = (*node_0).NU;
        let mut i_0: libc::c_uint = 0;
        n = (*node_0).Next;
        if nu_0 == 0_i32 as libc::c_uint {
            continue;
        }
        while nu_0 > 128_i32 as libc::c_uint {
            InsertNode(
                p,
                node_0 as *mut libc::c_void,
                (4_i32
                    + 4_i32
                    + 4_i32
                    + (128_i32 + 3_i32 - 1_i32 * 4_i32 - 2_i32 * 4_i32 - 3_i32 * 4_i32) / 4_i32
                    - 1_i32) as libc::c_uint,
            );
            nu_0 = (nu_0 as libc::c_uint).wrapping_sub(128_i32 as libc::c_uint) as UInt32 as UInt32;
            node_0 = node_0.offset(128_i32 as isize)
        }
        i_0 = (*p).Units2Indx[(nu_0 as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
            as libc::c_uint;
        if (*p).Indx2Units[i_0 as usize] as libc::c_uint != nu_0 {
            i_0 = i_0.wrapping_sub(1);
            let k: libc::c_uint = (*p).Indx2Units[i_0 as usize] as libc::c_uint;
            InsertNode(
                p,
                node_0.offset(k as isize) as *mut libc::c_void,
                nu_0.wrapping_sub(k).wrapping_sub(1_i32 as libc::c_uint),
            );
        }
        InsertNode(p, node_0 as *mut libc::c_void, i_0);
    }
}
#[inline(never)]
unsafe extern "C" fn AllocUnitsRare(
    mut p: *mut CPpmd8,
    indx: libc::c_uint,
) -> *mut libc::c_void {
    let mut i: libc::c_uint = 0;
    if (*p).GlueCount == 0_i32 as libc::c_uint {
        GlueFreeBlocks(p);
        if (*p).FreeList[indx as usize] != 0_i32 as libc::c_uint {
            return RemoveNode(p, indx);
        }
    }
    i = indx;
    loop {
        i = i.wrapping_add(1);
        if i == (4_i32
            + 4_i32
            + 4_i32
            + (128_i32 + 3_i32 - 1_i32 * 4_i32 - 2_i32 * 4_i32 - 3_i32 * 4_i32) / 4_i32)
            as libc::c_uint
        {
            let numBytes: UInt32 = ((*p).Indx2Units[indx as usize] as libc::c_uint)
                .wrapping_mul(12_i32 as libc::c_uint);
            let us: *mut Byte = (*p).UnitsStart;
            (*p).GlueCount = (*p).GlueCount.wrapping_sub(1);
            return if us.offset_from((*p).Text) as libc::c_long as UInt32 > numBytes {
                (*p).UnitsStart = us.offset(-(numBytes as isize));
                (*p).UnitsStart
            } else {
                std::ptr::null_mut::<Byte>()
            } as *mut libc::c_void;
        }
        if (*p).FreeList[i as usize] != 0_i32 as libc::c_uint {
            break;
        }
    }
    let block: *mut libc::c_void = RemoveNode(p, i);
    SplitBlock(p, block, i, indx);
    block
}
unsafe extern "C" fn AllocUnits(mut p: *mut CPpmd8, indx: libc::c_uint) -> *mut libc::c_void {
    if (*p).FreeList[indx as usize] != 0_i32 as libc::c_uint {
        return RemoveNode(p, indx);
    }
    let numBytes: UInt32 =
        ((*p).Indx2Units[indx as usize] as libc::c_uint).wrapping_mul(12_i32 as libc::c_uint);
    let lo: *mut Byte = (*p).LoUnit;
    if (*p).HiUnit.offset_from(lo) as libc::c_long as UInt32 >= numBytes {
        (*p).LoUnit = lo.offset(numBytes as isize);
        return lo as *mut libc::c_void;
    }
    AllocUnitsRare(p, indx)
}
unsafe extern "C" fn ShrinkUnits(
    p: *mut CPpmd8,
    oldPtr: *mut libc::c_void,
    oldNU: libc::c_uint,
    newNU: libc::c_uint,
) -> *mut libc::c_void {
    let i0: libc::c_uint = (*p).Units2Indx
        [(oldNU as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
        as libc::c_uint;
    let i1: libc::c_uint = (*p).Units2Indx
        [(newNU as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
        as libc::c_uint;
    if i0 == i1 {
        return oldPtr;
    }
    if (*p).FreeList[i1 as usize] != 0_i32 as libc::c_uint {
        let ptr: *mut libc::c_void = RemoveNode(p, i1);
        let mut d: *mut UInt32 = ptr as *mut UInt32;
        let mut z: *const UInt32 = oldPtr as *const UInt32;
        let mut n: UInt32 = newNU;
        loop {
            *d.offset(0_i32 as isize) = *z.offset(0_i32 as isize);
            *d.offset(1_i32 as isize) = *z.offset(1_i32 as isize);
            *d.offset(2_i32 as isize) = *z.offset(2_i32 as isize);
            z = z.offset(3_i32 as isize);
            d = d.offset(3_i32 as isize);
            n = n.wrapping_sub(1);
            if n == 0 {
                break;
            }
        }
        InsertNode(p, oldPtr, i0);
        return ptr;
    }
    SplitBlock(p, oldPtr, i0, i1);
    oldPtr
}
unsafe extern "C" fn FreeUnits(
    p: *mut CPpmd8,
    ptr: *mut libc::c_void,
    nu: libc::c_uint,
) {
    InsertNode(
        p,
        ptr,
        (*p).Units2Indx[(nu as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
            as libc::c_uint,
    );
}
unsafe extern "C" fn SpecialFreeUnit(mut p: *mut CPpmd8, ptr: *mut libc::c_void) {
    if ptr as *mut Byte != (*p).UnitsStart {
        InsertNode(p, ptr, 0_i32 as libc::c_uint);
    } else {
        (*p).UnitsStart = (*p).UnitsStart.offset(12_i32 as isize)
    };
}
/*
static void *MoveUnitsUp(CPpmd8 *p, void *oldPtr, unsigned nu)
{
  unsigned indx = U2I(nu);
  void *ptr;
  if ((Byte *)oldPtr > p->UnitsStart + (1 << 14) || REF(oldPtr) > p->FreeList[indx])
    return oldPtr;
  ptr = RemoveNode(p, indx);
  MyMem12Cpy(ptr, oldPtr, nu);
  if ((Byte *)oldPtr != p->UnitsStart)
    InsertNode(p, oldPtr, indx);
  else
    p->UnitsStart += U2B(I2U(indx));
  return ptr;
}
*/
unsafe extern "C" fn ExpandTextArea(mut p: *mut CPpmd8) {
    let mut count: [UInt32; 38] = [0; 38]; /* AllocContext(p); */
    let mut i: libc::c_uint = 0; /* AllocUnits(p, PPMD_NUM_INDEXES - 1); */
    memset(
        count.as_mut_ptr() as *mut libc::c_void,
        0_i32,
        ::std::mem::size_of::<[UInt32; 38]>() as libc::c_ulong,
    ); /* unused */
    if (*p).LoUnit != (*p).HiUnit {
        (*((*p).LoUnit as *mut libc::c_void as *mut CPpmd8_Node)).Stamp = 0_i32 as UInt32
    }
    let mut node: *mut CPpmd8_Node = (*p).UnitsStart as *mut libc::c_void as *mut CPpmd8_Node;
    while (*node).Stamp == 0xffffffff_u32 {
        let nu: UInt32 = (*node).NU;
        (*node).Stamp = 0_i32 as UInt32;
        count[(*p).Units2Indx[(nu as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
            as usize] = count[(*p).Units2Indx
            [(nu as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
            as usize]
            .wrapping_add(1);
        node = node.offset(nu as isize)
    }
    (*p).UnitsStart = node as *mut Byte;
    i = 0_i32 as libc::c_uint;
    while i
        < (4_i32
            + 4_i32
            + 4_i32
            + (128_i32 + 3_i32 - 1_i32 * 4_i32 - 2_i32 * 4_i32 - 3_i32 * 4_i32) / 4_i32)
            as libc::c_uint
    {
        let mut cnt: UInt32 = count[i as usize];
        if cnt != 0_i32 as libc::c_uint {
            let mut prev: *mut CPpmd8_Node_Ref = &mut *(*p).FreeList.as_mut_ptr().offset(i as isize)
                as *mut CPpmd_Void_Ref
                as *mut CPpmd8_Node_Ref;
            let mut n: CPpmd8_Node_Ref = *prev;
            (*p).Stamps[i as usize] =
                ((*p).Stamps[i as usize] as libc::c_uint).wrapping_sub(cnt) as UInt32 as UInt32;
            loop {
                let node_0: *mut CPpmd8_Node =
                    (*p).Base.offset(n as isize) as *mut libc::c_void as *mut CPpmd8_Node;
                n = (*node_0).Next;
                if (*node_0).Stamp != 0_i32 as libc::c_uint {
                    prev = &mut (*node_0).Next
                } else {
                    *prev = n;
                    cnt = cnt.wrapping_sub(1);
                    if cnt == 0_i32 as libc::c_uint {
                        break;
                    }
                }
            }
        }
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn SetSuccessor(mut p: *mut CPpmd_State, v: CPpmd_Void_Ref) {
    (*p).Successor_0 = v as UInt16;
    (*p).Successor_1 = (v >> 16_i32) as UInt16;
}
#[inline(never)]
unsafe extern "C" fn RestartModel(mut p: *mut CPpmd8) {
    let mut i: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut m: libc::c_uint = 0;
    memset(
        (*p).FreeList.as_mut_ptr() as *mut libc::c_void,
        0_i32,
        ::std::mem::size_of::<[CPpmd_Void_Ref; 38]>() as libc::c_ulong,
    );
    memset(
        (*p).Stamps.as_mut_ptr() as *mut libc::c_void,
        0_i32,
        ::std::mem::size_of::<[UInt32; 38]>() as libc::c_ulong,
    );
    (*p).Text = (*p)
        .Base
        .offset((*p).AlignOffset as isize)
        .offset(0_i32 as isize);
    (*p).HiUnit = (*p).Text.offset((*p).Size as isize);
    (*p).UnitsStart = (*p).HiUnit.offset(
        -((*p)
            .Size
            .wrapping_div(8_i32 as libc::c_uint)
            .wrapping_div(12_i32 as libc::c_uint)
            .wrapping_mul(7_i32 as libc::c_uint)
            .wrapping_mul(12_i32 as libc::c_uint) as isize),
    );
    (*p).LoUnit = (*p).UnitsStart;
    (*p).GlueCount = 0_i32 as UInt32;
    (*p).OrderFall = (*p).MaxOrder;
    (*p).InitRL = -((if (*p).MaxOrder < 12_i32 as libc::c_uint {
        (*p).MaxOrder
    } else {
        12_i32 as libc::c_uint
    }) as Int32)
        - 1_i32;
    (*p).RunLength = (*p).InitRL;
    (*p).PrevSuccess = 0_i32 as libc::c_uint;
    (*p).HiUnit = (*p).HiUnit.offset(-(12_i32 as isize));
    let mut mc: *mut CPpmd8_Context = (*p).HiUnit as *mut libc::c_void as CTX_PTR;
    let mut s: *mut CPpmd_State = (*p).LoUnit as *mut CPpmd_State;
    (*p).LoUnit = (*p)
        .LoUnit
        .offset(((256_i32 / 2_i32) as UInt32).wrapping_mul(12_i32 as libc::c_uint) as isize);
    (*p).MinContext = mc;
    (*p).MaxContext = (*p).MinContext;
    (*p).FoundState = s;
    (*mc).Flags = 0_i32 as Byte;
    (*mc).NumStats = (256_i32 - 1_i32) as Byte;
    (*mc).Union2.SummFreq = (256_i32 + 1_i32) as UInt16;
    (*mc).Union4.Stats = (s as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
    (*mc).Suffix = 0_i32 as CPpmd8_Context_Ref;
    i = 0_i32 as libc::c_uint;
    while i < 256_i32 as libc::c_uint {
        (*s).Symbol = i as Byte;
        (*s).Freq = 1_i32 as Byte;
        SetSuccessor(s, 0_i32 as CPpmd_Void_Ref);
        i = i.wrapping_add(1);
        s = s.offset(1)
    }
    m = 0_i32 as libc::c_uint;
    i = m;
    while m < 25_i32 as libc::c_uint {
        while (*p).NS2Indx[i as usize] as libc::c_uint == m {
            i = i.wrapping_add(1)
        }
        k = 0_i32 as libc::c_uint;
        while k < 8_i32 as libc::c_uint {
            let mut r: libc::c_uint = 0;
            let dest: *mut UInt16 = (*p).BinSumm[m as usize].as_mut_ptr().offset(k as isize);
            let val: UInt16 = ((1_i32 << (7_i32 + 7_i32)) as libc::c_uint).wrapping_sub(
                (kInitBinEsc[k as usize] as libc::c_uint)
                    .wrapping_div(i.wrapping_add(1_i32 as libc::c_uint)),
            ) as UInt16;
            r = 0_i32 as libc::c_uint;
            while r < 64_i32 as libc::c_uint {
                *dest.offset(r as isize) = val;
                r = r.wrapping_add(8_i32 as libc::c_uint)
            }
            k = k.wrapping_add(1)
        }
        m = m.wrapping_add(1)
    }
    m = 0_i32 as libc::c_uint;
    i = m;
    while m < 24_i32 as libc::c_uint {
        let mut summ: libc::c_uint = 0;
        let mut s_0: *mut CPpmd_See = std::ptr::null_mut::<CPpmd_See>();
        while (*p).NS2Indx[(i as size_t).wrapping_add(3_i32 as libc::c_ulong) as usize]
            as libc::c_uint
            == m.wrapping_add(3_i32 as libc::c_uint)
        {
            i = i.wrapping_add(1)
        }
        s_0 = (*p).See[m as usize].as_mut_ptr();
        summ = (2_i32 as libc::c_uint)
            .wrapping_mul(i)
            .wrapping_add(5_i32 as libc::c_uint)
            << (7_i32 - 4_i32);
        k = 0_i32 as libc::c_uint;
        while k < 32_i32 as libc::c_uint {
            (*s_0).Summ = summ as UInt16;
            (*s_0).Shift = (7_i32 - 4_i32) as Byte;
            (*s_0).Count = 7_i32 as Byte;
            k = k.wrapping_add(1);
            s_0 = s_0.offset(1)
        }
        m = m.wrapping_add(1)
    }
    (*p).DummySee.Summ = 0_i32 as UInt16;
    (*p).DummySee.Shift = 7_i32 as Byte;
    (*p).DummySee.Count = 64_i32 as Byte;
    /* unused */
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Init(
    mut p: *mut CPpmd8,
    maxOrder: libc::c_uint,
    restoreMethod: libc::c_uint,
) {
    (*p).MaxOrder = maxOrder;
    (*p).RestoreMethod = restoreMethod;
    RestartModel(p);
}
// #define PPMD8_HiBitsFlag_3(sym) (0x08 * ((sym) >= 0x40))
// #define PPMD8_HiBitsFlag_4(sym) (0x10 * ((sym) >= 0x40))
/*
Refresh() is called when we remove some symbols (successors) in context.
It increases Escape_Freq for sum of all removed symbols.
*/
unsafe extern "C" fn Refresh(
    p: *mut CPpmd8,
    mut ctx: CTX_PTR,
    oldNU: libc::c_uint,
    mut scale: libc::c_uint,
) {
    let mut i: libc::c_uint = (*ctx).NumStats as libc::c_uint;
    let mut escFreq: libc::c_uint = 0;
    let mut sumFreq: libc::c_uint = 0;
    let mut flags: libc::c_uint = 0;
    let mut s: *mut CPpmd_State = ShrinkUnits(
        p,
        (*p).Base.offset((*ctx).Union4.Stats as isize) as *mut libc::c_void as *mut CPpmd_State
            as *mut libc::c_void,
        oldNU,
        i.wrapping_add(2_i32 as libc::c_uint) >> 1_i32,
    ) as *mut CPpmd_State;
    (*ctx).Union4.Stats = (s as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
    // #ifdef PPMD8_FREEZE_SUPPORT
    /*
      (ctx->Union2.SummFreq >= ((UInt32)1 << 15)) can be in FREEZE mode for some files.
      It's not good for range coder. So new versions of support fix:
         -   original PPMdI code rev.1
         +   original PPMdI code rev.2
         -   7-Zip default ((PPMD8_FREEZE_SUPPORT is not defined)
         +   7-Zip (p->RestoreMethod >= PPMD8_RESTORE_METHOD_FREEZE)
      if we       use that fixed line, we can lose compatibility with some files created before fix
      if we don't use that fixed line, the program can work incorrectly in FREEZE mode in rare case.
    */
    // if (p->RestoreMethod >= PPMD8_RESTORE_METHOD_FREEZE)
    scale |= ((*ctx).Union2.SummFreq as libc::c_uint >= (1_i32 as UInt32) << 15_i32) as libc::c_int
        as libc::c_uint;
    // #endif
    flags = ((*s).Symbol as libc::c_uint).wrapping_add(0xc0_i32 as libc::c_uint);
    let mut freq: libc::c_uint = (*s).Freq as libc::c_uint;
    escFreq = ((*ctx).Union2.SummFreq as libc::c_uint).wrapping_sub(freq);
    freq = freq.wrapping_add(scale) >> scale;
    sumFreq = freq;
    (*s).Freq = freq as Byte;
    loop {
        s = s.offset(1);
        let mut freq_0: libc::c_uint = (*s).Freq as libc::c_uint;
        escFreq = escFreq.wrapping_sub(freq_0);
        freq_0 = freq_0.wrapping_add(scale) >> scale;
        sumFreq = sumFreq.wrapping_add(freq_0);
        (*s).Freq = freq_0 as Byte;
        flags |= ((*s).Symbol as libc::c_uint).wrapping_add(0xc0_i32 as libc::c_uint);
        i = i.wrapping_sub(1);
        if i == 0 {
            break;
        }
    }
    (*ctx).Union2.SummFreq = sumFreq.wrapping_add(escFreq.wrapping_add(scale) >> scale) as UInt16;
    (*ctx).Flags = ((*ctx).Flags as libc::c_uint
        & ((1_i32 << 4_i32) as libc::c_uint)
            .wrapping_add(((1_i32 << 2_i32) as libc::c_uint).wrapping_mul(scale)))
    .wrapping_add(flags >> (8_i32 - 3_i32) & (1_i32 << 3_i32) as libc::c_uint)
        as Byte;
}
unsafe extern "C" fn SwapStates(t1: *mut CPpmd_State, t2: *mut CPpmd_State) {
    std::mem::swap(&mut (*t1), &mut (*t2));
}
/*
CutOff() reduces contexts:
  It conversts Successors at MaxOrder to another Contexts to NULL-Successors
  It removes RAW-Successors and NULL-Successors that are not Order-0
      and it removes contexts when it has no Successors.
  if the (Union4.Stats) is close to (UnitsStart), it moves it up.
*/
unsafe extern "C" fn CutOff(
    mut p: *mut CPpmd8,
    mut ctx: CTX_PTR,
    order: libc::c_uint,
) -> CPpmd_Void_Ref {
    let mut ns: libc::c_int = (*ctx).NumStats as libc::c_int;
    let mut nu: libc::c_uint = 0;
    let mut stats: *mut CPpmd_State = std::ptr::null_mut::<CPpmd_State>();
    if ns == 0_i32 {
        let s: *mut CPpmd_State =
            &mut (*ctx).Union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State;
        let mut successor: CPpmd_Void_Ref =
            (*s).Successor_0 as libc::c_uint | ((*s).Successor_1 as UInt32) << 16_i32;
        if (*p).Base.offset(successor as isize) as *mut libc::c_void as *mut Byte >= (*p).UnitsStart
        {
            if order < (*p).MaxOrder {
                successor = CutOff(
                    p,
                    (*p).Base.offset(successor as isize) as *mut libc::c_void
                        as *mut CPpmd8_Context,
                    order.wrapping_add(1_i32 as libc::c_uint),
                )
            } else {
                successor = 0_i32 as CPpmd_Void_Ref
            }
            SetSuccessor(s, successor);
            if successor != 0 || order <= 9_i32 as libc::c_uint {
                /* O_BOUND */
                return (ctx as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
            }
        }
        SpecialFreeUnit(p, ctx as *mut libc::c_void);
        return 0_i32 as CPpmd_Void_Ref;
    }
    nu = (ns as libc::c_uint).wrapping_add(2_i32 as libc::c_uint) >> 1_i32;
    // ctx->Union4.Stats = STATS_REF(MoveUnitsUp(p, STATS(ctx), nu));
    let indx: libc::c_uint = (*p).Units2Indx
        [(nu as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
        as libc::c_uint;
    stats = (*p).Base.offset((*ctx).Union4.Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
    if (stats as *mut Byte).offset_from((*p).UnitsStart) as libc::c_long as UInt32
        <= (1_i32 << 14_i32) as libc::c_uint
        && (*ctx).Union4.Stats <= (*p).FreeList[indx as usize]
    {
        let ptr: *mut libc::c_void = RemoveNode(p, indx);
        (*ctx).Union4.Stats = (ptr as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
        let mut d: *mut UInt32 = ptr as *mut UInt32;
        let mut z: *const UInt32 = stats as *const libc::c_void as *const UInt32;
        let mut n: UInt32 = nu;
        loop {
            *d.offset(0_i32 as isize) = *z.offset(0_i32 as isize);
            *d.offset(1_i32 as isize) = *z.offset(1_i32 as isize);
            *d.offset(2_i32 as isize) = *z.offset(2_i32 as isize);
            z = z.offset(3_i32 as isize);
            d = d.offset(3_i32 as isize);
            n = n.wrapping_sub(1);
            if n == 0 {
                break;
            }
        }
        if stats as *mut Byte != (*p).UnitsStart {
            InsertNode(p, stats as *mut libc::c_void, indx);
        } else {
            (*p).UnitsStart = (*p).UnitsStart.offset(
                ((*p).Indx2Units[indx as usize] as libc::c_uint)
                    .wrapping_mul(12_i32 as libc::c_uint) as isize,
            )
        }
        stats = ptr as *mut CPpmd_State
    }
    let mut s_0: *mut CPpmd_State = stats.offset(ns as libc::c_uint as isize);
    loop {
        let successor_0: CPpmd_Void_Ref =
            (*s_0).Successor_0 as libc::c_uint | ((*s_0).Successor_1 as UInt32) << 16_i32;
        if ((*p).Base.offset(successor_0 as isize) as *mut libc::c_void as *mut Byte)
            < (*p).UnitsStart
        {
            let fresh1 = ns;
            ns -= 1;
            let s2: *mut CPpmd_State = stats.offset(fresh1 as libc::c_uint as isize);
            if order != 0 {
                if s_0 != s2 {
                    *s_0 = *s2
                }
            } else {
                SwapStates(s_0, s2);
                SetSuccessor(s2, 0_i32 as CPpmd_Void_Ref);
            }
        } else if order < (*p).MaxOrder {
            SetSuccessor(
                s_0,
                CutOff(
                    p,
                    (*p).Base.offset(successor_0 as isize) as *mut libc::c_void
                        as *mut CPpmd8_Context,
                    order.wrapping_add(1_i32 as libc::c_uint),
                ),
            );
        } else {
            SetSuccessor(s_0, 0_i32 as CPpmd_Void_Ref);
        }
        s_0 = s_0.offset(-1);
        if s_0 < stats {
            break;
        }
    }
    if ns != (*ctx).NumStats as libc::c_int && order != 0 {
        if ns < 0_i32 {
            FreeUnits(p, stats as *mut libc::c_void, nu);
            SpecialFreeUnit(p, ctx as *mut libc::c_void);
            return 0_i32 as CPpmd_Void_Ref;
        }
        (*ctx).NumStats = ns as Byte;
        if ns == 0_i32 {
            let sym: Byte = (*stats).Symbol;
            (*ctx).Flags = (((*ctx).Flags as libc::c_int & 1_i32 << 4_i32) as libc::c_uint)
                .wrapping_add(
                    (sym as libc::c_uint).wrapping_add(0xc0_i32 as libc::c_uint) >> (8_i32 - 3_i32)
                        & (1_i32 << 3_i32) as libc::c_uint,
                ) as Byte;
            // *ONE_STATE(ctx) = *stats;
            (*ctx).Union2.State2.Symbol = sym;
            (*ctx).Union2.State2.Freq = (((*stats).Freq as libc::c_uint)
                .wrapping_add(11_i32 as libc::c_uint)
                >> 3_i32) as Byte;
            (*ctx).Union4.State4.Successor_0 = (*stats).Successor_0;
            (*ctx).Union4.State4.Successor_1 = (*stats).Successor_1;
            FreeUnits(p, stats as *mut libc::c_void, nu);
        } else {
            Refresh(
                p,
                ctx,
                nu,
                ((*ctx).Union2.SummFreq as libc::c_uint
                    > (16_i32 as libc::c_uint).wrapping_mul(ns as libc::c_uint))
                    as libc::c_int as libc::c_uint,
            );
        }
    }
    (ctx as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32
}
unsafe extern "C" fn GetUsedMemory(p: *const CPpmd8) -> UInt32 {
    let mut v: UInt32 = 0_i32 as UInt32;
    let mut i: libc::c_uint = 0;
    i = 0_i32 as libc::c_uint;
    while i
        < (4_i32
            + 4_i32
            + 4_i32
            + (128_i32 + 3_i32 - 1_i32 * 4_i32 - 2_i32 * 4_i32 - 3_i32 * 4_i32) / 4_i32)
            as libc::c_uint
    {
        v = (v as libc::c_uint).wrapping_add(
            (*p).Stamps[i as usize].wrapping_mul((*p).Indx2Units[i as usize] as libc::c_uint),
        ) as UInt32 as UInt32;
        i = i.wrapping_add(1)
    }
    (*p).Size
        .wrapping_sub((*p).HiUnit.offset_from((*p).LoUnit) as libc::c_long as UInt32)
        .wrapping_sub((*p).UnitsStart.offset_from((*p).Text) as libc::c_long as UInt32)
        .wrapping_sub(v.wrapping_mul(12_i32 as libc::c_uint))
}
unsafe extern "C" fn RestoreModel(mut p: *mut CPpmd8, ctxError: CTX_PTR) {
    let mut c: CTX_PTR = std::ptr::null_mut::<CPpmd8_Context>();
    let mut s: *mut CPpmd_State = std::ptr::null_mut::<CPpmd_State>();
    (*p).Text = (*p)
        .Base
        .offset((*p).AlignOffset as isize)
        .offset(0_i32 as isize);
    // we go here in cases of error of allocation for context (c1)
    // Order(MinContext) < Order(ctxError) <= Order(MaxContext)
    // We remove last symbol from each of contexts [p->MaxContext ... ctxError) contexts
    // So we rollback all created (symbols) before error.
    c = (*p).MaxContext;
    while c != ctxError {
        (*c).NumStats = (*c).NumStats.wrapping_sub(1);
        if (*c).NumStats as libc::c_int == 0_i32 {
            s = (*p).Base.offset((*c).Union4.Stats as isize) as *mut libc::c_void
                as *mut CPpmd_State;
            (*c).Flags = (((*c).Flags as libc::c_int & 1_i32 << 4_i32) as libc::c_uint)
                .wrapping_add(
                    ((*s).Symbol as libc::c_uint).wrapping_add(0xc0_i32 as libc::c_uint)
                        >> (8_i32 - 3_i32)
                        & (1_i32 << 3_i32) as libc::c_uint,
                ) as Byte;
            // *ONE_STATE(c) = *s;
            (*c).Union2.State2.Symbol = (*s).Symbol;
            (*c).Union2.State2.Freq =
                (((*s).Freq as libc::c_uint).wrapping_add(11_i32 as libc::c_uint) >> 3_i32) as Byte;
            (*c).Union4.State4.Successor_0 = (*s).Successor_0;
            (*c).Union4.State4.Successor_1 = (*s).Successor_1;
            SpecialFreeUnit(p, s as *mut libc::c_void);
        } else {
            /* Refresh() can increase Escape_Freq on value of Freq of last symbol, that was added before error.
            so the largest possible increase for Escape_Freq is (8) from value before ModelUpoadet() */
            Refresh(
                p,
                c,
                ((*c).NumStats as libc::c_uint).wrapping_add(3_i32 as libc::c_uint) >> 1_i32,
                0_i32 as libc::c_uint,
            );
        }
        c = (*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context
    }
    // increase Escape Freq for context [ctxError ... p->MinContext)
    while c != (*p).MinContext {
        if (*c).NumStats as libc::c_int == 0_i32 {
            // ONE_STATE(c)
            (*c).Union2.State2.Freq = (((*c).Union2.State2.Freq as libc::c_uint)
                .wrapping_add(1_i32 as libc::c_uint)
                >> 1_i32) as Byte
        } else {
            (*c).Union2.SummFreq = ((*c).Union2.SummFreq as libc::c_int + 4_i32) as UInt16; /* fixed over Shkarin's code. Maybe it could work without + 1 too. */
            if (*c).Union2.SummFreq as libc::c_int > 128_i32 + 4_i32 * (*c).NumStats as libc::c_int
            {
                Refresh(
                    p,
                    c,
                    ((*c).NumStats as libc::c_uint).wrapping_add(2_i32 as libc::c_uint) >> 1_i32,
                    1_i32 as libc::c_uint,
                );
            }
        }
        c = (*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context
    }
    if (*p).RestoreMethod == PPMD8_RESTORE_METHOD_RESTART as libc::c_int as libc::c_uint
        || GetUsedMemory(p) < (*p).Size >> 1_i32
    {
        RestartModel(p);
    } else {
        while (*(*p).MaxContext).Suffix != 0 {
            (*p).MaxContext = (*p).Base.offset((*(*p).MaxContext).Suffix as isize)
                as *mut libc::c_void as *mut CPpmd8_Context
        }
        loop {
            CutOff(p, (*p).MaxContext, 0_i32 as libc::c_uint);
            ExpandTextArea(p);
            if GetUsedMemory(p) <= (3_i32 as libc::c_uint).wrapping_mul((*p).Size >> 2_i32) {
                break;
            }
        }
        (*p).GlueCount = 0_i32 as UInt32;
        (*p).OrderFall = (*p).MaxOrder
    }
    (*p).MinContext = (*p).MaxContext;
}
#[inline(never)]
unsafe extern "C" fn CreateSuccessors(
    mut p: *mut CPpmd8,
    skip: BoolInt,
    mut s1: *mut CPpmd_State,
    mut c: CTX_PTR,
) -> CTX_PTR {
    let mut upBranch: CPpmd_Byte_Ref = (*(*p).FoundState).Successor_0 as libc::c_uint
        | ((*(*p).FoundState).Successor_1 as UInt32) << 16_i32;
    let mut newSym: Byte = 0;
    let mut newFreq: Byte = 0;
    let mut flags: Byte = 0;
    let mut numPs: libc::c_uint = 0_i32 as libc::c_uint;
    let mut ps: [*mut CPpmd_State; 17] = [std::ptr::null_mut::<CPpmd_State>(); 17];
    if skip == 0 {
        let fresh2 = numPs;
        numPs = numPs.wrapping_add(1);
        ps[fresh2 as usize] = (*p).FoundState
    }
    while (*c).Suffix != 0 {
        let mut successor: CPpmd_Void_Ref = 0;
        let mut s: *mut CPpmd_State = std::ptr::null_mut::<CPpmd_State>();
        c = (*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context;
        if !s1.is_null() {
            s = s1;
            s1 = std::ptr::null_mut::<CPpmd_State>()
        } else if (*c).NumStats as libc::c_int != 0_i32 {
            let sym: Byte = (*(*p).FoundState).Symbol;
            s = (*p).Base.offset((*c).Union4.Stats as isize) as *mut libc::c_void
                as *mut CPpmd_State;
            while (*s).Symbol as libc::c_int != sym as libc::c_int {
                s = s.offset(1)
            }
            if ((*s).Freq as libc::c_int) < 124_i32 - 9_i32 {
                (*s).Freq = (*s).Freq.wrapping_add(1);
                (*c).Union2.SummFreq = (*c).Union2.SummFreq.wrapping_add(1)
            }
        } else {
            s = &mut (*c).Union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State;
            (*s).Freq = ((*s).Freq as libc::c_int
                + (((*((*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void
                    as *mut CPpmd8_Context))
                    .NumStats
                    == 0) as libc::c_int
                    & (((*s).Freq as libc::c_int) < 24_i32) as libc::c_int))
                as Byte
        }
        successor = (*s).Successor_0 as libc::c_uint | ((*s).Successor_1 as UInt32) << 16_i32;
        if successor != upBranch {
            c = (*p).Base.offset(successor as isize) as *mut libc::c_void as *mut CPpmd8_Context;
            if numPs == 0_i32 as libc::c_uint {
                return c;
            }
            break;
        } else {
            let fresh3 = numPs;
            numPs = numPs.wrapping_add(1);
            ps[fresh3 as usize] = s
        }
    }
    newSym = *((*p).Base.offset(upBranch as isize) as *mut libc::c_void as *const Byte);
    upBranch = upBranch.wrapping_add(1);
    flags = (((*(*p).FoundState).Symbol as libc::c_uint).wrapping_add(0xc0_i32 as libc::c_uint)
        >> (8_i32 - 4_i32)
        & (1_i32 << 4_i32) as libc::c_uint)
        .wrapping_add(
            (newSym as libc::c_uint).wrapping_add(0xc0_i32 as libc::c_uint) >> (8_i32 - 3_i32)
                & (1_i32 << 3_i32) as libc::c_uint,
        ) as Byte;
    if (*c).NumStats as libc::c_int == 0_i32 {
        newFreq = (*c).Union2.State2.Freq
    } else {
        let mut cf: UInt32 = 0;
        let mut s0: UInt32 = 0;
        let mut s_0: *mut CPpmd_State = std::ptr::null_mut::<CPpmd_State>();
        s_0 = (*p).Base.offset((*c).Union4.Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
        while (*s_0).Symbol as libc::c_int != newSym as libc::c_int {
            s_0 = s_0.offset(1)
        }
        cf = ((*s_0).Freq as UInt32).wrapping_sub(1_i32 as libc::c_uint);
        s0 = ((*c).Union2.SummFreq as UInt32)
            .wrapping_sub((*c).NumStats as libc::c_uint)
            .wrapping_sub(cf);
        /*


          max(newFreq)= (s->Freq - 1), when (s0 == 1)


        */
        newFreq = (1_i32 as libc::c_uint).wrapping_add(
            if (2_i32 as libc::c_uint).wrapping_mul(cf) <= s0 {
                ((5_i32 as libc::c_uint).wrapping_mul(cf) > s0) as libc::c_int as libc::c_uint
            } else {
                cf.wrapping_add((2_i32 as libc::c_uint).wrapping_mul(s0))
                    .wrapping_sub(3_i32 as libc::c_uint)
                    .wrapping_div(s0)
            },
        ) as Byte
    }
    loop {
        let mut c1: CTX_PTR = std::ptr::null_mut::<CPpmd8_Context>();
        /* = AllocContext(p); */
        if (*p).HiUnit != (*p).LoUnit {
            (*p).HiUnit = (*p).HiUnit.offset(-(12_i32 as isize));
            c1 = (*p).HiUnit as *mut libc::c_void as CTX_PTR
        } else if (*p).FreeList[0_i32 as usize] != 0_i32 as libc::c_uint {
            c1 = RemoveNode(p, 0_i32 as libc::c_uint) as CTX_PTR
        } else {
            c1 = AllocUnitsRare(p, 0_i32 as libc::c_uint) as CTX_PTR;
            if c1.is_null() {
                return 0 as CTX_PTR;
            }
        }
        (*c1).Flags = flags;
        (*c1).NumStats = 0_i32 as Byte;
        (*c1).Union2.State2.Symbol = newSym;
        (*c1).Union2.State2.Freq = newFreq;
        SetSuccessor(
            &mut (*c1).Union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State,
            upBranch,
        );
        (*c1).Suffix = (c as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
        numPs = numPs.wrapping_sub(1);
        SetSuccessor(
            ps[numPs as usize],
            (c1 as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32,
        );
        c = c1;
        if numPs == 0_i32 as libc::c_uint {
            break;
        }
    }
    c
}
unsafe extern "C" fn ReduceOrder(
    mut p: *mut CPpmd8,
    mut s1: *mut CPpmd_State,
    mut c: CTX_PTR,
) -> CTX_PTR {
    let mut s: *mut CPpmd_State = std::ptr::null_mut::<CPpmd_State>();
    let c1: CTX_PTR = c;
    let upBranch: CPpmd_Void_Ref = (*p).Text.offset_from((*p).Base) as libc::c_long as UInt32;
    SetSuccessor((*p).FoundState, upBranch);
    (*p).OrderFall = (*p).OrderFall.wrapping_add(1);
    loop {
        if !s1.is_null() {
            c = (*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context;
            s = s1;
            s1 = std::ptr::null_mut::<CPpmd_State>()
        } else {
            if (*c).Suffix == 0 {
                return c;
            }
            c = (*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context;
            if (*c).NumStats != 0 {
                s = (*p).Base.offset((*c).Union4.Stats as isize) as *mut libc::c_void
                    as *mut CPpmd_State;
                if (*s).Symbol as libc::c_int != (*(*p).FoundState).Symbol as libc::c_int {
                    loop {
                        s = s.offset(1);
                        if (*s).Symbol as libc::c_int == (*(*p).FoundState).Symbol as libc::c_int {
                            break;
                        }
                    }
                }
                if ((*s).Freq as libc::c_int) < 124_i32 - 9_i32 {
                    (*s).Freq = ((*s).Freq as libc::c_int + 2_i32) as Byte;
                    (*c).Union2.SummFreq = ((*c).Union2.SummFreq as libc::c_int + 2_i32) as UInt16
                }
            } else {
                s = &mut (*c).Union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State;
                (*s).Freq = ((*s).Freq as libc::c_int
                    + (((*s).Freq as libc::c_int) < 32_i32) as libc::c_int)
                    as Byte
            }
        }
        if (*s).Successor_0 as libc::c_uint | ((*s).Successor_1 as UInt32) << 16_i32 != 0 {
            break;
        }
        SetSuccessor(s, upBranch);
        (*p).OrderFall = (*p).OrderFall.wrapping_add(1)
    }
    if (*s).Successor_0 as libc::c_uint | ((*s).Successor_1 as UInt32) << 16_i32 <= upBranch {
        let mut successor: CTX_PTR = std::ptr::null_mut::<CPpmd8_Context>();
        let s2: *mut CPpmd_State = (*p).FoundState;
        (*p).FoundState = s;
        successor = CreateSuccessors(p, 0_i32, std::ptr::null_mut::<CPpmd_State>(), c);
        if successor.is_null() {
            SetSuccessor(s, 0_i32 as CPpmd_Void_Ref);
        } else {
            SetSuccessor(
                s,
                (successor as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32,
            );
        }
        (*p).FoundState = s2
    }
    let successor_0: CPpmd_Void_Ref =
        (*s).Successor_0 as libc::c_uint | ((*s).Successor_1 as UInt32) << 16_i32;
    if (*p).OrderFall == 1_i32 as libc::c_uint && c1 == (*p).MaxContext {
        SetSuccessor((*p).FoundState, successor_0);
        (*p).Text = (*p).Text.offset(-1)
    }
    if successor_0 == 0_i32 as libc::c_uint {
        return 0 as CTX_PTR;
    }
    (*p).Base.offset(successor_0 as isize) as *mut libc::c_void as *mut CPpmd8_Context
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn Ppmd8_UpdateModel(mut p: *mut CPpmd8) {
    let mut maxSuccessor: CPpmd_Void_Ref = 0;
    let mut minSuccessor: CPpmd_Void_Ref = (*(*p).FoundState).Successor_0 as libc::c_uint
        | ((*(*p).FoundState).Successor_1 as UInt32) << 16_i32;
    let mut c: CTX_PTR = std::ptr::null_mut::<CPpmd8_Context>();
    let mut s0: libc::c_uint = 0;
    let mut ns: libc::c_uint = 0;
    let fFreq: libc::c_uint = (*(*p).FoundState).Freq as libc::c_uint;
    let mut flag: Byte = 0;
    let fSymbol: Byte = (*(*p).FoundState).Symbol;
    let mut s: *mut CPpmd_State = std::ptr::null_mut::<CPpmd_State>();
    if ((*(*p).FoundState).Freq as libc::c_int) < 124_i32 / 4_i32
        && (*(*p).MinContext).Suffix != 0_i32 as libc::c_uint
    {
        /* Update Freqs in Suffix Context */
        c = (*p).Base.offset((*(*p).MinContext).Suffix as isize) as *mut libc::c_void
            as *mut CPpmd8_Context; /* check it */
        if (*c).NumStats as libc::c_int == 0_i32 {
            s = &mut (*c).Union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State;
            if ((*s).Freq as libc::c_int) < 32_i32 {
                (*s).Freq = (*s).Freq.wrapping_add(1)
            }
        } else {
            let sym: Byte = (*(*p).FoundState).Symbol;
            s = (*p).Base.offset((*c).Union4.Stats as isize) as *mut libc::c_void
                as *mut CPpmd_State;
            if (*s).Symbol as libc::c_int != sym as libc::c_int {
                loop {
                    s = s.offset(1);
                    if (*s).Symbol as libc::c_int == sym as libc::c_int {
                        break;
                    }
                }
                if (*s.offset(0_i32 as isize)).Freq as libc::c_int
                    >= (*s.offset(-1_i32 as isize)).Freq as libc::c_int
                {
                    SwapStates(
                        &mut *s.offset(0_i32 as isize),
                        &mut *s.offset(-1_i32 as isize),
                    );
                    s = s.offset(-1)
                }
            }
            if ((*s).Freq as libc::c_int) < 124_i32 - 9_i32 {
                (*s).Freq = ((*s).Freq as libc::c_int + 2_i32) as Byte;
                (*c).Union2.SummFreq = ((*c).Union2.SummFreq as libc::c_int + 2_i32) as UInt16
            }
        }
    }
    c = (*p).MaxContext;
    if (*p).OrderFall == 0_i32 as libc::c_uint && minSuccessor != 0 {
        let cs: CTX_PTR = CreateSuccessors(p, 1_i32, s, (*p).MinContext);
        if cs.is_null() {
            SetSuccessor((*p).FoundState, 0_i32 as CPpmd_Void_Ref);
            RestoreModel(p, c);
            return;
        }
        SetSuccessor(
            (*p).FoundState,
            (cs as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32,
        );
        (*p).MaxContext = cs;
        (*p).MinContext = (*p).MaxContext;
        return;
    }
    let mut text: *mut Byte = (*p).Text;
    let fresh4 = text;
    text = text.offset(1);
    *fresh4 = (*(*p).FoundState).Symbol;
    (*p).Text = text;
    if text >= (*p).UnitsStart {
        RestoreModel(p, c);
        return;
    }
    maxSuccessor = text.offset_from((*p).Base) as libc::c_long as UInt32;
    if minSuccessor == 0 {
        let cs_0: CTX_PTR = ReduceOrder(p, s, (*p).MinContext);
        if cs_0.is_null() {
            RestoreModel(p, c);
            return;
        }
        minSuccessor = (cs_0 as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32
    } else if ((*p).Base.offset(minSuccessor as isize) as *mut libc::c_void as *mut Byte)
        < (*p).UnitsStart
    {
        let cs_1: CTX_PTR = CreateSuccessors(p, 0_i32, s, (*p).MinContext);
        if cs_1.is_null() {
            RestoreModel(p, c);
            return;
        }
        minSuccessor = (cs_1 as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32
    }
    (*p).OrderFall = (*p).OrderFall.wrapping_sub(1);
    if (*p).OrderFall == 0_i32 as libc::c_uint {
        maxSuccessor = minSuccessor;
        (*p).Text = (*p)
            .Text
            .offset(-(((*p).MaxContext != (*p).MinContext) as libc::c_int as isize))
    }
    flag = ((fSymbol as libc::c_uint).wrapping_add(0xc0_i32 as libc::c_uint) >> (8_i32 - 3_i32)
        & (1_i32 << 3_i32) as libc::c_uint) as Byte;
    ns = (*(*p).MinContext).NumStats as libc::c_uint;
    s0 = ((*(*p).MinContext).Union2.SummFreq as libc::c_uint)
        .wrapping_sub(ns)
        .wrapping_sub(fFreq);
    while c != (*p).MinContext {
        let mut ns1: libc::c_uint = 0;
        let mut sum: UInt32 = 0;
        ns1 = (*c).NumStats as libc::c_uint;
        if ns1 != 0_i32 as libc::c_uint {
            if ns1 & 1_i32 as libc::c_uint != 0_i32 as libc::c_uint {
                /* Expand for one UNIT */
                let oldNU: libc::c_uint = ns1.wrapping_add(1_i32 as libc::c_uint) >> 1_i32;
                let i: libc::c_uint = (*p).Units2Indx
                    [(oldNU as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
                    as libc::c_uint;
                if i != (*p).Units2Indx[(oldNU as size_t)
                    .wrapping_add(1_i32 as libc::c_ulong)
                    .wrapping_sub(1_i32 as libc::c_ulong)
                    as usize] as libc::c_uint
                {
                    let ptr: *mut libc::c_void =
                        AllocUnits(p, i.wrapping_add(1_i32 as libc::c_uint));
                    let mut oldPtr: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
                    if ptr.is_null() {
                        RestoreModel(p, c);
                        return;
                    }
                    oldPtr = (*p).Base.offset((*c).Union4.Stats as isize) as *mut libc::c_void
                        as *mut CPpmd_State as *mut libc::c_void;
                    let mut d: *mut UInt32 = ptr as *mut UInt32;
                    let mut z: *const UInt32 = oldPtr as *const UInt32;
                    let mut n: UInt32 = oldNU;
                    loop {
                        *d.offset(0_i32 as isize) = *z.offset(0_i32 as isize);
                        *d.offset(1_i32 as isize) = *z.offset(1_i32 as isize);
                        *d.offset(2_i32 as isize) = *z.offset(2_i32 as isize);
                        z = z.offset(3_i32 as isize);
                        d = d.offset(3_i32 as isize);
                        n = n.wrapping_sub(1);
                        if n == 0 {
                            break;
                        }
                    }
                    InsertNode(p, oldPtr, i);
                    (*c).Union4.Stats =
                        (ptr as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32
                }
            }
            sum = (*c).Union2.SummFreq as UInt32;
            /* original PPMdH uses 16-bit variable for (sum) here.
            But (sum < ???). Do we need to truncate (sum) to 16-bit */
            // sum = (UInt16)sum;
            sum = (sum as libc::c_uint).wrapping_add(
                ((3_i32 as libc::c_uint)
                    .wrapping_mul(ns1)
                    .wrapping_add(1_i32 as libc::c_uint)
                    < ns) as libc::c_int as libc::c_uint,
            ) as UInt32 as UInt32
        } else {
            let mut s_0: *mut CPpmd_State =
                AllocUnits(p, 0_i32 as libc::c_uint) as *mut CPpmd_State;
            if s_0.is_null() {
                RestoreModel(p, c);
                return;
            }
            let mut freq: libc::c_uint = (*c).Union2.State2.Freq as libc::c_uint;
            /* max increase of Escape_Freq is 1 here.
            an average increase is 1/3 per symbol */
            // Ppmd8 (> 2)
            (*s_0).Symbol = (*c).Union2.State2.Symbol;
            (*s_0).Successor_0 = (*c).Union4.State4.Successor_0;
            (*s_0).Successor_1 = (*c).Union4.State4.Successor_1;
            (*c).Union4.Stats = (s_0 as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
            if freq < (124_i32 / 4_i32 - 1_i32) as libc::c_uint {
                freq <<= 1_i32
            } else {
                freq = (124_i32 - 4_i32) as libc::c_uint
            }
            (*s_0).Freq = freq as Byte;
            sum = freq
                .wrapping_add((*p).InitEsc)
                .wrapping_add((ns > 2_i32 as libc::c_uint) as libc::c_int as libc::c_uint)
        }
        let mut s_1: *mut CPpmd_State = ((*p).Base.offset((*c).Union4.Stats as isize)
            as *mut libc::c_void as *mut CPpmd_State)
            .offset(ns1 as isize)
            .offset(1_i32 as isize);
        let mut cf: UInt32 = (2_i32 as libc::c_uint)
            .wrapping_mul(sum.wrapping_add(6_i32 as libc::c_uint))
            .wrapping_mul(fFreq);
        let sf: UInt32 = s0.wrapping_add(sum);
        (*s_1).Symbol = fSymbol;
        (*c).NumStats = ns1.wrapping_add(1_i32 as libc::c_uint) as Byte;
        SetSuccessor(s_1, maxSuccessor);
        (*c).Flags = ((*c).Flags as libc::c_int | flag as libc::c_int) as Byte;
        if cf < (6_i32 as libc::c_uint).wrapping_mul(sf) {
            cf = (1_i32 as libc::c_uint)
                .wrapping_add((cf > sf) as libc::c_int as libc::c_uint)
                .wrapping_add(
                    (cf >= (4_i32 as libc::c_uint).wrapping_mul(sf)) as libc::c_int as libc::c_uint,
                );
            sum = (sum as libc::c_uint).wrapping_add(4_i32 as libc::c_uint) as UInt32 as UInt32
            // s = *ONE_STATE(c);
            // SetSuccessor(s, c->Union4.Stats);  // call it only for debug purposes to check the order of
            // (Successor_0 and Successor_1) in LE/BE.
            /* It can add (1, 2, 3) to Escape_Freq */
        } else {
            cf = (4_i32 as libc::c_uint)
                .wrapping_add(
                    (cf > (9_i32 as libc::c_uint).wrapping_mul(sf)) as libc::c_int as libc::c_uint,
                )
                .wrapping_add(
                    (cf > (12_i32 as libc::c_uint).wrapping_mul(sf)) as libc::c_int as libc::c_uint,
                )
                .wrapping_add(
                    (cf > (15_i32 as libc::c_uint).wrapping_mul(sf)) as libc::c_int as libc::c_uint,
                );
            sum = (sum as libc::c_uint).wrapping_add(cf) as UInt32 as UInt32
        }
        (*c).Union2.SummFreq = sum as UInt16;
        (*s_1).Freq = cf as Byte;
        c = (*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context
    }
    (*p).MinContext =
        (*p).Base.offset(minSuccessor as isize) as *mut libc::c_void as *mut CPpmd8_Context;
    (*p).MaxContext = (*p).MinContext;
}
#[inline(never)]
unsafe extern "C" fn Rescale(mut p: *mut CPpmd8) {
    let mut i: libc::c_uint = 0;
    let mut adder: libc::c_uint = 0;
    let mut sumFreq: libc::c_uint = 0;
    let mut escFreq: libc::c_uint = 0;
    let stats: *mut CPpmd_State = (*p).Base.offset((*(*p).MinContext).Union4.Stats as isize)
        as *mut libc::c_void as *mut CPpmd_State;
    let mut s: *mut CPpmd_State = (*p).FoundState;
    /* Sort the list by Freq */
    if s != stats {
        let tmp: CPpmd_State = *s;
        loop {
            *s.offset(0_i32 as isize) = *s.offset(-1_i32 as isize);
            s = s.offset(-1);
            if s == stats {
                break;
            }
        }
        *s = tmp
    }
    sumFreq = (*s).Freq as libc::c_uint;
    escFreq = ((*(*p).MinContext).Union2.SummFreq as libc::c_uint).wrapping_sub(sumFreq);
    adder = ((*p).OrderFall != 0_i32 as libc::c_uint) as libc::c_int as libc::c_uint;
    sumFreq = sumFreq
        .wrapping_add(4_i32 as libc::c_uint)
        .wrapping_add(adder)
        >> 1_i32;
    i = (*(*p).MinContext).NumStats as libc::c_uint;
    (*s).Freq = sumFreq as Byte;
    loop {
        s = s.offset(1);
        let mut freq: libc::c_uint = (*s).Freq as libc::c_uint;
        escFreq = escFreq.wrapping_sub(freq);
        freq = freq.wrapping_add(adder) >> 1_i32;
        sumFreq = sumFreq.wrapping_add(freq);
        (*s).Freq = freq as Byte;
        if freq > (*s.offset(-1_i32 as isize)).Freq as libc::c_uint {
            let tmp_0: CPpmd_State = *s;
            let mut s1: *mut CPpmd_State = s;
            loop {
                *s1.offset(0_i32 as isize) = *s1.offset(-1_i32 as isize);
                s1 = s1.offset(-1);
                if !(s1 != stats && freq > (*s1.offset(-1_i32 as isize)).Freq as libc::c_uint) {
                    break;
                }
            }
            *s1 = tmp_0
        }
        i = i.wrapping_sub(1);
        if i == 0 {
            break;
        }
    }
    if (*s).Freq as libc::c_int == 0_i32 {
        /* Remove all items with Freq == 0 */
        let mut mc: *mut CPpmd8_Context = std::ptr::null_mut::<CPpmd8_Context>();
        let mut numStats: libc::c_uint = 0;
        let mut numStatsNew: libc::c_uint = 0;
        let mut n0: libc::c_uint = 0;
        let mut n1: libc::c_uint = 0;
        i = 0_i32 as libc::c_uint;
        loop {
            i = i.wrapping_add(1);
            s = s.offset(-1);
            if (*s).Freq as libc::c_int != 0_i32 {
                break;
            }
        }
        escFreq = escFreq.wrapping_add(i);
        mc = (*p).MinContext;
        numStats = (*mc).NumStats as libc::c_uint;
        numStatsNew = numStats.wrapping_sub(i);
        (*mc).NumStats = numStatsNew as Byte;
        n0 = numStats.wrapping_add(2_i32 as libc::c_uint) >> 1_i32;
        if numStatsNew == 0_i32 as libc::c_uint {
            let mut freq_0: libc::c_uint = (2_i32 as libc::c_uint)
                .wrapping_mul((*stats).Freq as libc::c_uint)
                .wrapping_add(escFreq)
                .wrapping_sub(1_i32 as libc::c_uint)
                .wrapping_div(escFreq);
            if freq_0 > (124_i32 / 3_i32) as libc::c_uint {
                freq_0 = (124_i32 / 3_i32) as libc::c_uint
            }
            (*mc).Flags = (((*mc).Flags as libc::c_int & 1_i32 << 4_i32) as libc::c_uint)
                .wrapping_add(
                    ((*stats).Symbol as libc::c_uint).wrapping_add(0xc0_i32 as libc::c_uint)
                        >> (8_i32 - 3_i32)
                        & (1_i32 << 3_i32) as libc::c_uint,
                ) as Byte;
            s = &mut (*mc).Union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State;
            *s = *stats;
            (*s).Freq = freq_0 as Byte;
            (*p).FoundState = s;
            InsertNode(
                p,
                stats as *mut libc::c_void,
                (*p).Units2Indx[(n0 as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
                    as libc::c_uint,
            );
            return;
        }
        n1 = numStatsNew.wrapping_add(2_i32 as libc::c_uint) >> 1_i32;
        if n0 != n1 {
            (*mc).Union4.Stats = (ShrinkUnits(p, stats as *mut libc::c_void, n0, n1) as *mut Byte)
                .offset_from((*p).Base) as libc::c_long as UInt32
        }
    }
    let mut mc_0: *mut CPpmd8_Context = (*p).MinContext;
    (*mc_0).Union2.SummFreq =
        sumFreq.wrapping_add(escFreq).wrapping_sub(escFreq >> 1_i32) as UInt16;
    (*mc_0).Flags = ((*mc_0).Flags as libc::c_int | 1_i32 << 2_i32) as Byte;
    (*p).FoundState =
        (*p).Base.offset((*mc_0).Union4.Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_MakeEscFreq(
    p: *mut CPpmd8,
    numMasked1: libc::c_uint,
    escFreq: *mut UInt32,
) -> *mut CPpmd_See {
    let mut see: *mut CPpmd_See = std::ptr::null_mut::<CPpmd_See>();
    let mc: *const CPpmd8_Context = (*p).MinContext;
    let numStats: libc::c_uint = (*mc).NumStats as libc::c_uint;
    if numStats != 0xff_i32 as libc::c_uint {
        // (3 <= numStats + 2 <= 256)   (3 <= NS2Indx[3] and NS2Indx[256] === 26)
        see = (*p).See[((*p).NS2Indx
            [(numStats as size_t).wrapping_add(2_i32 as libc::c_ulong) as usize]
            as libc::c_uint as size_t)
            .wrapping_sub(3_i32 as libc::c_ulong) as usize]
            .as_mut_ptr()
            .offset(
                ((*mc).Union2.SummFreq as libc::c_uint
                    > (11_i32 as libc::c_uint)
                        .wrapping_mul(numStats.wrapping_add(1_i32 as libc::c_uint)))
                    as libc::c_int as isize,
            )
            .offset(
                (2_i32 as libc::c_uint).wrapping_mul(
                    ((2_i32 as libc::c_uint).wrapping_mul(numStats)
                        < ((*((*p).Base.offset((*mc).Suffix as isize) as *mut libc::c_void
                            as *mut CPpmd8_Context))
                            .NumStats as libc::c_uint)
                            .wrapping_add(numMasked1)) as libc::c_int
                        as libc::c_uint,
                ) as isize,
            )
            .offset((*mc).Flags as libc::c_int as isize);
        // if (see->Summ) field is larger than 16-bit, we need only low 16 bits of Summ
        let summ: libc::c_uint = (*see).Summ as libc::c_uint; // & 0xFFFF
        let r: libc::c_uint = summ >> (*see).Shift as libc::c_int; // Ppmd8 (>=)
        (*see).Summ = summ.wrapping_sub(r) as UInt16;
        *escFreq = r.wrapping_add((r == 0_i32 as libc::c_uint) as libc::c_int as libc::c_uint)
    } else {
        see = &mut (*p).DummySee;
        *escFreq = 1_i32 as UInt32
    }
    see
}
unsafe extern "C" fn NextContext(mut p: *mut CPpmd8) {
    let c: CTX_PTR = (*p).Base.offset(
        ((*(*p).FoundState).Successor_0 as libc::c_uint
            | ((*(*p).FoundState).Successor_1 as UInt32) << 16_i32) as isize,
    ) as *mut libc::c_void as *mut CPpmd8_Context;
    if (*p).OrderFall == 0_i32 as libc::c_uint && c as *const Byte >= (*p).UnitsStart {
        (*p).MinContext = c;
        (*p).MaxContext = (*p).MinContext
    } else {
        Ppmd8_UpdateModel(p);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Update1(mut p: *mut CPpmd8) {
    let mut s: *mut CPpmd_State = (*p).FoundState;
    let mut freq: libc::c_uint = (*s).Freq as libc::c_uint;
    freq = freq.wrapping_add(4_i32 as libc::c_uint);
    (*(*p).MinContext).Union2.SummFreq =
        ((*(*p).MinContext).Union2.SummFreq as libc::c_int + 4_i32) as UInt16;
    (*s).Freq = freq as Byte;
    if freq > (*s.offset(-1_i32 as isize)).Freq as libc::c_uint {
        SwapStates(s, &mut *s.offset(-1_i32 as isize));
        s = s.offset(-1);
        (*p).FoundState = s;
        if freq > 124_i32 as libc::c_uint {
            Rescale(p);
        }
    }
    NextContext(p);
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Update1_0(mut p: *mut CPpmd8) {
    let mut s: *mut CPpmd_State = (*p).FoundState;
    let mut mc: *mut CPpmd8_Context = (*p).MinContext;
    let mut freq: libc::c_uint = (*s).Freq as libc::c_uint;
    let summFreq: libc::c_uint = (*mc).Union2.SummFreq as libc::c_uint;
    (*p).PrevSuccess =
        ((2_i32 as libc::c_uint).wrapping_mul(freq) >= summFreq) as libc::c_int as libc::c_uint;
    (*p).RunLength += (*p).PrevSuccess as libc::c_int;
    (*mc).Union2.SummFreq = summFreq.wrapping_add(4_i32 as libc::c_uint) as UInt16;
    freq = freq.wrapping_add(4_i32 as libc::c_uint);
    (*s).Freq = freq as Byte;
    if freq > 124_i32 as libc::c_uint {
        Rescale(p);
    }
    NextContext(p);
}
/*
void Ppmd8_UpdateBin(CPpmd8 *p)
{
  unsigned freq = p->FoundState->Freq;
  p->FoundState->Freq = (Byte)(freq + (freq < 196)); // Ppmd8 (196)
  p->PrevSuccess = 1;
  p->RunLength++;
  NextContext(p);
}
*/
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Update2(mut p: *mut CPpmd8) {
    let mut s: *mut CPpmd_State = (*p).FoundState;
    let mut freq: libc::c_uint = (*s).Freq as libc::c_uint;
    freq = freq.wrapping_add(4_i32 as libc::c_uint);
    (*p).RunLength = (*p).InitRL;
    (*(*p).MinContext).Union2.SummFreq =
        ((*(*p).MinContext).Union2.SummFreq as libc::c_int + 4_i32) as UInt16;
    (*s).Freq = freq as Byte;
    if freq > 124_i32 as libc::c_uint {
        Rescale(p);
    }
    Ppmd8_UpdateModel(p);
}
/* H->I changes:
  NS2Indx
  GlueCount, and Glue method
  BinSum
  See / EscFreq
  CreateSuccessors updates more suffix contexts
  Ppmd8_UpdateModel consts.
  PrevSuccess Update

Flags:
  (1 << 2) - the Context was Rescaled
  (1 << 3) - there is symbol in Stats with (sym >= 0x40) in
  (1 << 4) - main symbol of context is (sym >= 0x40)
*/

/* ---------- Encode ---------- */
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Flush_RangeEnc(mut p: *mut CPpmd8) {
    let mut i: libc::c_uint = 0;
    i = 0_i32 as libc::c_uint;
    while i < 4_i32 as libc::c_uint {
        (*(*p).Stream.Out).Write.expect("non-null function pointer")(
            (*p).Stream.Out,
            ((*p).Low >> 24_i32) as Byte,
        );
        i = i.wrapping_add(1);
        (*p).Low <<= 8_i32
    }
}
// MY_NO_INLINE
unsafe extern "C" fn RangeEnc_Encode(
    mut p: *mut CPpmd8,
    start: UInt32,
    size: UInt32,
    total: UInt32,
) {
    (*p).Range = ((*p).Range as libc::c_uint).wrapping_div(total) as UInt32 as UInt32;
    (*p).Low =
        ((*p).Low as libc::c_uint).wrapping_add(start.wrapping_mul((*p).Range)) as UInt32 as UInt32;
    (*p).Range = ((*p).Range as libc::c_uint).wrapping_mul(size) as UInt32 as UInt32;
}
// MY_FORCE_INLINE
// static
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_EncodeSymbol(mut p: *mut CPpmd8, symbol: libc::c_int) {
    let mut charMask: [size_t; 32] = [0; 32];
    if (*(*p).MinContext).NumStats as libc::c_int != 0_i32 {
        let mut s: *mut CPpmd_State = (*p).Base.offset((*(*p).MinContext).Union4.Stats as isize)
            as *mut libc::c_void as *mut CPpmd_State;
        let mut sum: UInt32 = 0;
        let mut i: libc::c_uint = 0;
        let mut summFreq: UInt32 = (*(*p).MinContext).Union2.SummFreq as UInt32;
        if summFreq > (*p).Range {
            summFreq = (*p).Range
        }
        // RC_PRE(summFreq);
        if (*s).Symbol as libc::c_int == symbol {
            RangeEnc_Encode(p, 0_i32 as UInt32, (*s).Freq as UInt32, summFreq);
            while (*p).Low ^ (*p).Low.wrapping_add((*p).Range) < (1_i32 << 24_i32) as libc::c_uint
                || (*p).Range < (1_i32 << 15_i32) as libc::c_uint && {
                    (*p).Range = (0_i32 as libc::c_uint).wrapping_sub((*p).Low)
                        & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                    1_i32 != 0
                }
            {
                (*(*p).Stream.Out).Write.expect("non-null function pointer")(
                    (*p).Stream.Out,
                    ((*p).Low >> 24_i32) as Byte,
                );
                (*p).Range <<= 8_i32;
                (*p).Low <<= 8_i32
            }
            (*p).FoundState = s;
            Ppmd8_Update1_0(p);
            return;
        }
        (*p).PrevSuccess = 0_i32 as libc::c_uint;
        sum = (*s).Freq as UInt32;
        i = (*(*p).MinContext).NumStats as libc::c_uint;
        loop {
            s = s.offset(1);
            if (*s).Symbol as libc::c_int == symbol {
                RangeEnc_Encode(p, sum, (*s).Freq as UInt32, summFreq);
                while (*p).Low ^ (*p).Low.wrapping_add((*p).Range)
                    < (1_i32 << 24_i32) as libc::c_uint
                    || (*p).Range < (1_i32 << 15_i32) as libc::c_uint && {
                        (*p).Range = (0_i32 as libc::c_uint).wrapping_sub((*p).Low)
                            & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                        1_i32 != 0
                    }
                {
                    (*(*p).Stream.Out).Write.expect("non-null function pointer")(
                        (*p).Stream.Out,
                        ((*p).Low >> 24_i32) as Byte,
                    );
                    (*p).Range <<= 8_i32;
                    (*p).Low <<= 8_i32
                }
                (*p).FoundState = s;
                Ppmd8_Update1(p);
                return;
            }
            sum = (sum as libc::c_uint).wrapping_add((*s).Freq as libc::c_uint) as UInt32 as UInt32;
            i = i.wrapping_sub(1);
            if i == 0 {
                break;
            }
        }
        RangeEnc_Encode(p, sum, summFreq.wrapping_sub(sum), summFreq);
        let mut z: size_t = 0;
        z = 0_i32 as size_t;
        while z
            < (256_i32 as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<size_t>() as libc::c_ulong)
        {
            charMask[z.wrapping_add(0_i32 as libc::c_ulong) as usize] = !(0_i32 as size_t);
            charMask[z.wrapping_add(1_i32 as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(0_i32 as libc::c_ulong) as usize];
            charMask[z.wrapping_add(2_i32 as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(1_i32 as libc::c_ulong) as usize];
            charMask[z.wrapping_add(3_i32 as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(2_i32 as libc::c_ulong) as usize];
            charMask[z.wrapping_add(4_i32 as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(3_i32 as libc::c_ulong) as usize];
            charMask[z.wrapping_add(5_i32 as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(4_i32 as libc::c_ulong) as usize];
            charMask[z.wrapping_add(6_i32 as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(5_i32 as libc::c_ulong) as usize];
            charMask[z.wrapping_add(7_i32 as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(6_i32 as libc::c_ulong) as usize];
            z = (z as libc::c_ulong).wrapping_add(8_i32 as libc::c_ulong) as size_t as size_t
        }
        // MASK(s->Symbol) = 0;
        // i = p->MinContext->NumStats;
        // do { MASK((--s)->Symbol) = 0; } while (--i);
        let mut s2: *mut CPpmd_State = (*p).Base.offset((*(*p).MinContext).Union4.Stats as isize)
            as *mut libc::c_void as *mut CPpmd_State;
        *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset((*s).Symbol as isize) =
            0_i32 as libc::c_uchar;
        loop {
            let sym0: libc::c_uint = (*s2.offset(0_i32 as isize)).Symbol as libc::c_uint;
            let sym1: libc::c_uint = (*s2.offset(1_i32 as isize)).Symbol as libc::c_uint;
            s2 = s2.offset(2_i32 as isize);
            *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset(sym0 as isize) =
                0_i32 as libc::c_uchar;
            *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset(sym1 as isize) =
                0_i32 as libc::c_uchar;
            if s2 >= s {
                break;
            }
        }
    } else {
        let prob: *mut UInt16 = &mut *(*(*p).BinSumm.as_mut_ptr().offset(
            *(*p).NS2Indx.as_mut_ptr().offset(
                ((*(&mut (*(*p).MinContext).Union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State))
                    .Freq as size_t)
                    .wrapping_sub(1_i32 as libc::c_ulong) as isize,
            ) as isize,
        ))
        .as_mut_ptr()
        .offset(
            (*p).PrevSuccess
                .wrapping_add(((*p).RunLength >> 26_i32 & 0x20_i32) as libc::c_uint)
                .wrapping_add(
                    *(*p).NS2BSIndx.as_mut_ptr().offset(
                        (*((*p).Base.offset((*(*p).MinContext).Suffix as isize) as *mut libc::c_void
                            as *mut CPpmd8_Context))
                            .NumStats as isize,
                    ) as libc::c_uint,
                )
                .wrapping_add((*(*p).MinContext).Flags as libc::c_int as libc::c_uint)
                as isize,
        ) as *mut UInt16;
        let mut s_0: *mut CPpmd_State =
            &mut (*(*p).MinContext).Union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State;
        let mut pr: UInt32 = *prob as UInt32;
        let bound: UInt32 = ((*p).Range >> 14_i32).wrapping_mul(pr);
        pr = pr.wrapping_sub(pr.wrapping_add((1_i32 << (7_i32 - 2_i32)) as libc::c_uint) >> 7_i32);
        if (*s_0).Symbol as libc::c_int == symbol {
            *prob = pr.wrapping_add((1_i32 << 7_i32) as libc::c_uint) as UInt16;
            // RangeEnc_EncodeBit_0(p, bound);
            (*p).Range = bound;
            while (*p).Low ^ (*p).Low.wrapping_add((*p).Range) < (1_i32 << 24_i32) as libc::c_uint
                || (*p).Range < (1_i32 << 15_i32) as libc::c_uint && {
                    (*p).Range = (0_i32 as libc::c_uint).wrapping_sub((*p).Low)
                        & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                    1_i32 != 0
                }
            {
                (*(*p).Stream.Out).Write.expect("non-null function pointer")(
                    (*p).Stream.Out,
                    ((*p).Low >> 24_i32) as Byte,
                );
                (*p).Range <<= 8_i32;
                (*p).Low <<= 8_i32
            }
            // p->FoundState = s;
            // Ppmd8_UpdateBin(p);
            let freq: libc::c_uint = (*s_0).Freq as libc::c_uint; // Ppmd8 (196)
            let c: CTX_PTR = (*p).Base.offset(
                ((*s_0).Successor_0 as libc::c_uint | ((*s_0).Successor_1 as UInt32) << 16_i32)
                    as isize,
            ) as *mut libc::c_void as *mut CPpmd8_Context;
            (*p).FoundState = s_0;
            (*p).PrevSuccess = 1_i32 as libc::c_uint;
            (*p).RunLength += 1;
            (*s_0).Freq = freq
                .wrapping_add((freq < 196_i32 as libc::c_uint) as libc::c_int as libc::c_uint)
                as Byte;
            // NextContext(p);
            if (*p).OrderFall == 0_i32 as libc::c_uint && c as *const Byte >= (*p).UnitsStart {
                (*p).MinContext = c;
                (*p).MaxContext = (*p).MinContext
            } else {
                Ppmd8_UpdateModel(p);
            }
            return;
        }
        *prob = pr as UInt16;
        (*p).InitEsc = (*p).ExpEscape[(pr >> 10_i32) as usize] as libc::c_uint;
        // RangeEnc_EncodeBit_1(p, bound);
        (*p).Low = ((*p).Low as libc::c_uint).wrapping_add(bound) as UInt32 as UInt32; /* EndMarker (symbol = -1) */
        (*p).Range = ((*p).Range
            & !((1_i32 << (7_i32 + 7_i32)) as UInt32).wrapping_sub(1_i32 as libc::c_uint))
        .wrapping_sub(bound);
        let mut z_0: size_t = 0;
        z_0 = 0_i32 as size_t;
        while z_0
            < (256_i32 as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<size_t>() as libc::c_ulong)
        {
            charMask[z_0.wrapping_add(0_i32 as libc::c_ulong) as usize] = !(0_i32 as size_t);
            charMask[z_0.wrapping_add(1_i32 as libc::c_ulong) as usize] =
                charMask[z_0.wrapping_add(0_i32 as libc::c_ulong) as usize];
            charMask[z_0.wrapping_add(2_i32 as libc::c_ulong) as usize] =
                charMask[z_0.wrapping_add(1_i32 as libc::c_ulong) as usize];
            charMask[z_0.wrapping_add(3_i32 as libc::c_ulong) as usize] =
                charMask[z_0.wrapping_add(2_i32 as libc::c_ulong) as usize];
            charMask[z_0.wrapping_add(4_i32 as libc::c_ulong) as usize] =
                charMask[z_0.wrapping_add(3_i32 as libc::c_ulong) as usize];
            charMask[z_0.wrapping_add(5_i32 as libc::c_ulong) as usize] =
                charMask[z_0.wrapping_add(4_i32 as libc::c_ulong) as usize];
            charMask[z_0.wrapping_add(6_i32 as libc::c_ulong) as usize] =
                charMask[z_0.wrapping_add(5_i32 as libc::c_ulong) as usize];
            charMask[z_0.wrapping_add(7_i32 as libc::c_ulong) as usize] =
                charMask[z_0.wrapping_add(6_i32 as libc::c_ulong) as usize];
            z_0 = (z_0 as libc::c_ulong).wrapping_add(8_i32 as libc::c_ulong) as size_t as size_t
        }
        *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset((*s_0).Symbol as isize) =
            0_i32 as libc::c_uchar;
        (*p).PrevSuccess = 0_i32 as libc::c_uint
    }
    loop {
        let mut see: *mut CPpmd_See = std::ptr::null_mut::<CPpmd_See>();
        let mut s_1: *mut CPpmd_State = std::ptr::null_mut::<CPpmd_State>();
        let mut sum_0: UInt32 = 0;
        let mut escFreq: UInt32 = 0;
        let mut mc: *mut CPpmd8_Context = std::ptr::null_mut::<CPpmd8_Context>();
        let mut i_0: libc::c_uint = 0;
        let mut numMasked: libc::c_uint = 0;
        while (*p).Low ^ (*p).Low.wrapping_add((*p).Range) < (1_i32 << 24_i32) as libc::c_uint
            || (*p).Range < (1_i32 << 15_i32) as libc::c_uint && {
                (*p).Range = (0_i32 as libc::c_uint).wrapping_sub((*p).Low)
                    & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                1_i32 != 0
            }
        {
            (*(*p).Stream.Out).Write.expect("non-null function pointer")(
                (*p).Stream.Out,
                ((*p).Low >> 24_i32) as Byte,
            );
            (*p).Range <<= 8_i32;
            (*p).Low <<= 8_i32
        }
        mc = (*p).MinContext;
        numMasked = (*mc).NumStats as libc::c_uint;
        loop {
            (*p).OrderFall = (*p).OrderFall.wrapping_add(1);
            if (*mc).Suffix == 0 {
                return;
            }
            mc =
                (*p).Base.offset((*mc).Suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context;
            if (*mc).NumStats as libc::c_uint != numMasked {
                break;
            }
        }
        (*p).MinContext = mc;
        see = Ppmd8_MakeEscFreq(p, numMasked, &mut escFreq);
        s_1 = (*p).Base.offset((*(*p).MinContext).Union4.Stats as isize) as *mut libc::c_void
            as *mut CPpmd_State;
        sum_0 = 0_i32 as UInt32;
        i_0 = ((*(*p).MinContext).NumStats as libc::c_uint).wrapping_add(1_i32 as libc::c_uint);
        loop {
            let cur: libc::c_uint = (*s_1).Symbol as libc::c_uint;
            if cur as libc::c_int == symbol {
                let low: UInt32 = sum_0;
                let freq_0: UInt32 = (*s_1).Freq as UInt32;
                let mut num2: libc::c_uint = 0;
                if ((*see).Shift as libc::c_int) < 7_i32 && {
                    (*see).Count = (*see).Count.wrapping_sub(1);
                    ((*see).Count as libc::c_int) == 0_i32
                } {
                    (*see).Summ = (((*see).Summ as libc::c_int) << 1_i32) as UInt16;
                    let fresh0 = (*see).Shift;
                    (*see).Shift = (*see).Shift.wrapping_add(1);
                    (*see).Count = (3_i32 << fresh0 as libc::c_int) as Byte
                }
                (*p).FoundState = s_1;
                sum_0 = (sum_0 as libc::c_uint).wrapping_add(escFreq) as UInt32 as UInt32;
                num2 = i_0.wrapping_div(2_i32 as libc::c_uint);
                i_0 &= 1_i32 as libc::c_uint;
                sum_0 = (sum_0 as libc::c_uint)
                    .wrapping_add(freq_0 & (0_i32 as libc::c_uint).wrapping_sub(i_0))
                    as UInt32 as UInt32;
                if num2 != 0_i32 as libc::c_uint {
                    s_1 = s_1.offset(i_0 as isize);
                    loop {
                        let sym0_0: libc::c_uint =
                            (*s_1.offset(0_i32 as isize)).Symbol as libc::c_uint;
                        let sym1_0: libc::c_uint =
                            (*s_1.offset(1_i32 as isize)).Symbol as libc::c_uint;
                        s_1 = s_1.offset(2_i32 as isize);
                        sum_0 = (sum_0 as libc::c_uint).wrapping_add(
                            (*s_1.offset(-2_i32 as isize)).Freq as libc::c_uint
                                & *(charMask.as_mut_ptr() as *mut libc::c_uchar)
                                    .offset(sym0_0 as isize)
                                    as libc::c_uint,
                        ) as UInt32 as UInt32;
                        sum_0 = (sum_0 as libc::c_uint).wrapping_add(
                            (*s_1.offset(-1_i32 as isize)).Freq as libc::c_uint
                                & *(charMask.as_mut_ptr() as *mut libc::c_uchar)
                                    .offset(sym1_0 as isize)
                                    as libc::c_uint,
                        ) as UInt32 as UInt32;
                        num2 = num2.wrapping_sub(1);
                        if num2 == 0_i32 as libc::c_uint {
                            break;
                        }
                    }
                }
                if sum_0 > (*p).Range {
                    sum_0 = (*p).Range
                }
                RangeEnc_Encode(p, low, freq_0, sum_0);
                while (*p).Low ^ (*p).Low.wrapping_add((*p).Range)
                    < (1_i32 << 24_i32) as libc::c_uint
                    || (*p).Range < (1_i32 << 15_i32) as libc::c_uint && {
                        (*p).Range = (0_i32 as libc::c_uint).wrapping_sub((*p).Low)
                            & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                        1_i32 != 0
                    }
                {
                    (*(*p).Stream.Out).Write.expect("non-null function pointer")(
                        (*p).Stream.Out,
                        ((*p).Low >> 24_i32) as Byte,
                    );
                    (*p).Range <<= 8_i32;
                    (*p).Low <<= 8_i32
                }
                Ppmd8_Update2(p);
                return;
            }
            sum_0 = (sum_0 as libc::c_uint).wrapping_add(
                (*s_1).Freq as libc::c_uint
                    & *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset(cur as isize)
                        as libc::c_uint,
            ) as UInt32 as UInt32;
            s_1 = s_1.offset(1);
            i_0 = i_0.wrapping_sub(1);
            if i_0 == 0 {
                break;
            }
        }
        let mut total: UInt32 = sum_0.wrapping_add(escFreq);
        (*see).Summ = ((*see).Summ as libc::c_uint).wrapping_add(total) as UInt16;
        if total > (*p).Range {
            total = (*p).Range
        }
        RangeEnc_Encode(p, sum_0, total.wrapping_sub(sum_0), total);
        let mut s2_0: *mut CPpmd_State = (*p).Base.offset((*(*p).MinContext).Union4.Stats as isize)
            as *mut libc::c_void as *mut CPpmd_State;
        s_1 = s_1.offset(-1);
        *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset((*s_1).Symbol as isize) =
            0_i32 as libc::c_uchar;
        loop {
            let sym0_1: libc::c_uint = (*s2_0.offset(0_i32 as isize)).Symbol as libc::c_uint;
            let sym1_1: libc::c_uint = (*s2_0.offset(1_i32 as isize)).Symbol as libc::c_uint;
            s2_0 = s2_0.offset(2_i32 as isize);
            *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset(sym0_1 as isize) =
                0_i32 as libc::c_uchar;
            *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset(sym1_1 as isize) =
                0_i32 as libc::c_uchar;
            if s2_0 >= s_1 {
                break;
            }
        }
    }
}

/*
You must set (CPpmd8::Stream.In) before Ppmd8_RangeDec_Init()

Ppmd8_DecodeSymbol()
out:
  >= 0 : decoded byte
    -1 : PPMD8_SYM_END   : End of payload marker
    -2 : PPMD8_SYM_ERROR : Data error
*/
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_Init_RangeDec(mut p: *mut CPpmd8) -> BoolInt {
    let mut i: libc::c_uint = 0;
    (*p).Code = 0_i32 as UInt32;
    (*p).Range = 0xffffffff_u32;
    (*p).Low = 0_i32 as UInt32;
    i = 0_i32 as libc::c_uint;
    while i < 4_i32 as libc::c_uint {
        (*p).Code = (*p).Code << 8_i32
            | (*(*p).Stream.In).Read.expect("non-null function pointer")((*p).Stream.In)
                as libc::c_uint;
        i = i.wrapping_add(1)
    }
    ((*p).Code < 0xffffffff_u32) as libc::c_int
}
// MY_NO_INLINE
unsafe extern "C" fn RangeDec_Decode(mut p: *mut CPpmd8, mut start: UInt32, size: UInt32) {
    start = (start as libc::c_uint).wrapping_mul((*p).Range) as UInt32 as UInt32;
    (*p).Low = ((*p).Low as libc::c_uint).wrapping_add(start) as UInt32 as UInt32;
    (*p).Code = ((*p).Code as libc::c_uint).wrapping_sub(start) as UInt32 as UInt32;
    (*p).Range = ((*p).Range as libc::c_uint).wrapping_mul(size) as UInt32 as UInt32;
}
#[no_mangle]
pub unsafe extern "C" fn Ppmd8_DecodeSymbol(mut p: *mut CPpmd8) -> libc::c_int {
    let mut charMask: [size_t; 32] = [0; 32];
    if (*(*p).MinContext).NumStats as libc::c_int != 0_i32 {
        let mut s: *mut CPpmd_State = (*p).Base.offset((*(*p).MinContext).Union4.Stats as isize)
            as *mut libc::c_void as *mut CPpmd_State;
        let mut i: libc::c_uint = 0;
        let mut count: UInt32 = 0;
        let mut hiCnt: UInt32 = 0;
        let mut summFreq: UInt32 = (*(*p).MinContext).Union2.SummFreq as UInt32;
        if summFreq > (*p).Range {
            summFreq = (*p).Range
        }
        (*p).Range = ((*p).Range as libc::c_uint).wrapping_div(summFreq) as UInt32 as UInt32;
        count = (*p).Code.wrapping_div((*p).Range);
        hiCnt = count;
        count = (count as libc::c_uint).wrapping_sub((*s).Freq as libc::c_uint) as UInt32 as UInt32;
        if (count as Int32) < 0_i32 {
            let mut sym: Byte = 0;
            RangeDec_Decode(p, 0_i32 as UInt32, (*s).Freq as UInt32);
            while (*p).Low ^ (*p).Low.wrapping_add((*p).Range) < (1_i32 << 24_i32) as libc::c_uint
                || (*p).Range < (1_i32 << 15_i32) as libc::c_uint && {
                    (*p).Range = (0_i32 as libc::c_uint).wrapping_sub((*p).Low)
                        & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                    1_i32 != 0
                }
            {
                (*p).Code = (*p).Code << 8_i32
                    | (*(*p).Stream.In).Read.expect("non-null function pointer")((*p).Stream.In)
                        as libc::c_uint;
                (*p).Range <<= 8_i32;
                (*p).Low <<= 8_i32
            }
            (*p).FoundState = s;
            sym = (*s).Symbol;
            Ppmd8_Update1_0(p);
            return sym as libc::c_int;
        }
        (*p).PrevSuccess = 0_i32 as libc::c_uint;
        i = (*(*p).MinContext).NumStats as libc::c_uint;
        loop {
            s = s.offset(1);
            count =
                (count as libc::c_uint).wrapping_sub((*s).Freq as libc::c_uint) as UInt32 as UInt32;
            if (count as Int32) < 0_i32 {
                let mut sym_0: Byte = 0;
                RangeDec_Decode(
                    p,
                    hiCnt
                        .wrapping_sub(count)
                        .wrapping_sub((*s).Freq as libc::c_uint),
                    (*s).Freq as UInt32,
                );
                while (*p).Low ^ (*p).Low.wrapping_add((*p).Range)
                    < (1_i32 << 24_i32) as libc::c_uint
                    || (*p).Range < (1_i32 << 15_i32) as libc::c_uint && {
                        (*p).Range = (0_i32 as libc::c_uint).wrapping_sub((*p).Low)
                            & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                        1_i32 != 0
                    }
                {
                    (*p).Code = (*p).Code << 8_i32
                        | (*(*p).Stream.In).Read.expect("non-null function pointer")((*p).Stream.In)
                            as libc::c_uint;
                    (*p).Range <<= 8_i32;
                    (*p).Low <<= 8_i32
                }
                (*p).FoundState = s;
                sym_0 = (*s).Symbol;
                Ppmd8_Update1(p);
                return sym_0 as libc::c_int;
            }
            i = i.wrapping_sub(1);
            if i == 0 {
                break;
            }
        }
        if hiCnt >= summFreq {
            return -2_i32;
        }
        hiCnt = (hiCnt as libc::c_uint).wrapping_sub(count) as UInt32 as UInt32;
        RangeDec_Decode(p, hiCnt, summFreq.wrapping_sub(hiCnt));
        let mut z: size_t = 0;
        z = 0_i32 as size_t;
        while z
            < (256_i32 as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<size_t>() as libc::c_ulong)
        {
            charMask[z.wrapping_add(0_i32 as libc::c_ulong) as usize] = !(0_i32 as size_t);
            charMask[z.wrapping_add(1_i32 as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(0_i32 as libc::c_ulong) as usize];
            charMask[z.wrapping_add(2_i32 as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(1_i32 as libc::c_ulong) as usize];
            charMask[z.wrapping_add(3_i32 as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(2_i32 as libc::c_ulong) as usize];
            charMask[z.wrapping_add(4_i32 as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(3_i32 as libc::c_ulong) as usize];
            charMask[z.wrapping_add(5_i32 as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(4_i32 as libc::c_ulong) as usize];
            charMask[z.wrapping_add(6_i32 as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(5_i32 as libc::c_ulong) as usize];
            charMask[z.wrapping_add(7_i32 as libc::c_ulong) as usize] =
                charMask[z.wrapping_add(6_i32 as libc::c_ulong) as usize];
            z = (z as libc::c_ulong).wrapping_add(8_i32 as libc::c_ulong) as size_t as size_t
        }
        // i = p->MinContext->NumStats - 1;
        // do { MASK((--s)->Symbol) = 0; } while (--i);
        let mut s2: *mut CPpmd_State = (*p).Base.offset((*(*p).MinContext).Union4.Stats as isize)
            as *mut libc::c_void as *mut CPpmd_State;
        *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset((*s).Symbol as isize) =
            0_i32 as libc::c_uchar;
        loop {
            let sym0: libc::c_uint = (*s2.offset(0_i32 as isize)).Symbol as libc::c_uint;
            let sym1: libc::c_uint = (*s2.offset(1_i32 as isize)).Symbol as libc::c_uint;
            s2 = s2.offset(2_i32 as isize);
            *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset(sym0 as isize) =
                0_i32 as libc::c_uchar;
            *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset(sym1 as isize) =
                0_i32 as libc::c_uchar;
            if s2 >= s {
                break;
            }
        }
    } else {
        let mut s_0: *mut CPpmd_State =
            &mut (*(*p).MinContext).Union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State;
        let prob: *mut UInt16 = &mut *(*(*p).BinSumm.as_mut_ptr().offset(
            *(*p).NS2Indx.as_mut_ptr().offset(
                ((*(&mut (*(*p).MinContext).Union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State))
                    .Freq as size_t)
                    .wrapping_sub(1_i32 as libc::c_ulong) as isize,
            ) as isize,
        ))
        .as_mut_ptr()
        .offset(
            (*p).PrevSuccess
                .wrapping_add(((*p).RunLength >> 26_i32 & 0x20_i32) as libc::c_uint)
                .wrapping_add(
                    *(*p).NS2BSIndx.as_mut_ptr().offset(
                        (*((*p).Base.offset((*(*p).MinContext).Suffix as isize) as *mut libc::c_void
                            as *mut CPpmd8_Context))
                            .NumStats as isize,
                    ) as libc::c_uint,
                )
                .wrapping_add((*(*p).MinContext).Flags as libc::c_int as libc::c_uint)
                as isize,
        ) as *mut UInt16;
        let mut pr: UInt32 = *prob as UInt32;
        let size0: UInt32 = ((*p).Range >> 14_i32).wrapping_mul(pr);
        pr = pr.wrapping_sub(pr.wrapping_add((1_i32 << (7_i32 - 2_i32)) as libc::c_uint) >> 7_i32);
        if (*p).Code < size0 {
            let mut sym_1: Byte = 0;
            *prob = pr.wrapping_add((1_i32 << 7_i32) as libc::c_uint) as UInt16;
            // RangeDec_DecodeBit0(size0);
            (*p).Range = size0;
            while (*p).Low ^ (*p).Low.wrapping_add((*p).Range) < (1_i32 << 24_i32) as libc::c_uint
                || (*p).Range < (1_i32 << 15_i32) as libc::c_uint && {
                    (*p).Range = (0_i32 as libc::c_uint).wrapping_sub((*p).Low)
                        & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                    1_i32 != 0
                }
            {
                (*p).Code = (*p).Code << 8_i32
                    | (*(*p).Stream.In).Read.expect("non-null function pointer")((*p).Stream.In)
                        as libc::c_uint;
                (*p).Range <<= 8_i32;
                (*p).Low <<= 8_i32
            }
            // sym = (p->FoundState = Ppmd8Context_OneState(p->MinContext))->Symbol;
            // Ppmd8_UpdateBin(p);
            let freq: libc::c_uint = (*s_0).Freq as libc::c_uint;
            let c: CTX_PTR = (*p).Base.offset(
                ((*s_0).Successor_0 as libc::c_uint | ((*s_0).Successor_1 as UInt32) << 16_i32)
                    as isize,
            ) as *mut libc::c_void as *mut CPpmd8_Context;
            sym_1 = (*s_0).Symbol;
            (*p).FoundState = s_0;
            (*p).PrevSuccess = 1_i32 as libc::c_uint;
            (*p).RunLength += 1;
            (*s_0).Freq = freq
                .wrapping_add((freq < 196_i32 as libc::c_uint) as libc::c_int as libc::c_uint)
                as Byte;
            // NextContext(p);
            if (*p).OrderFall == 0_i32 as libc::c_uint && c as *const Byte >= (*p).UnitsStart {
                (*p).MinContext = c;
                (*p).MaxContext = (*p).MinContext
            } else {
                Ppmd8_UpdateModel(p);
            }
            return sym_1 as libc::c_int;
        }
        *prob = pr as UInt16;
        (*p).InitEsc = (*p).ExpEscape[(pr >> 10_i32) as usize] as libc::c_uint;
        // RangeDec_DecodeBit1(rc2, size0);
        (*p).Low = ((*p).Low as libc::c_uint).wrapping_add(size0) as UInt32 as UInt32;
        (*p).Code = ((*p).Code as libc::c_uint).wrapping_sub(size0) as UInt32 as UInt32;
        (*p).Range = ((*p).Range
            & !((1_i32 << (7_i32 + 7_i32)) as UInt32).wrapping_sub(1_i32 as libc::c_uint))
        .wrapping_sub(size0);
        let mut z_0: size_t = 0;
        z_0 = 0_i32 as size_t;
        while z_0
            < (256_i32 as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<size_t>() as libc::c_ulong)
        {
            charMask[z_0.wrapping_add(0_i32 as libc::c_ulong) as usize] = !(0_i32 as size_t);
            charMask[z_0.wrapping_add(1_i32 as libc::c_ulong) as usize] =
                charMask[z_0.wrapping_add(0_i32 as libc::c_ulong) as usize];
            charMask[z_0.wrapping_add(2_i32 as libc::c_ulong) as usize] =
                charMask[z_0.wrapping_add(1_i32 as libc::c_ulong) as usize];
            charMask[z_0.wrapping_add(3_i32 as libc::c_ulong) as usize] =
                charMask[z_0.wrapping_add(2_i32 as libc::c_ulong) as usize];
            charMask[z_0.wrapping_add(4_i32 as libc::c_ulong) as usize] =
                charMask[z_0.wrapping_add(3_i32 as libc::c_ulong) as usize];
            charMask[z_0.wrapping_add(5_i32 as libc::c_ulong) as usize] =
                charMask[z_0.wrapping_add(4_i32 as libc::c_ulong) as usize];
            charMask[z_0.wrapping_add(6_i32 as libc::c_ulong) as usize] =
                charMask[z_0.wrapping_add(5_i32 as libc::c_ulong) as usize];
            charMask[z_0.wrapping_add(7_i32 as libc::c_ulong) as usize] =
                charMask[z_0.wrapping_add(6_i32 as libc::c_ulong) as usize];
            z_0 = (z_0 as libc::c_ulong).wrapping_add(8_i32 as libc::c_ulong) as size_t as size_t
        }
        *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset(
            (*(&mut (*(*p).MinContext).Union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State)).Symbol
                as isize,
        ) = 0_i32 as libc::c_uchar;
        (*p).PrevSuccess = 0_i32 as libc::c_uint
    }
    loop {
        let mut s_1: *mut CPpmd_State = std::ptr::null_mut::<CPpmd_State>();
        let mut s2_0: *mut CPpmd_State = std::ptr::null_mut::<CPpmd_State>();
        let mut freqSum: UInt32 = 0;
        let mut count_0: UInt32 = 0;
        let mut hiCnt_0: UInt32 = 0;
        let mut freqSum2: UInt32 = 0;
        let mut see: *mut CPpmd_See = std::ptr::null_mut::<CPpmd_See>();
        let mut mc: *mut CPpmd8_Context = std::ptr::null_mut::<CPpmd8_Context>();
        let mut numMasked: libc::c_uint = 0;
        while (*p).Low ^ (*p).Low.wrapping_add((*p).Range) < (1_i32 << 24_i32) as libc::c_uint
            || (*p).Range < (1_i32 << 15_i32) as libc::c_uint && {
                (*p).Range = (0_i32 as libc::c_uint).wrapping_sub((*p).Low)
                    & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                1_i32 != 0
            }
        {
            (*p).Code = (*p).Code << 8_i32
                | (*(*p).Stream.In).Read.expect("non-null function pointer")((*p).Stream.In)
                    as libc::c_uint;
            (*p).Range <<= 8_i32;
            (*p).Low <<= 8_i32
        }
        mc = (*p).MinContext;
        numMasked = (*mc).NumStats as libc::c_uint;
        loop {
            (*p).OrderFall = (*p).OrderFall.wrapping_add(1);
            if (*mc).Suffix == 0 {
                return -1_i32;
            }
            mc =
                (*p).Base.offset((*mc).Suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context;
            if (*mc).NumStats as libc::c_uint != numMasked {
                break;
            }
        }
        s_1 =
            (*p).Base.offset((*mc).Union4.Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
        let mut num: libc::c_uint =
            ((*mc).NumStats as libc::c_uint).wrapping_add(1_i32 as libc::c_uint);
        let mut num2: libc::c_uint = num.wrapping_div(2_i32 as libc::c_uint);
        num &= 1_i32 as libc::c_uint;
        hiCnt_0 = (*s_1).Freq as libc::c_uint
            & *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset((*s_1).Symbol as isize)
                as libc::c_uint
            & (0_i32 as libc::c_uint).wrapping_sub(num);
        s_1 = s_1.offset(num as isize);
        (*p).MinContext = mc;
        loop {
            let sym0_0: libc::c_uint = (*s_1.offset(0_i32 as isize)).Symbol as libc::c_uint;
            let sym1_0: libc::c_uint = (*s_1.offset(1_i32 as isize)).Symbol as libc::c_uint;
            s_1 = s_1.offset(2_i32 as isize);
            hiCnt_0 = (hiCnt_0 as libc::c_uint).wrapping_add(
                (*s_1.offset(-2_i32 as isize)).Freq as libc::c_uint
                    & *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset(sym0_0 as isize)
                        as libc::c_uint,
            ) as UInt32 as UInt32;
            hiCnt_0 = (hiCnt_0 as libc::c_uint).wrapping_add(
                (*s_1.offset(-1_i32 as isize)).Freq as libc::c_uint
                    & *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset(sym1_0 as isize)
                        as libc::c_uint,
            ) as UInt32 as UInt32;
            num2 = num2.wrapping_sub(1);
            if num2 == 0 {
                break;
            }
        }
        see = Ppmd8_MakeEscFreq(p, numMasked, &mut freqSum);
        freqSum = (freqSum as libc::c_uint).wrapping_add(hiCnt_0) as UInt32 as UInt32;
        freqSum2 = freqSum;
        if freqSum2 > (*p).Range {
            freqSum2 = (*p).Range
        }
        (*p).Range = ((*p).Range as libc::c_uint).wrapping_div(freqSum2) as UInt32 as UInt32;
        count_0 = (*p).Code.wrapping_div((*p).Range);
        if count_0 < hiCnt_0 {
            let mut sym_2: Byte = 0;
            // Ppmd_See_Update(see); // new (see->Summ) value can overflow over 16-bits in some rare cases
            s_1 = (*p).Base.offset((*(*p).MinContext).Union4.Stats as isize) as *mut libc::c_void
                as *mut CPpmd_State;
            hiCnt_0 = count_0;
            loop {
                count_0 = (count_0 as libc::c_uint).wrapping_sub(
                    (*s_1).Freq as libc::c_uint
                        & *(charMask.as_mut_ptr() as *mut libc::c_uchar)
                            .offset((*s_1).Symbol as isize)
                            as libc::c_uint,
                ) as UInt32 as UInt32;
                s_1 = s_1.offset(1);
                if (count_0 as Int32) < 0_i32 {
                    break;
                }
                // count -= s->Freq & (unsigned)(MASK((s)->Symbol)); s++; if ((Int32)count < 0) break;
            }
            s_1 = s_1.offset(-1);
            RangeDec_Decode(
                p,
                hiCnt_0
                    .wrapping_sub(count_0)
                    .wrapping_sub((*s_1).Freq as libc::c_uint),
                (*s_1).Freq as UInt32,
            );
            while (*p).Low ^ (*p).Low.wrapping_add((*p).Range) < (1_i32 << 24_i32) as libc::c_uint
                || (*p).Range < (1_i32 << 15_i32) as libc::c_uint && {
                    (*p).Range = (0_i32 as libc::c_uint).wrapping_sub((*p).Low)
                        & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                    1_i32 != 0
                }
            {
                (*p).Code = (*p).Code << 8_i32
                    | (*(*p).Stream.In).Read.expect("non-null function pointer")((*p).Stream.In)
                        as libc::c_uint;
                (*p).Range <<= 8_i32;
                (*p).Low <<= 8_i32
            }
            // new (see->Summ) value can overflow over 16-bits in some rare cases
            if ((*see).Shift as libc::c_int) < 7_i32 && {
                (*see).Count = (*see).Count.wrapping_sub(1);
                ((*see).Count as libc::c_int) == 0_i32
            } {
                (*see).Summ = (((*see).Summ as libc::c_int) << 1_i32) as UInt16;
                let fresh0 = (*see).Shift;
                (*see).Shift = (*see).Shift.wrapping_add(1);
                (*see).Count = (3_i32 << fresh0 as libc::c_int) as Byte
            }
            (*p).FoundState = s_1;
            sym_2 = (*s_1).Symbol;
            Ppmd8_Update2(p);
            return sym_2 as libc::c_int;
        }
        if count_0 >= freqSum2 {
            return -2_i32;
        }
        RangeDec_Decode(p, hiCnt_0, freqSum2.wrapping_sub(hiCnt_0));
        // We increase (see->Summ) for sum of Freqs of all non_Masked symbols.
        // new (see->Summ) value can overflow over 16-bits in some rare cases
        (*see).Summ = ((*see).Summ as libc::c_uint).wrapping_add(freqSum) as UInt16;
        s_1 = (*p).Base.offset((*(*p).MinContext).Union4.Stats as isize) as *mut libc::c_void
            as *mut CPpmd_State;
        s2_0 = s_1
            .offset((*(*p).MinContext).NumStats as libc::c_int as isize)
            .offset(1_i32 as isize);
        loop {
            *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset((*s_1).Symbol as isize) =
                0_i32 as libc::c_uchar;
            s_1 = s_1.offset(1);
            if s_1 == s2_0 {
                break;
            }
        }
    }
}
