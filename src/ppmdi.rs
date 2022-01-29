/// 7ztypes
type Byte = u8;

struct IByteIn {
    Read: fn(p: *mut IByteIn) -> Byte,
}

struct IByteOut {
    Write: fn(p: *mut IByteOut, b: Byte),
}

pub type ISzAllocPtr = *const ISzAlloc;

struct ISzAlloc {
    Alloc: fn(p: ISzAllocPtr, size: usize) -> *mut u8,
    Free: fn(p: ISzAllocPtr, address: *mut u8),
}

//#define ISzAlloc_Alloc(p, size) (p)->Alloc(p, size)

fn ISzAlloc_Alloc(alloc: ISzAllocPtr, size: usize) -> *mut u8 {
    unsafe { ((*alloc).Alloc)(alloc, size) }
}

fn ISzAlloc_Free(alloc: ISzAllocPtr, address: *mut u8) {
    unsafe {
        ((*alloc).Free)(alloc, address);
    }
}

/// PPMD.h specific
/// -----------------------------------

const PPMD_N1: i32 = 4;
const PPMD_N2: i32 = 4;
const PPMD_N3: i32 = 4;
const PPMD_N4: i32 = ((128 + 3 - 1 * PPMD_N1 - 2 * PPMD_N2 - 3 * PPMD_N3) / 4);
const PPMD_NUM_INDEXES: i32 = PPMD_N1 + PPMD_N2 + PPMD_N3 + PPMD_N4;

type CPpmd_State_Ref = u32;
type CPpmd_Void_Ref = u32;
type CPpmd_Byte_Ref = u32;

struct CPpmd_see {
    Summ: u16,
    Shift: Byte,
    Count: Byte,
}

struct CPpmd_State {
    Symbol: Byte,
    Freq: Byte,
    SuccessorLow: u16,
    SuccessorHigh: u16,
}

/// PPMD8.h
///------------------------------
const PPMD8_MIN_ORDER: i32 = 8;
const PPMD8_MAX_ORDER: i32 = 16;

type CPpmd8_Context_Ref = u32;

struct Cppmd8_Context {
    NumStats: Byte,
    Flags: Byte,
    SummFreq: u16,
    Stats: CPpmd_State_Ref,
    Suffix: CPpmd8_Context_Ref,
}

enum Ppmd8_Modes {
    PPMD8_RESTORE_METHOD_RESTART,
    PPMD8_RESTORE_METHOD_CUT_OFF,
    PPMD8_RESTORE_METHOD_FREEZE, // TODO: Disable it under the feature since the original code does it: https://github.com/svpv/ppmd-mini/blob/master/lib/Ppmd8.h#L42
}

union StreamType {
    In: *mut IByteIn,
    Out: *mut IByteOut,
}

pub struct CPpmd8 {
    MinContext: *mut Cppmd8_Context,
    MaxContext: *mut Cppmd8_Context,
    FoundState: *mut CPpmd_State,
    OrderFall: u32,
    InitEsc: u32,
    PrevSuccess: u32,
    MaxOrder: u32,
    RunLength: i32,
    InitRL: i32,

    Size: u32,
    GlueCount: u32,
    Base: *mut Byte,
    LoUnit: *mut Byte,
    HiUnit: *mut Byte,
    Text: *mut Byte,
    UnitsStart: *mut Byte,
    AlignOffset: u32,
    RestoreMethod: u32,

    // Range coder
    Range: u32,
    Code: u32,
    Low: u32,
    Stream: StreamType,

    Indx2Units: [Byte; PPMD_NUM_INDEXES as usize],
    Units2Indx: [Byte; 128],
    FreeList: [CPpmd_Void_Ref; PPMD_NUM_INDEXES as usize],
    Stamps: [u32; PPMD_NUM_INDEXES as usize],

    NS2BSIndx: [Byte; 256],
    NS2Indx: [Byte; 260],
    DummySee: CPpmd_see,
    See: [[CPpmd_see; 32]; 24],
    BinSumm: [[u16; 64]; 25],
}

/// ppmd8.c

const PPMD8_kExpEscape: [Byte; 16] = [25, 14, 9, 7, 5, 5, 4, 4, 4, 3, 3, 3, 2, 2, 2, 2];
const kInitBinEsc: [u16; 8] = [
    0x3CDD, 0x1F3F, 0x59BF, 0x48F3, 0x64A1, 0x5ABC, 0x6632, 0x6051,
];
const MAX_FREQ: i32 = 124;
const UNIT_SIZE: u32 = 12;

type CPpmd8_Node_Ref = u32;

struct CPpmd8_Node {
    Stamp: u32,
    Next: CPpmd8_Node_Ref,
    NU: u32,
}

const EMPTY_NODE: u32 = 0xFFFFFFFF;

// #define U2B(nu) ((UInt32)(nu) * UNIT_SIZE)
fn U2B(nu: Byte) -> u32 {
    (nu as u32) * UNIT_SIZE
}

//#define U2I(nu) (p->Units2Indx[(size_t)(nu) - 1])
fn U2I(p: *mut CPpmd8, nu: Byte) -> Byte {
    unsafe { (*p).Units2Indx[((nu as isize) - 1) as usize] }
}

// #define I2U(indx) (p->Indx2Units[indx])
fn I2U(p: *mut CPpmd8, indx: u32) -> Byte {
    unsafe { (*p).Indx2Units[indx as usize] }
}

// #define REF(ptr) ((UInt32)((Byte *)(ptr) - (p)->Base))
fn REF(ptr: *mut u8, p: *mut CPpmd8) -> u32 {
    unsafe { ((ptr as usize) - ((*p).Base as usize)) as u32 }
}

fn NODE(offs: isize, p: *mut CPpmd8) -> *mut CPpmd8_Node {
    unsafe { ((*p).Base.offset(offs)) as *mut CPpmd8_Node }
}

pub fn Ppmd8_Construct(p: *mut CPpmd8) {
    let mut i: u32 = 0;
    let mut k: u32 = 0;
    let mut m: u32 = 0;
    unsafe {
        (*p).Base = std::ptr::null_mut();
    }

    k = 0;
    for i in 0..PPMD_NUM_INDEXES {
        let mut step: u32 = if i >= 12 {
            4
        } else {
            ((i >> 2) + 1).try_into().unwrap()
        };

        loop {
            unsafe {
                (*p).Units2Indx[k as usize] = i as Byte;
            }
            k += 1;
            step -= 1;
            if step == 0 {
                break;
            }
        }
    }

    unsafe {
        (*p).NS2BSIndx[0] = (0 << 1);
        (*p).NS2BSIndx[1] = (1 << 1);
        std::ptr::write_bytes((*p).NS2BSIndx.as_mut_ptr().offset(2), (2 << 1), 9);
        std::ptr::write_bytes((*p).NS2BSIndx.as_mut_ptr().offset(11), (3 << 1), 256 - 11);
    }

    for i in 0..5 {
        unsafe {
            (*p).NS2Indx[i] = i as Byte;
        }
    }

    m = i;
    k = 1;
    while i < 260 {
        unsafe {
            (*p).NS2Indx[i as usize] = m as Byte;
        }
        k -= 1;
        if k == 0 {
            m += 1;
            k = m - 4;
        }
        i += 1;
    }
}

pub fn Ppmd8_Free(p: *mut CPpmd8, alloc: ISzAllocPtr) {
    unsafe {
        ISzAlloc_Free(alloc, (*p).Base);
        (*p).Size = 0;
        (*p).Base = std::ptr::null_mut();
    }
}

pub fn Ppmd8_Alloc(p: *mut CPpmd8, size: u32, alloc: ISzAllocPtr) -> bool {
    unsafe {
        if (*p).Base == std::ptr::null_mut() || (*p).Size != size {
            Ppmd8_Free(p, alloc);
            (*p).AlignOffset = 4 - (size & 3);

            (*p).Base = ISzAlloc_Alloc(alloc, ((*p).AlignOffset + size) as usize);

            if (*p).Base == std::ptr::null_mut() {
                return false;
            }
            (*p).Size = size;
        }
    }
    return true;
}

fn InsertNode(p: *mut CPpmd8, node: *mut u8, indx: u32) {
    unsafe {
        (*(node as (*mut CPpmd8_Node))).Stamp = EMPTY_NODE;
        (*(node as (*mut CPpmd8_Node))).Next = (*p).FreeList[indx as usize];
        (*(node as (*mut CPpmd8_Node))).NU = I2U(p, indx) as u32;
        (*p).FreeList[indx as usize] = REF(node, p);
        (*p).Stamps[indx as usize] += 1;
    }
}

fn RemoveNode(p: *mut CPpmd8, indx: u32) -> *mut u8 {
    unsafe {
        let node: *mut CPpmd8_Node = NODE((*p).FreeList[indx as usize] as isize, p);
        (*p).FreeList[indx as usize] = (*node).Next;
        (*p).Stamps[indx as usize] -= 1;
        node as *mut u8
    }
}

fn SplitBlock(p: *mut CPpmd8, mut ptr: *mut u8, oldIndx: u32, newIndx: u32) {
    unsafe {
        let mut i = 0u32;
        let mut nu = I2U(p, oldIndx) - I2U(p, newIndx);
        ptr = (ptr as *mut Byte).offset(U2B(I2U(p, newIndx)) as isize);

        i = U2I(p, nu) as u32;
        if I2U(p, i) != nu {
            i -= 1;
            let k = I2U(p, i);
            InsertNode(
                p,
                (ptr as *mut Byte).offset(U2B(k) as isize),
                (nu - k - 1) as u32,
            );
        }

        InsertNode(p, ptr, i);
    }
}

fn GlueFreeBlocks(p: *mut CPpmd8) {
    unsafe {
        let mut head: CPpmd8_Node_Ref = 0;
        let mut prev: *mut CPpmd8_Node_Ref = &mut head;

        let mut i = 0u32;

        (*p).GlueCount = 1 << 13;
        std::ptr::write_bytes((*p).Stamps.as_mut_ptr(), 0, (*p).Stamps.len());

        /* Order-0 context is always at top UNIT, so we don't need guard NODE at the end.
        All blocks up to p->LoUnit can be free, so we need guard NODE at LoUnit. */
        if (*p).LoUnit != (*p).HiUnit {
            (*((*p).LoUnit as *mut CPpmd8_Node)).Stamp = 0;
        }

        /* Glue free blocks */
        while i < PPMD_NUM_INDEXES as u32 {
            let mut next: CPpmd8_Node_Ref = (*p).FreeList[i as usize];
            (*p).FreeList[i as usize] = 0;
            while next != 0 {
                let node: *mut CPpmd8_Node = NODE(next as isize, p);
                if (*node).NU != 0 {
                    let node2: *mut CPpmd8_Node;
                    *prev = next;
                    prev = &mut (*node).Next;
                    //while
                }
            }
            i += 1;
        }
    }
}
