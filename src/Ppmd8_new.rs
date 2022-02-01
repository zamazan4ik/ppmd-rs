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
pub struct Header {
    pub magic: u32,
    pub attr: u32,
    pub info: libc::c_ushort,
    pub fnlen: libc::c_ushort,
    pub date: libc::c_ushort,
    pub time: libc::c_ushort,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            magic: 0x84acaf8f_u32,
            attr: 0x80_i32 as u32,
            info: 0,
            fnlen: 1,
            date: 0,
            time: 0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharWriter {
    pub write: Option<unsafe fn(_: *mut libc::c_void, _: libc::c_uchar) -> ()>,
    pub fp: *mut libc::FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharReader {
    pub read: Option<unsafe fn(_: *mut libc::c_void) -> libc::c_uchar>,
    pub fp: *mut libc::FILE,
    pub eof: bool,
}

pub unsafe fn write(p: *mut libc::c_void, b: libc::c_uchar) {
    let cw: *mut CharWriter = p as *mut CharWriter;
    libc::fputc(b as i32, (*cw).fp as *mut libc::FILE);
}
pub unsafe fn read(p: *mut libc::c_void) -> libc::c_uchar {
    let mut cr: *mut CharReader = p as *mut CharReader;
    if (*cr).eof {
        return 0_i32 as libc::c_uchar;
    }
    let c: i32 = libc::fgetc((*cr).fp as *mut libc::FILE);
    if c == -1_i32 {
        (*cr).eof = 1_i32 != 0;
        return 0_i32 as libc::c_uchar;
    }
    c as libc::c_uchar
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct IByteIn {
    pub read: Option<unsafe fn(_: *const IByteIn) -> Byte>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IByteOut {
    pub write: Option<unsafe fn(_: *const IByteOut, _: Byte) -> ()>,
}
/* Returns: result. (result != SZ_OK) means break.
Value (UInt64)(Int64)-1 for size means unknown value. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ISzAlloc {
    pub alloc: Option<unsafe fn(_: ISzAllocPtr, _: size_t) -> *mut libc::c_void>,
    pub free: Option<unsafe fn(_: ISzAllocPtr, _: *mut libc::c_void) -> ()>,
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
    pub summ: UInt16,
    pub shift: Byte,
    pub count: Byte,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct CPpmd_State {
    pub symbol: Byte,
    pub freq: Byte,
    pub successor_0: UInt16,
    pub successor_1: UInt16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct CPpmd_State2_ {
    pub symbol: Byte,
    pub freq: Byte,
}
pub type CPpmd_State2 = CPpmd_State2_;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct CPpmd_State4_ {
    pub successor_0: UInt16,
    pub successor_1: UInt16,
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
    pub num_stats: Byte,
    pub flags: Byte,
    pub union2: C2RustUnnamed_0,
    pub union4: C2RustUnnamed,
    pub suffix: CPpmd8_Context_Ref,
}
pub type CPpmd8_Context_Ref = UInt32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub stats: CPpmd_State_Ref,
    pub state4: CPpmd_State4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub summ_freq: UInt16,
    pub state2: CPpmd_State2,
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
    pub min_context: *mut CPpmd8_Context,
    pub max_context: *mut CPpmd8_Context,
    pub found_state: *mut CPpmd_State,
    pub order_fall: libc::c_uint,
    pub init_esc: libc::c_uint,
    pub prev_success: libc::c_uint,
    pub max_order: libc::c_uint,
    pub restore_method: libc::c_uint,
    pub run_length: Int32,
    pub init_rl: Int32,
    pub size: UInt32,
    pub glue_count: UInt32,
    pub align_offset: UInt32,
    pub base: *mut Byte,
    pub lo_unit: *mut Byte,
    pub hi_unit: *mut Byte,
    pub text: *mut Byte,
    pub units_start: *mut Byte,
    pub range: UInt32,
    pub code: UInt32,
    pub low: UInt32,
    pub stream: C2RustUnnamed_2,
    pub indx2units: [Byte; 40],
    pub units2indx: [Byte; 128],
    pub free_list: [CPpmd_Void_Ref; 38],
    pub stamps: [UInt32; 38],
    pub ns2bsindx: [Byte; 256],
    pub ns2indx: [Byte; 260],
    pub exp_escape: [Byte; 16],
    pub dummy_see: CPpmd_See,
    pub see: [[CPpmd_See; 32]; 24],
    pub bin_summ: [[UInt16; 64]; 25],
}

impl CPpmd8 {
    fn default_encoder(char_writer: &mut CharWriter) -> Self {
        Self {
            min_context: std::ptr::null_mut::<CPpmd8_Context>(),
            max_context: std::ptr::null_mut::<CPpmd8_Context>(),
            found_state: std::ptr::null_mut::<CPpmd_State>(),
            order_fall: 0,
            init_esc: 0,
            prev_success: 0,
            max_order: 0,
            run_length: 0,
            init_rl: 0,
            size: 0,
            glue_count: 0,
            base: std::ptr::null_mut::<u8>(),
            lo_unit: std::ptr::null_mut::<u8>(),
            hi_unit: std::ptr::null_mut::<u8>(),
            text: std::ptr::null_mut::<u8>(),
            units_start: std::ptr::null_mut::<u8>(),
            align_offset: 0,
            restore_method: 0,
            range: 0,
            code: 0,
            low: 0,
            stream: C2RustUnnamed_2 {
                out: char_writer as *mut CharWriter as *mut IByteOut,
            },
            indx2units: [0; 40],
            units2indx: [0; 128],
            free_list: [0; 38],
            stamps: [0; 38],
            ns2bsindx: [0; 256],
            ns2indx: [0; 260],
            exp_escape: [0; 16],
            dummy_see: CPpmd_See {
                summ: 0,
                shift: 0,
                count: 0,
            },
            see: [[CPpmd_See {
                summ: 0,
                shift: 0,
                count: 0,
            }; 32]; 24],
            bin_summ: [[0; 64]; 25],
        }
    }

    fn default_decoder(char_reader: &mut CharReader) -> Self {
        Self {
            min_context: std::ptr::null_mut::<CPpmd8_Context>(),
            max_context: std::ptr::null_mut::<CPpmd8_Context>(),
            found_state: std::ptr::null_mut::<CPpmd_State>(),
            order_fall: 0,
            init_esc: 0,
            prev_success: 0,
            max_order: 0,
            run_length: 0,
            init_rl: 0,
            size: 0,
            glue_count: 0,
            base: std::ptr::null_mut::<u8>(),
            lo_unit: std::ptr::null_mut::<u8>(),
            hi_unit: std::ptr::null_mut::<u8>(),
            text: std::ptr::null_mut::<u8>(),
            units_start: std::ptr::null_mut::<u8>(),
            align_offset: 0,
            restore_method: 0,
            range: 0,
            code: 0,
            low: 0,
            stream: C2RustUnnamed_2 {
                r#in: char_reader as *mut CharReader as *mut IByteIn,
            },
            indx2units: [0; 40],
            units2indx: [0; 128],
            free_list: [0; 38],
            stamps: [0; 38],
            ns2bsindx: [0; 256],
            ns2indx: [0; 260],
            exp_escape: [0; 16],
            dummy_see: CPpmd_See {
                summ: 0,
                shift: 0,
                count: 0,
            },
            see: [[CPpmd_See {
                summ: 0,
                shift: 0,
                count: 0,
            }; 32]; 24],
            bin_summ: [[0; 64]; 25],
        }
    }

    pub unsafe fn Ppmd8_Construct(&mut self) {
        let mut i: libc::c_uint = 0;
        let mut k: libc::c_uint = 0;
        let mut m: libc::c_uint = 0;
        self.base = std::ptr::null_mut::<Byte>();
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
                self.units2indx[fresh0 as usize] = i as Byte;
                step = step.wrapping_sub(1);
                if step == 0 {
                    break;
                }
            }
            self.indx2units[i as usize] = k as Byte;
            i = i.wrapping_add(1)
        }
        self.ns2bsindx[0_i32 as usize] = (0_i32 << 1_i32) as Byte;
        self.ns2bsindx[1_i32 as usize] = (1_i32 << 1_i32) as Byte;
        memset(
            self.ns2bsindx.as_mut_ptr().offset(2_i32 as isize) as *mut libc::c_void,
            2_i32 << 1_i32,
            9_i32 as libc::c_ulong,
        );
        memset(
            self.ns2bsindx.as_mut_ptr().offset(11_i32 as isize) as *mut libc::c_void,
            3_i32 << 1_i32,
            (256_i32 - 11_i32) as libc::c_ulong,
        );
        i = 0_i32 as libc::c_uint;
        while i < 5_i32 as libc::c_uint {
            self.ns2indx[i as usize] = i as Byte;
            i = i.wrapping_add(1)
        }
        m = i;
        k = 1_i32 as libc::c_uint;
        while i < 260_i32 as libc::c_uint {
            self.ns2indx[i as usize] = m as Byte;
            k = k.wrapping_sub(1);
            if k == 0_i32 as libc::c_uint {
                m = m.wrapping_add(1);
                k = m.wrapping_sub(4_i32 as libc::c_uint)
            }
            i = i.wrapping_add(1)
        }
        memcpy(
            self.exp_escape.as_mut_ptr() as *mut libc::c_void,
            PPMD8_kExpEscape.as_ptr() as *const libc::c_void,
            16_i32 as libc::c_ulong,
        );
    }

    pub unsafe fn new_encoder(char_writer: &mut CharWriter) -> Self {
        let mut initial_state = Self::default_encoder(char_writer);
        initial_state.Ppmd8_Construct();
        initial_state
    }

    pub unsafe fn new_decoder(char_reader: &mut CharReader) -> Self {
        let mut initial_state = Self::default_decoder(char_reader);
        initial_state.Ppmd8_Construct();
        initial_state
    }

    pub unsafe fn free(&mut self, alloc: ISzAllocPtr) {
        (*alloc).free.expect("non-null function pointer")(alloc, self.base as *mut libc::c_void);
        self.size = 0_i32 as UInt32;
        self.base = std::ptr::null_mut::<Byte>();
    }

    pub unsafe fn allocate(&mut self, size: UInt32, alloc: ISzAllocPtr) -> BoolInt {
        if self.base.is_null() || self.size != size {
            self.free(alloc);
            self.align_offset = (4_i32 as libc::c_uint).wrapping_sub(size) & 3_i32 as libc::c_uint;
            self.base = (*alloc).alloc.expect("non-null function pointer")(
                alloc,
                self.align_offset.wrapping_add(size) as size_t,
            ) as *mut Byte;
            if self.base.is_null() {
                return 0_i32;
            }
            self.size = size
        }
        1_i32
    }
    unsafe fn InsertNode(&mut self, node: *mut libc::c_void, indx: libc::c_uint) {
        (*(node as *mut CPpmd8_Node)).Stamp = 0xffffffff_u32;
        (*(node as *mut CPpmd8_Node)).Next = self.free_list[indx as usize];
        (*(node as *mut CPpmd8_Node)).NU = self.indx2units[indx as usize] as libc::c_uint;
        self.free_list[indx as usize] =
            (node as *mut Byte).offset_from(self.base) as libc::c_long as UInt32;
        self.stamps[indx as usize] = self.stamps[indx as usize].wrapping_add(1);
    }
    unsafe fn RemoveNode(&mut self, indx: libc::c_uint) -> *mut libc::c_void {
        let node: *mut CPpmd8_Node = self.base.offset(self.free_list[indx as usize] as isize)
            as *mut libc::c_void as *mut CPpmd8_Node;
        self.free_list[indx as usize] = (*node).Next;
        self.stamps[indx as usize] = self.stamps[indx as usize].wrapping_sub(1);
        node as *mut libc::c_void
    }
    unsafe fn SplitBlock(
        &mut self,
        mut ptr: *mut libc::c_void,
        oldIndx: libc::c_uint,
        newIndx: libc::c_uint,
    ) {
        let mut i: libc::c_uint = 0;
        let nu: libc::c_uint = (self.indx2units[oldIndx as usize] as libc::c_uint)
            .wrapping_sub(self.indx2units[newIndx as usize] as libc::c_uint);
        ptr = (ptr as *mut Byte).offset(
            (self.indx2units[newIndx as usize] as libc::c_uint).wrapping_mul(12_i32 as libc::c_uint)
                as isize,
        ) as *mut libc::c_void;
        i = self.units2indx[(nu as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
            as libc::c_uint;
        if self.indx2units[i as usize] as libc::c_uint != nu {
            i = i.wrapping_sub(1);
            let k: libc::c_uint = self.indx2units[i as usize] as libc::c_uint;
            self.InsertNode(
                (ptr as *mut Byte).offset(k.wrapping_mul(12_i32 as libc::c_uint) as isize)
                    as *mut libc::c_void,
                nu.wrapping_sub(k).wrapping_sub(1_i32 as libc::c_uint),
            );
        }
        self.InsertNode(ptr, i);
    }
    unsafe fn GlueFreeBlocks(&mut self) {
        /*
        we use first UInt32 field of 12-bytes UNITs as record type stamp
          CPpmd_State    { Byte symbol; Byte freq; : freq != 0xFF
          CPpmd8_Context { Byte num_stats; Byte flags; UInt16 summ_freq;  : flags != 0xFF ???
          CPpmd8_Node    { UInt32 Stamp            : Stamp == 0xFFFFFFFF for free record
                                                   : Stamp == 0 for guard
          Last 12-bytes UNIT in array is always contains 12-bytes order-0 CPpmd8_Context record
        */
        let mut n: CPpmd8_Node_Ref = 0;
        self.glue_count = (1_i32 << 13_i32) as UInt32;
        memset(
            self.stamps.as_mut_ptr() as *mut libc::c_void,
            0_i32,
            ::std::mem::size_of::<[UInt32; 38]>() as libc::c_ulong,
        );
        /* we set guard NODE at lo_unit */
        if self.lo_unit != self.hi_unit {
            (*(self.lo_unit as *mut libc::c_void as *mut CPpmd8_Node)).Stamp = 0_i32 as UInt32
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
            let mut next: CPpmd8_Node_Ref = self.free_list[i as usize];
            self.free_list[i as usize] = 0_i32 as CPpmd_Void_Ref;
            while next != 0_i32 as libc::c_uint {
                let mut node: *mut CPpmd8_Node =
                    self.base.offset(next as isize) as *mut libc::c_void as *mut CPpmd8_Node;
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
                self.base.offset(n as isize) as *mut libc::c_void as *mut CPpmd8_Node;
            let mut nu_0: UInt32 = (*node_0).NU;
            let mut i_0: libc::c_uint = 0;
            n = (*node_0).Next;
            if nu_0 == 0_i32 as libc::c_uint {
                continue;
            }
            while nu_0 > 128_i32 as libc::c_uint {
                self.InsertNode(
                    node_0 as *mut libc::c_void,
                    (4_i32
                        + 4_i32
                        + 4_i32
                        + (128_i32 + 3_i32 - 1_i32 * 4_i32 - 2_i32 * 4_i32 - 3_i32 * 4_i32) / 4_i32
                        - 1_i32) as libc::c_uint,
                );
                nu_0 = (nu_0 as libc::c_uint).wrapping_sub(128_i32 as libc::c_uint) as UInt32
                    as UInt32;
                node_0 = node_0.offset(128_i32 as isize)
            }
            i_0 = self.units2indx[(nu_0 as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
                as libc::c_uint;
            if self.indx2units[i_0 as usize] as libc::c_uint != nu_0 {
                i_0 = i_0.wrapping_sub(1);
                let k: libc::c_uint = self.indx2units[i_0 as usize] as libc::c_uint;
                self.InsertNode(
                    node_0.offset(k as isize) as *mut libc::c_void,
                    nu_0.wrapping_sub(k).wrapping_sub(1_i32 as libc::c_uint),
                );
            }
            self.InsertNode(node_0 as *mut libc::c_void, i_0);
        }
    }
    #[inline(never)]
    unsafe fn AllocUnitsRare(&mut self, indx: libc::c_uint) -> *mut libc::c_void {
        let mut i: libc::c_uint = 0;
        if self.glue_count == 0_i32 as libc::c_uint {
            self.GlueFreeBlocks();
            if self.free_list[indx as usize] != 0_i32 as libc::c_uint {
                return self.RemoveNode(indx);
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
                let numBytes: UInt32 = (self.indx2units[indx as usize] as libc::c_uint)
                    .wrapping_mul(12_i32 as libc::c_uint);
                let us: *mut Byte = self.units_start;
                self.glue_count = self.glue_count.wrapping_sub(1);
                return if us.offset_from(self.text) as libc::c_long as UInt32 > numBytes {
                    self.units_start = us.offset(-(numBytes as isize));
                    self.units_start
                } else {
                    std::ptr::null_mut::<Byte>()
                } as *mut libc::c_void;
            }
            if self.free_list[i as usize] != 0_i32 as libc::c_uint {
                break;
            }
        }
        let block: *mut libc::c_void = self.RemoveNode(i);
        self.SplitBlock(block, i, indx);
        block
    }
    unsafe fn AllocUnits(&mut self, indx: libc::c_uint) -> *mut libc::c_void {
        if self.free_list[indx as usize] != 0_i32 as libc::c_uint {
            return self.RemoveNode(indx);
        }
        let numBytes: UInt32 =
            (self.indx2units[indx as usize] as libc::c_uint).wrapping_mul(12_i32 as libc::c_uint);
        let lo: *mut Byte = self.lo_unit;
        if self.hi_unit.offset_from(lo) as libc::c_long as UInt32 >= numBytes {
            self.lo_unit = lo.offset(numBytes as isize);
            return lo as *mut libc::c_void;
        }
        self.AllocUnitsRare(indx)
    }
    unsafe fn ShrinkUnits(
        &mut self,
        oldPtr: *mut libc::c_void,
        oldNU: libc::c_uint,
        newNU: libc::c_uint,
    ) -> *mut libc::c_void {
        let i0: libc::c_uint = self.units2indx
            [(oldNU as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
            as libc::c_uint;
        let i1: libc::c_uint = self.units2indx
            [(newNU as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
            as libc::c_uint;
        if i0 == i1 {
            return oldPtr;
        }
        if self.free_list[i1 as usize] != 0_i32 as libc::c_uint {
            let ptr: *mut libc::c_void = self.RemoveNode(i1);
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
            self.InsertNode(oldPtr, i0);
            return ptr;
        }
        self.SplitBlock(oldPtr, i0, i1);
        oldPtr
    }
    unsafe fn FreeUnits(&mut self, ptr: *mut libc::c_void, nu: libc::c_uint) {
        self.InsertNode(
            ptr,
            self.units2indx[(nu as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
                as libc::c_uint,
        );
    }
    unsafe fn SpecialFreeUnit(&mut self, ptr: *mut libc::c_void) {
        if ptr as *mut Byte != self.units_start {
            self.InsertNode(ptr, 0_i32 as libc::c_uint);
        } else {
            self.units_start = self.units_start.offset(12_i32 as isize)
        };
    }
    /*
    static void *MoveUnitsUp(CPpmd8 *p, void *oldPtr, unsigned nu)
    {
      unsigned indx = U2I(nu);
      void *ptr;
      if ((Byte *)oldPtr > p->units_start + (1 << 14) || REF(oldPtr) > p->free_list[indx])
        return oldPtr;
      ptr = self.RemoveNode(p, indx);
      MyMem12Cpy(ptr, oldPtr, nu);
      if ((Byte *)oldPtr != p->units_start)
        self.InsertNode(p, oldPtr, indx);
      else
        p->units_start += U2B(I2U(indx));
      return ptr;
    }
    */
    unsafe fn ExpandTextArea(&mut self) {
        let mut count: [UInt32; 38] = [0; 38]; /* AllocContext(p); */
        let mut i: libc::c_uint = 0; /* self.AllocUnits(p, PPMD_NUM_INDEXES - 1); */
        memset(
            count.as_mut_ptr() as *mut libc::c_void,
            0_i32,
            ::std::mem::size_of::<[UInt32; 38]>() as libc::c_ulong,
        ); /* unused */
        if self.lo_unit != self.hi_unit {
            (*(self.lo_unit as *mut libc::c_void as *mut CPpmd8_Node)).Stamp = 0_i32 as UInt32
        }
        let mut node: *mut CPpmd8_Node = self.units_start as *mut libc::c_void as *mut CPpmd8_Node;
        while (*node).Stamp == 0xffffffff_u32 {
            let nu: UInt32 = (*node).NU;
            (*node).Stamp = 0_i32 as UInt32;
            count[self.units2indx[(nu as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
                as usize] = count[self.units2indx
                [(nu as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
                as usize]
                .wrapping_add(1);
            node = node.offset(nu as isize)
        }
        self.units_start = node as *mut Byte;
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
                let mut prev: *mut CPpmd8_Node_Ref =
                    &mut *self.free_list.as_mut_ptr().offset(i as isize) as *mut CPpmd_Void_Ref
                        as *mut CPpmd8_Node_Ref;
                let mut n: CPpmd8_Node_Ref = *prev;
                self.stamps[i as usize] =
                    (self.stamps[i as usize] as libc::c_uint).wrapping_sub(cnt) as UInt32 as UInt32;
                loop {
                    let node_0: *mut CPpmd8_Node =
                        self.base.offset(n as isize) as *mut libc::c_void as *mut CPpmd8_Node;
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

    #[inline(never)]
    unsafe fn RestartModel(&mut self) {
        let mut i: libc::c_uint = 0;
        let mut k: libc::c_uint = 0;
        let mut m: libc::c_uint = 0;
        memset(
            self.free_list.as_mut_ptr() as *mut libc::c_void,
            0_i32,
            ::std::mem::size_of::<[CPpmd_Void_Ref; 38]>() as libc::c_ulong,
        );
        memset(
            self.stamps.as_mut_ptr() as *mut libc::c_void,
            0_i32,
            ::std::mem::size_of::<[UInt32; 38]>() as libc::c_ulong,
        );
        self.text = self
            .base
            .offset(self.align_offset as isize)
            .offset(0_i32 as isize);
        self.hi_unit = self.text.offset(self.size as isize);
        self.units_start = self.hi_unit.offset(
            -(self
                .size
                .wrapping_div(8_i32 as libc::c_uint)
                .wrapping_div(12_i32 as libc::c_uint)
                .wrapping_mul(7_i32 as libc::c_uint)
                .wrapping_mul(12_i32 as libc::c_uint) as isize),
        );
        self.lo_unit = self.units_start;
        self.glue_count = 0_i32 as UInt32;
        self.order_fall = self.max_order;
        self.init_rl = -((if self.max_order < 12_i32 as libc::c_uint {
            self.max_order
        } else {
            12_i32 as libc::c_uint
        }) as Int32)
            - 1_i32;
        self.run_length = self.init_rl;
        self.prev_success = 0_i32 as libc::c_uint;
        self.hi_unit = self.hi_unit.offset(-(12_i32 as isize));
        let mut mc: *mut CPpmd8_Context = self.hi_unit as *mut libc::c_void as CTX_PTR;
        let mut s: *mut CPpmd_State = self.lo_unit as *mut CPpmd_State;
        self.lo_unit = self
            .lo_unit
            .offset(((256_i32 / 2_i32) as UInt32).wrapping_mul(12_i32 as libc::c_uint) as isize);
        self.min_context = mc;
        self.max_context = self.min_context;
        self.found_state = s;
        (*mc).flags = 0_i32 as Byte;
        (*mc).num_stats = (256_i32 - 1_i32) as Byte;
        (*mc).union2.summ_freq = (256_i32 + 1_i32) as UInt16;
        (*mc).union4.stats = (s as *mut Byte).offset_from(self.base) as libc::c_long as UInt32;
        (*mc).suffix = 0_i32 as CPpmd8_Context_Ref;
        i = 0_i32 as libc::c_uint;
        while i < 256_i32 as libc::c_uint {
            (*s).symbol = i as Byte;
            (*s).freq = 1_i32 as Byte;
            SetSuccessor(s, 0_i32 as CPpmd_Void_Ref);
            i = i.wrapping_add(1);
            s = s.offset(1)
        }
        m = 0_i32 as libc::c_uint;
        i = m;
        while m < 25_i32 as libc::c_uint {
            while self.ns2indx[i as usize] as libc::c_uint == m {
                i = i.wrapping_add(1)
            }
            k = 0_i32 as libc::c_uint;
            while k < 8_i32 as libc::c_uint {
                let mut r: libc::c_uint = 0;
                let dest: *mut UInt16 = self.bin_summ[m as usize].as_mut_ptr().offset(k as isize);
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
            while self.ns2indx[(i as size_t).wrapping_add(3_i32 as libc::c_ulong) as usize]
                as libc::c_uint
                == m.wrapping_add(3_i32 as libc::c_uint)
            {
                i = i.wrapping_add(1)
            }
            s_0 = self.see[m as usize].as_mut_ptr();
            summ = (2_i32 as libc::c_uint)
                .wrapping_mul(i)
                .wrapping_add(5_i32 as libc::c_uint)
                << (7_i32 - 4_i32);
            k = 0_i32 as libc::c_uint;
            while k < 32_i32 as libc::c_uint {
                (*s_0).summ = summ as UInt16;
                (*s_0).shift = (7_i32 - 4_i32) as Byte;
                (*s_0).count = 7_i32 as Byte;
                k = k.wrapping_add(1);
                s_0 = s_0.offset(1)
            }
            m = m.wrapping_add(1)
        }
        self.dummy_see.summ = 0_i32 as UInt16;
        self.dummy_see.shift = 7_i32 as Byte;
        self.dummy_see.count = 64_i32 as Byte;
        /* unused */
    }

    pub unsafe fn Ppmd8_Init(&mut self, maxOrder: libc::c_uint, restoreMethod: libc::c_uint) {
        self.max_order = maxOrder;
        self.restore_method = restoreMethod;
        self.RestartModel();
    }
    // #define PPMD8_HiBitsFlag_3(sym) (0x08 * ((sym) >= 0x40))
    // #define PPMD8_HiBitsFlag_4(sym) (0x10 * ((sym) >= 0x40))
    /*
    self.Refresh() is called when we remove some symbols (successors) in context.
    It increases Escape_Freq for sum of all removed symbols.
    */
    unsafe fn Refresh(&mut self, mut ctx: CTX_PTR, oldNU: libc::c_uint, mut scale: libc::c_uint) {
        let mut i: libc::c_uint = (*ctx).num_stats as libc::c_uint;
        let mut escFreq: libc::c_uint = 0;
        let mut sumFreq: libc::c_uint = 0;
        let mut flags: libc::c_uint = 0;
        let mut s: *mut CPpmd_State = self.ShrinkUnits(
            self.base.offset((*ctx).union4.stats as isize) as *mut libc::c_void as *mut CPpmd_State
                as *mut libc::c_void,
            oldNU,
            i.wrapping_add(2_i32 as libc::c_uint) >> 1_i32,
        ) as *mut CPpmd_State;
        (*ctx).union4.stats = (s as *mut Byte).offset_from(self.base) as libc::c_long as UInt32;
        // #ifdef self.Ppmd8_FreeZE_SUPPORT
        /*
          (ctx->union2.summ_freq >= ((UInt32)1 << 15)) can be in FREEZE mode for some files.
          It's not good for range coder. So new versions of support fix:
             -   original PPMdI code rev.1
             +   original PPMdI code rev.2
             -   7-Zip default ((self.Ppmd8_FreeZE_SUPPORT is not defined)
             +   7-Zip (p->restore_method >= PPMD8_RESTORE_METHOD_FREEZE)
          if we       use that fixed line, we can lose compatibility with some files created before fix
          if we don't use that fixed line, the program can work incorrectly in FREEZE mode in rare case.
        */
        // if (p->restore_method >= PPMD8_RESTORE_METHOD_FREEZE)
        scale |= ((*ctx).union2.summ_freq as libc::c_uint >= (1_i32 as UInt32) << 15_i32)
            as libc::c_int as libc::c_uint;
        // #endif
        flags = ((*s).symbol as libc::c_uint).wrapping_add(0xc0_i32 as libc::c_uint);
        let mut freq: libc::c_uint = (*s).freq as libc::c_uint;
        escFreq = ((*ctx).union2.summ_freq as libc::c_uint).wrapping_sub(freq);
        freq = freq.wrapping_add(scale) >> scale;
        sumFreq = freq;
        (*s).freq = freq as Byte;
        loop {
            s = s.offset(1);
            let mut freq_0: libc::c_uint = (*s).freq as libc::c_uint;
            escFreq = escFreq.wrapping_sub(freq_0);
            freq_0 = freq_0.wrapping_add(scale) >> scale;
            sumFreq = sumFreq.wrapping_add(freq_0);
            (*s).freq = freq_0 as Byte;
            flags |= ((*s).symbol as libc::c_uint).wrapping_add(0xc0_i32 as libc::c_uint);
            i = i.wrapping_sub(1);
            if i == 0 {
                break;
            }
        }
        (*ctx).union2.summ_freq =
            sumFreq.wrapping_add(escFreq.wrapping_add(scale) >> scale) as UInt16;
        (*ctx).flags = ((*ctx).flags as libc::c_uint
            & ((1_i32 << 4_i32) as libc::c_uint)
                .wrapping_add(((1_i32 << 2_i32) as libc::c_uint).wrapping_mul(scale)))
        .wrapping_add(flags >> (8_i32 - 3_i32) & (1_i32 << 3_i32) as libc::c_uint)
            as Byte;
    }

    /*
    self.CutOff() reduces contexts:
      It conversts Successors at max_order to another Contexts to NULL-Successors
      It removes RAW-Successors and NULL-Successors that are not Order-0
          and it removes contexts when it has no Successors.
      if the (union4.stats) is close to (units_start), it moves it up.
    */
    unsafe fn CutOff(&mut self, mut ctx: CTX_PTR, order: libc::c_uint) -> CPpmd_Void_Ref {
        let mut ns: libc::c_int = (*ctx).num_stats as libc::c_int;
        let mut nu: libc::c_uint = 0;
        let mut stats: *mut CPpmd_State = std::ptr::null_mut::<CPpmd_State>();
        if ns == 0_i32 {
            let s: *mut CPpmd_State =
                &mut (*ctx).union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State;
            let mut successor: CPpmd_Void_Ref =
                (*s).successor_0 as libc::c_uint | ((*s).successor_1 as UInt32) << 16_i32;
            if self.base.offset(successor as isize) as *mut libc::c_void as *mut Byte
                >= self.units_start
            {
                if order < self.max_order {
                    successor = self.CutOff(
                        self.base.offset(successor as isize) as *mut libc::c_void
                            as *mut CPpmd8_Context,
                        order.wrapping_add(1_i32 as libc::c_uint),
                    )
                } else {
                    successor = 0_i32 as CPpmd_Void_Ref
                }
                SetSuccessor(s, successor);
                if successor != 0 || order <= 9_i32 as libc::c_uint {
                    /* O_BOUND */
                    return (ctx as *mut Byte).offset_from(self.base) as libc::c_long as UInt32;
                }
            }
            self.SpecialFreeUnit(ctx as *mut libc::c_void);
            return 0_i32 as CPpmd_Void_Ref;
        }
        nu = (ns as libc::c_uint).wrapping_add(2_i32 as libc::c_uint) >> 1_i32;
        // ctx->union4.stats = STATS_REF(MoveUnitsUp(p, STATS(ctx), nu));
        let indx: libc::c_uint = self.units2indx
            [(nu as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
            as libc::c_uint;
        stats =
            self.base.offset((*ctx).union4.stats as isize) as *mut libc::c_void as *mut CPpmd_State;
        if (stats as *mut Byte).offset_from(self.units_start) as libc::c_long as UInt32
            <= (1_i32 << 14_i32) as libc::c_uint
            && (*ctx).union4.stats <= self.free_list[indx as usize]
        {
            let ptr: *mut libc::c_void = self.RemoveNode(indx);
            (*ctx).union4.stats =
                (ptr as *mut Byte).offset_from(self.base) as libc::c_long as UInt32;
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
            if stats as *mut Byte != self.units_start {
                self.InsertNode(stats as *mut libc::c_void, indx);
            } else {
                self.units_start = self.units_start.offset(
                    (self.indx2units[indx as usize] as libc::c_uint)
                        .wrapping_mul(12_i32 as libc::c_uint) as isize,
                )
            }
            stats = ptr as *mut CPpmd_State
        }
        let mut s_0: *mut CPpmd_State = stats.offset(ns as libc::c_uint as isize);
        loop {
            let successor_0: CPpmd_Void_Ref =
                (*s_0).successor_0 as libc::c_uint | ((*s_0).successor_1 as UInt32) << 16_i32;
            if (self.base.offset(successor_0 as isize) as *mut libc::c_void as *mut Byte)
                < self.units_start
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
            } else if order < self.max_order {
                SetSuccessor(
                    s_0,
                    self.CutOff(
                        self.base.offset(successor_0 as isize) as *mut libc::c_void
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
        if ns != (*ctx).num_stats as libc::c_int && order != 0 {
            if ns < 0_i32 {
                self.FreeUnits(stats as *mut libc::c_void, nu);
                self.SpecialFreeUnit(ctx as *mut libc::c_void);
                return 0_i32 as CPpmd_Void_Ref;
            }
            (*ctx).num_stats = ns as Byte;
            if ns == 0_i32 {
                let sym: Byte = (*stats).symbol;
                (*ctx).flags = (((*ctx).flags as libc::c_int & 1_i32 << 4_i32) as libc::c_uint)
                    .wrapping_add(
                        (sym as libc::c_uint).wrapping_add(0xc0_i32 as libc::c_uint)
                            >> (8_i32 - 3_i32)
                            & (1_i32 << 3_i32) as libc::c_uint,
                    ) as Byte;
                // *ONE_STATE(ctx) = *stats;
                (*ctx).union2.state2.symbol = sym;
                (*ctx).union2.state2.freq = (((*stats).freq as libc::c_uint)
                    .wrapping_add(11_i32 as libc::c_uint)
                    >> 3_i32) as Byte;
                (*ctx).union4.state4.successor_0 = (*stats).successor_0;
                (*ctx).union4.state4.successor_1 = (*stats).successor_1;
                self.FreeUnits(stats as *mut libc::c_void, nu);
            } else {
                self.Refresh(
                    ctx,
                    nu,
                    ((*ctx).union2.summ_freq as libc::c_uint
                        > (16_i32 as libc::c_uint).wrapping_mul(ns as libc::c_uint))
                        as libc::c_int as libc::c_uint,
                );
            }
        }
        (ctx as *mut Byte).offset_from(self.base) as libc::c_long as UInt32
    }
    unsafe fn GetUsedMemory(&mut self) -> UInt32 {
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
                self.stamps[i as usize].wrapping_mul(self.indx2units[i as usize] as libc::c_uint),
            ) as UInt32 as UInt32;
            i = i.wrapping_add(1)
        }
        self.size
            .wrapping_sub(self.hi_unit.offset_from(self.lo_unit) as libc::c_long as UInt32)
            .wrapping_sub(self.units_start.offset_from(self.text) as libc::c_long as UInt32)
            .wrapping_sub(v.wrapping_mul(12_i32 as libc::c_uint))
    }
    unsafe fn RestoreModel(&mut self, ctxError: CTX_PTR) {
        let mut c: CTX_PTR = std::ptr::null_mut::<CPpmd8_Context>();
        let mut s: *mut CPpmd_State = std::ptr::null_mut::<CPpmd_State>();
        self.text = self
            .base
            .offset(self.align_offset as isize)
            .offset(0_i32 as isize);
        // we go here in cases of error of allocation for context (c1)
        // Order(min_context) < Order(ctxError) <= Order(max_context)
        // We remove last symbol from each of contexts [p->max_context ... ctxError) contexts
        // So we rollback all created (symbols) before error.
        c = self.max_context;
        while c != ctxError {
            (*c).num_stats = (*c).num_stats.wrapping_sub(1);
            if (*c).num_stats as libc::c_int == 0_i32 {
                s = self.base.offset((*c).union4.stats as isize) as *mut libc::c_void
                    as *mut CPpmd_State;
                (*c).flags = (((*c).flags as libc::c_int & 1_i32 << 4_i32) as libc::c_uint)
                    .wrapping_add(
                        ((*s).symbol as libc::c_uint).wrapping_add(0xc0_i32 as libc::c_uint)
                            >> (8_i32 - 3_i32)
                            & (1_i32 << 3_i32) as libc::c_uint,
                    ) as Byte;
                // *ONE_STATE(c) = *s;
                (*c).union2.state2.symbol = (*s).symbol;
                (*c).union2.state2.freq = (((*s).freq as libc::c_uint)
                    .wrapping_add(11_i32 as libc::c_uint)
                    >> 3_i32) as Byte;
                (*c).union4.state4.successor_0 = (*s).successor_0;
                (*c).union4.state4.successor_1 = (*s).successor_1;
                self.SpecialFreeUnit(s as *mut libc::c_void);
            } else {
                /* self.Refresh() can increase Escape_Freq on value of freq of last symbol, that was added before error.
                so the largest possible increase for Escape_Freq is (8) from value before ModelUpoadet() */
                self.Refresh(
                    c,
                    ((*c).num_stats as libc::c_uint).wrapping_add(3_i32 as libc::c_uint) >> 1_i32,
                    0_i32 as libc::c_uint,
                );
            }
            c = self.base.offset((*c).suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context
        }
        // increase Escape freq for context [ctxError ... p->min_context)
        while c != self.min_context {
            if (*c).num_stats as libc::c_int == 0_i32 {
                // ONE_STATE(c)
                (*c).union2.state2.freq = (((*c).union2.state2.freq as libc::c_uint)
                    .wrapping_add(1_i32 as libc::c_uint)
                    >> 1_i32) as Byte
            } else {
                (*c).union2.summ_freq = ((*c).union2.summ_freq as libc::c_int + 4_i32) as UInt16; /* fixed over Shkarin's code. Maybe it could work without + 1 too. */
                if (*c).union2.summ_freq as libc::c_int
                    > 128_i32 + 4_i32 * (*c).num_stats as libc::c_int
                {
                    self.Refresh(
                        c,
                        ((*c).num_stats as libc::c_uint).wrapping_add(2_i32 as libc::c_uint)
                            >> 1_i32,
                        1_i32 as libc::c_uint,
                    );
                }
            }
            c = self.base.offset((*c).suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context
        }
        if self.restore_method == PPMD8_RESTORE_METHOD_RESTART as libc::c_int as libc::c_uint
            || self.GetUsedMemory() < self.size >> 1_i32
        {
            self.RestartModel();
        } else {
            while (*self.max_context).suffix != 0 {
                self.max_context = self.base.offset((*self.max_context).suffix as isize)
                    as *mut libc::c_void as *mut CPpmd8_Context
            }
            loop {
                self.CutOff(self.max_context, 0_i32 as libc::c_uint);
                self.ExpandTextArea();
                if self.GetUsedMemory() <= (3_i32 as libc::c_uint).wrapping_mul(self.size >> 2_i32)
                {
                    break;
                }
            }
            self.glue_count = 0_i32 as UInt32;
            self.order_fall = self.max_order
        }
        self.min_context = self.max_context;
    }
    #[inline(never)]
    unsafe fn CreateSuccessors(
        &mut self,
        skip: BoolInt,
        mut s1: *mut CPpmd_State,
        mut c: CTX_PTR,
    ) -> CTX_PTR {
        let mut upBranch: CPpmd_Byte_Ref = (*self.found_state).successor_0 as libc::c_uint
            | ((*self.found_state).successor_1 as UInt32) << 16_i32;
        let mut newSym: Byte = 0;
        let mut newFreq: Byte = 0;
        let mut flags: Byte = 0;
        let mut numPs: libc::c_uint = 0_i32 as libc::c_uint;
        let mut ps: [*mut CPpmd_State; 17] = [std::ptr::null_mut::<CPpmd_State>(); 17];
        if skip == 0 {
            let fresh2 = numPs;
            numPs = numPs.wrapping_add(1);
            ps[fresh2 as usize] = self.found_state
        }
        while (*c).suffix != 0 {
            let mut successor: CPpmd_Void_Ref = 0;
            let mut s: *mut CPpmd_State = std::ptr::null_mut::<CPpmd_State>();
            c = self.base.offset((*c).suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context;
            if !s1.is_null() {
                s = s1;
                s1 = std::ptr::null_mut::<CPpmd_State>()
            } else if (*c).num_stats as libc::c_int != 0_i32 {
                let sym: Byte = (*self.found_state).symbol;
                s = self.base.offset((*c).union4.stats as isize) as *mut libc::c_void
                    as *mut CPpmd_State;
                while (*s).symbol as libc::c_int != sym as libc::c_int {
                    s = s.offset(1)
                }
                if ((*s).freq as libc::c_int) < 124_i32 - 9_i32 {
                    (*s).freq = (*s).freq.wrapping_add(1);
                    (*c).union2.summ_freq = (*c).union2.summ_freq.wrapping_add(1)
                }
            } else {
                s = &mut (*c).union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State;
                (*s).freq = ((*s).freq as libc::c_int
                    + (((*(self.base.offset((*c).suffix as isize) as *mut libc::c_void
                        as *mut CPpmd8_Context))
                        .num_stats
                        == 0) as libc::c_int
                        & (((*s).freq as libc::c_int) < 24_i32) as libc::c_int))
                    as Byte
            }
            successor = (*s).successor_0 as libc::c_uint | ((*s).successor_1 as UInt32) << 16_i32;
            if successor != upBranch {
                c = self.base.offset(successor as isize) as *mut libc::c_void
                    as *mut CPpmd8_Context;
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
        newSym = *(self.base.offset(upBranch as isize) as *mut libc::c_void as *const Byte);
        upBranch = upBranch.wrapping_add(1);
        flags = (((*self.found_state).symbol as libc::c_uint)
            .wrapping_add(0xc0_i32 as libc::c_uint)
            >> (8_i32 - 4_i32)
            & (1_i32 << 4_i32) as libc::c_uint)
            .wrapping_add(
                (newSym as libc::c_uint).wrapping_add(0xc0_i32 as libc::c_uint) >> (8_i32 - 3_i32)
                    & (1_i32 << 3_i32) as libc::c_uint,
            ) as Byte;
        if (*c).num_stats as libc::c_int == 0_i32 {
            newFreq = (*c).union2.state2.freq
        } else {
            let mut cf: UInt32 = 0;
            let mut s0: UInt32 = 0;
            let mut s_0: *mut CPpmd_State = std::ptr::null_mut::<CPpmd_State>();
            s_0 = self.base.offset((*c).union4.stats as isize) as *mut libc::c_void
                as *mut CPpmd_State;
            while (*s_0).symbol as libc::c_int != newSym as libc::c_int {
                s_0 = s_0.offset(1)
            }
            cf = ((*s_0).freq as UInt32).wrapping_sub(1_i32 as libc::c_uint);
            s0 = ((*c).union2.summ_freq as UInt32)
                .wrapping_sub((*c).num_stats as libc::c_uint)
                .wrapping_sub(cf);
            /*


              max(newFreq)= (s->freq - 1), when (s0 == 1)


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
            if self.hi_unit != self.lo_unit {
                self.hi_unit = self.hi_unit.offset(-(12_i32 as isize));
                c1 = self.hi_unit as *mut libc::c_void as CTX_PTR
            } else if self.free_list[0_i32 as usize] != 0_i32 as libc::c_uint {
                c1 = self.RemoveNode(0_i32 as libc::c_uint) as CTX_PTR
            } else {
                c1 = self.AllocUnitsRare(0_i32 as libc::c_uint) as CTX_PTR;
                if c1.is_null() {
                    return 0 as CTX_PTR;
                }
            }
            (*c1).flags = flags;
            (*c1).num_stats = 0_i32 as Byte;
            (*c1).union2.state2.symbol = newSym;
            (*c1).union2.state2.freq = newFreq;
            SetSuccessor(
                &mut (*c1).union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State,
                upBranch,
            );
            (*c1).suffix = (c as *mut Byte).offset_from(self.base) as libc::c_long as UInt32;
            numPs = numPs.wrapping_sub(1);
            SetSuccessor(
                ps[numPs as usize],
                (c1 as *mut Byte).offset_from(self.base) as libc::c_long as UInt32,
            );
            c = c1;
            if numPs == 0_i32 as libc::c_uint {
                break;
            }
        }
        c
    }
    unsafe fn ReduceOrder(&mut self, mut s1: *mut CPpmd_State, mut c: CTX_PTR) -> CTX_PTR {
        let mut s: *mut CPpmd_State = std::ptr::null_mut::<CPpmd_State>();
        let c1: CTX_PTR = c;
        let upBranch: CPpmd_Void_Ref = self.text.offset_from(self.base) as libc::c_long as UInt32;
        SetSuccessor(self.found_state, upBranch);
        self.order_fall = self.order_fall.wrapping_add(1);
        loop {
            if !s1.is_null() {
                c = self.base.offset((*c).suffix as isize) as *mut libc::c_void
                    as *mut CPpmd8_Context;
                s = s1;
                s1 = std::ptr::null_mut::<CPpmd_State>()
            } else {
                if (*c).suffix == 0 {
                    return c;
                }
                c = self.base.offset((*c).suffix as isize) as *mut libc::c_void
                    as *mut CPpmd8_Context;
                if (*c).num_stats != 0 {
                    s = self.base.offset((*c).union4.stats as isize) as *mut libc::c_void
                        as *mut CPpmd_State;
                    if (*s).symbol as libc::c_int != (*self.found_state).symbol as libc::c_int {
                        loop {
                            s = s.offset(1);
                            if (*s).symbol as libc::c_int
                                == (*self.found_state).symbol as libc::c_int
                            {
                                break;
                            }
                        }
                    }
                    if ((*s).freq as libc::c_int) < 124_i32 - 9_i32 {
                        (*s).freq = ((*s).freq as libc::c_int + 2_i32) as Byte;
                        (*c).union2.summ_freq =
                            ((*c).union2.summ_freq as libc::c_int + 2_i32) as UInt16
                    }
                } else {
                    s = &mut (*c).union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State;
                    (*s).freq = ((*s).freq as libc::c_int
                        + (((*s).freq as libc::c_int) < 32_i32) as libc::c_int)
                        as Byte
                }
            }
            if (*s).successor_0 as libc::c_uint | ((*s).successor_1 as UInt32) << 16_i32 != 0 {
                break;
            }
            SetSuccessor(s, upBranch);
            self.order_fall = self.order_fall.wrapping_add(1)
        }
        if (*s).successor_0 as libc::c_uint | ((*s).successor_1 as UInt32) << 16_i32 <= upBranch {
            let mut successor: CTX_PTR = std::ptr::null_mut::<CPpmd8_Context>();
            let s2: *mut CPpmd_State = self.found_state;
            self.found_state = s;
            successor = self.CreateSuccessors(0_i32, std::ptr::null_mut::<CPpmd_State>(), c);
            if successor.is_null() {
                SetSuccessor(s, 0_i32 as CPpmd_Void_Ref);
            } else {
                SetSuccessor(
                    s,
                    (successor as *mut Byte).offset_from(self.base) as libc::c_long as UInt32,
                );
            }
            self.found_state = s2
        }
        let successor_0: CPpmd_Void_Ref =
            (*s).successor_0 as libc::c_uint | ((*s).successor_1 as UInt32) << 16_i32;
        if self.order_fall == 1_i32 as libc::c_uint && c1 == self.max_context {
            SetSuccessor(self.found_state, successor_0);
            self.text = self.text.offset(-1)
        }
        if successor_0 == 0_i32 as libc::c_uint {
            return 0 as CTX_PTR;
        }
        self.base.offset(successor_0 as isize) as *mut libc::c_void as *mut CPpmd8_Context
    }

    #[inline(never)]
    pub unsafe fn Ppmd8_UpdateModel(&mut self) {
        let mut maxSuccessor: CPpmd_Void_Ref = 0;
        let mut minSuccessor: CPpmd_Void_Ref = (*self.found_state).successor_0 as libc::c_uint
            | ((*self.found_state).successor_1 as UInt32) << 16_i32;
        let mut c: CTX_PTR = std::ptr::null_mut::<CPpmd8_Context>();
        let mut s0: libc::c_uint = 0;
        let mut ns: libc::c_uint = 0;
        let fFreq: libc::c_uint = (*self.found_state).freq as libc::c_uint;
        let mut flag: Byte = 0;
        let fSymbol: Byte = (*self.found_state).symbol;
        let mut s: *mut CPpmd_State = std::ptr::null_mut::<CPpmd_State>();
        if ((*self.found_state).freq as libc::c_int) < 124_i32 / 4_i32
            && (*self.min_context).suffix != 0_i32 as libc::c_uint
        {
            /* Update Freqs in suffix Context */
            c = self.base.offset((*self.min_context).suffix as isize) as *mut libc::c_void
                as *mut CPpmd8_Context; /* check it */
            if (*c).num_stats as libc::c_int == 0_i32 {
                s = &mut (*c).union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State;
                if ((*s).freq as libc::c_int) < 32_i32 {
                    (*s).freq = (*s).freq.wrapping_add(1)
                }
            } else {
                let sym: Byte = (*self.found_state).symbol;
                s = self.base.offset((*c).union4.stats as isize) as *mut libc::c_void
                    as *mut CPpmd_State;
                if (*s).symbol as libc::c_int != sym as libc::c_int {
                    loop {
                        s = s.offset(1);
                        if (*s).symbol as libc::c_int == sym as libc::c_int {
                            break;
                        }
                    }
                    if (*s.offset(0_i32 as isize)).freq as libc::c_int
                        >= (*s.offset(-1_i32 as isize)).freq as libc::c_int
                    {
                        SwapStates(
                            &mut *s.offset(0_i32 as isize),
                            &mut *s.offset(-1_i32 as isize),
                        );
                        s = s.offset(-1)
                    }
                }
                if ((*s).freq as libc::c_int) < 124_i32 - 9_i32 {
                    (*s).freq = ((*s).freq as libc::c_int + 2_i32) as Byte;
                    (*c).union2.summ_freq = ((*c).union2.summ_freq as libc::c_int + 2_i32) as UInt16
                }
            }
        }
        c = self.max_context;
        if self.order_fall == 0_i32 as libc::c_uint && minSuccessor != 0 {
            let cs: CTX_PTR = self.CreateSuccessors(1_i32, s, self.min_context);
            if cs.is_null() {
                SetSuccessor(self.found_state, 0_i32 as CPpmd_Void_Ref);
                self.RestoreModel(c);
                return;
            }
            SetSuccessor(
                self.found_state,
                (cs as *mut Byte).offset_from(self.base) as libc::c_long as UInt32,
            );
            self.max_context = cs;
            self.min_context = self.max_context;
            return;
        }
        let mut text: *mut Byte = self.text;
        let fresh4 = text;
        text = text.offset(1);
        *fresh4 = (*self.found_state).symbol;
        self.text = text;
        if text >= self.units_start {
            self.RestoreModel(c);
            return;
        }
        maxSuccessor = text.offset_from(self.base) as libc::c_long as UInt32;
        if minSuccessor == 0 {
            let cs_0: CTX_PTR = self.ReduceOrder(s, self.min_context);
            if cs_0.is_null() {
                self.RestoreModel(c);
                return;
            }
            minSuccessor = (cs_0 as *mut Byte).offset_from(self.base) as libc::c_long as UInt32
        } else if (self.base.offset(minSuccessor as isize) as *mut libc::c_void as *mut Byte)
            < self.units_start
        {
            let cs_1: CTX_PTR = self.CreateSuccessors(0_i32, s, self.min_context);
            if cs_1.is_null() {
                self.RestoreModel(c);
                return;
            }
            minSuccessor = (cs_1 as *mut Byte).offset_from(self.base) as libc::c_long as UInt32
        }
        self.order_fall = self.order_fall.wrapping_sub(1);
        if self.order_fall == 0_i32 as libc::c_uint {
            maxSuccessor = minSuccessor;
            self.text = self
                .text
                .offset(-((self.max_context != self.min_context) as libc::c_int as isize))
        }
        flag = ((fSymbol as libc::c_uint).wrapping_add(0xc0_i32 as libc::c_uint) >> (8_i32 - 3_i32)
            & (1_i32 << 3_i32) as libc::c_uint) as Byte;
        ns = (*self.min_context).num_stats as libc::c_uint;
        s0 = ((*self.min_context).union2.summ_freq as libc::c_uint)
            .wrapping_sub(ns)
            .wrapping_sub(fFreq);
        while c != self.min_context {
            let mut ns1: libc::c_uint = 0;
            let mut sum: UInt32 = 0;
            ns1 = (*c).num_stats as libc::c_uint;
            if ns1 != 0_i32 as libc::c_uint {
                if ns1 & 1_i32 as libc::c_uint != 0_i32 as libc::c_uint {
                    /* Expand for one UNIT */
                    let oldNU: libc::c_uint = ns1.wrapping_add(1_i32 as libc::c_uint) >> 1_i32;
                    let i: libc::c_uint = self.units2indx
                        [(oldNU as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
                        as libc::c_uint;
                    if i != self.units2indx[(oldNU as size_t)
                        .wrapping_add(1_i32 as libc::c_ulong)
                        .wrapping_sub(1_i32 as libc::c_ulong)
                        as usize] as libc::c_uint
                    {
                        let ptr: *mut libc::c_void =
                            self.AllocUnits(i.wrapping_add(1_i32 as libc::c_uint));
                        let mut oldPtr: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
                        if ptr.is_null() {
                            self.RestoreModel(c);
                            return;
                        }
                        oldPtr = self.base.offset((*c).union4.stats as isize) as *mut libc::c_void
                            as *mut CPpmd_State
                            as *mut libc::c_void;
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
                        self.InsertNode(oldPtr, i);
                        (*c).union4.stats =
                            (ptr as *mut Byte).offset_from(self.base) as libc::c_long as UInt32
                    }
                }
                sum = (*c).union2.summ_freq as UInt32;
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
                    self.AllocUnits(0_i32 as libc::c_uint) as *mut CPpmd_State;
                if s_0.is_null() {
                    self.RestoreModel(c);
                    return;
                }
                let mut freq: libc::c_uint = (*c).union2.state2.freq as libc::c_uint;
                /* max increase of Escape_Freq is 1 here.
                an average increase is 1/3 per symbol */
                // Ppmd8 (> 2)
                (*s_0).symbol = (*c).union2.state2.symbol;
                (*s_0).successor_0 = (*c).union4.state4.successor_0;
                (*s_0).successor_1 = (*c).union4.state4.successor_1;
                (*c).union4.stats =
                    (s_0 as *mut Byte).offset_from(self.base) as libc::c_long as UInt32;
                if freq < (124_i32 / 4_i32 - 1_i32) as libc::c_uint {
                    freq <<= 1_i32
                } else {
                    freq = (124_i32 - 4_i32) as libc::c_uint
                }
                (*s_0).freq = freq as Byte;
                sum = freq
                    .wrapping_add(self.init_esc)
                    .wrapping_add((ns > 2_i32 as libc::c_uint) as libc::c_int as libc::c_uint)
            }
            let mut s_1: *mut CPpmd_State = (self.base.offset((*c).union4.stats as isize)
                as *mut libc::c_void
                as *mut CPpmd_State)
                .offset(ns1 as isize)
                .offset(1_i32 as isize);
            let mut cf: UInt32 = (2_i32 as libc::c_uint)
                .wrapping_mul(sum.wrapping_add(6_i32 as libc::c_uint))
                .wrapping_mul(fFreq);
            let sf: UInt32 = s0.wrapping_add(sum);
            (*s_1).symbol = fSymbol;
            (*c).num_stats = ns1.wrapping_add(1_i32 as libc::c_uint) as Byte;
            SetSuccessor(s_1, maxSuccessor);
            (*c).flags = ((*c).flags as libc::c_int | flag as libc::c_int) as Byte;
            if cf < (6_i32 as libc::c_uint).wrapping_mul(sf) {
                cf = (1_i32 as libc::c_uint)
                    .wrapping_add((cf > sf) as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (cf >= (4_i32 as libc::c_uint).wrapping_mul(sf)) as libc::c_int
                            as libc::c_uint,
                    );
                sum = (sum as libc::c_uint).wrapping_add(4_i32 as libc::c_uint) as UInt32 as UInt32
                // s = *ONE_STATE(c);
                // SetSuccessor(s, c->union4.stats);  // call it only for debug purposes to check the order of
                // (successor_0 and successor_1) in LE/BE.
                /* It can add (1, 2, 3) to Escape_Freq */
            } else {
                cf = (4_i32 as libc::c_uint)
                    .wrapping_add(
                        (cf > (9_i32 as libc::c_uint).wrapping_mul(sf)) as libc::c_int
                            as libc::c_uint,
                    )
                    .wrapping_add(
                        (cf > (12_i32 as libc::c_uint).wrapping_mul(sf)) as libc::c_int
                            as libc::c_uint,
                    )
                    .wrapping_add(
                        (cf > (15_i32 as libc::c_uint).wrapping_mul(sf)) as libc::c_int
                            as libc::c_uint,
                    );
                sum = (sum as libc::c_uint).wrapping_add(cf) as UInt32 as UInt32
            }
            (*c).union2.summ_freq = sum as UInt16;
            (*s_1).freq = cf as Byte;
            c = self.base.offset((*c).suffix as isize) as *mut libc::c_void as *mut CPpmd8_Context
        }
        self.min_context =
            self.base.offset(minSuccessor as isize) as *mut libc::c_void as *mut CPpmd8_Context;
        self.max_context = self.min_context;
    }
    #[inline(never)]
    unsafe fn rescale(&mut self) {
        let mut i: libc::c_uint = 0;
        let mut adder: libc::c_uint = 0;
        let mut sumFreq: libc::c_uint = 0;
        let mut escFreq: libc::c_uint = 0;
        let stats: *mut CPpmd_State = self.base.offset((*self.min_context).union4.stats as isize)
            as *mut libc::c_void as *mut CPpmd_State;
        let mut s: *mut CPpmd_State = self.found_state;
        /* Sort the list by freq */
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
        sumFreq = (*s).freq as libc::c_uint;
        escFreq = ((*self.min_context).union2.summ_freq as libc::c_uint).wrapping_sub(sumFreq);
        adder = (self.order_fall != 0_i32 as libc::c_uint) as libc::c_int as libc::c_uint;
        sumFreq = sumFreq
            .wrapping_add(4_i32 as libc::c_uint)
            .wrapping_add(adder)
            >> 1_i32;
        i = (*self.min_context).num_stats as libc::c_uint;
        (*s).freq = sumFreq as Byte;
        loop {
            s = s.offset(1);
            let mut freq: libc::c_uint = (*s).freq as libc::c_uint;
            escFreq = escFreq.wrapping_sub(freq);
            freq = freq.wrapping_add(adder) >> 1_i32;
            sumFreq = sumFreq.wrapping_add(freq);
            (*s).freq = freq as Byte;
            if freq > (*s.offset(-1_i32 as isize)).freq as libc::c_uint {
                let tmp_0: CPpmd_State = *s;
                let mut s1: *mut CPpmd_State = s;
                loop {
                    *s1.offset(0_i32 as isize) = *s1.offset(-1_i32 as isize);
                    s1 = s1.offset(-1);
                    if !(s1 != stats && freq > (*s1.offset(-1_i32 as isize)).freq as libc::c_uint) {
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
        if (*s).freq as libc::c_int == 0_i32 {
            /* Remove all items with freq == 0 */
            let mut mc: *mut CPpmd8_Context = std::ptr::null_mut::<CPpmd8_Context>();
            let mut numStats: libc::c_uint = 0;
            let mut numStatsNew: libc::c_uint = 0;
            let mut n0: libc::c_uint = 0;
            let mut n1: libc::c_uint = 0;
            i = 0_i32 as libc::c_uint;
            loop {
                i = i.wrapping_add(1);
                s = s.offset(-1);
                if (*s).freq as libc::c_int != 0_i32 {
                    break;
                }
            }
            escFreq = escFreq.wrapping_add(i);
            mc = self.min_context;
            numStats = (*mc).num_stats as libc::c_uint;
            numStatsNew = numStats.wrapping_sub(i);
            (*mc).num_stats = numStatsNew as Byte;
            n0 = numStats.wrapping_add(2_i32 as libc::c_uint) >> 1_i32;
            if numStatsNew == 0_i32 as libc::c_uint {
                let mut freq_0: libc::c_uint = (2_i32 as libc::c_uint)
                    .wrapping_mul((*stats).freq as libc::c_uint)
                    .wrapping_add(escFreq)
                    .wrapping_sub(1_i32 as libc::c_uint)
                    .wrapping_div(escFreq);
                if freq_0 > (124_i32 / 3_i32) as libc::c_uint {
                    freq_0 = (124_i32 / 3_i32) as libc::c_uint
                }
                (*mc).flags = (((*mc).flags as libc::c_int & 1_i32 << 4_i32) as libc::c_uint)
                    .wrapping_add(
                        ((*stats).symbol as libc::c_uint).wrapping_add(0xc0_i32 as libc::c_uint)
                            >> (8_i32 - 3_i32)
                            & (1_i32 << 3_i32) as libc::c_uint,
                    ) as Byte;
                s = &mut (*mc).union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State;
                *s = *stats;
                (*s).freq = freq_0 as Byte;
                self.found_state = s;
                self.InsertNode(
                    stats as *mut libc::c_void,
                    self.units2indx[(n0 as size_t).wrapping_sub(1_i32 as libc::c_ulong) as usize]
                        as libc::c_uint,
                );
                return;
            }
            n1 = numStatsNew.wrapping_add(2_i32 as libc::c_uint) >> 1_i32;
            if n0 != n1 {
                (*mc).union4.stats =
                    (self.ShrinkUnits(stats as *mut libc::c_void, n0, n1) as *mut Byte)
                        .offset_from(self.base) as libc::c_long as UInt32
            }
        }
        let mut mc_0: *mut CPpmd8_Context = self.min_context;
        (*mc_0).union2.summ_freq =
            sumFreq.wrapping_add(escFreq).wrapping_sub(escFreq >> 1_i32) as UInt16;
        (*mc_0).flags = ((*mc_0).flags as libc::c_int | 1_i32 << 2_i32) as Byte;
        self.found_state = self.base.offset((*mc_0).union4.stats as isize) as *mut libc::c_void
            as *mut CPpmd_State;
    }

    pub unsafe fn Ppmd8_MakeEscFreq(
        &mut self,
        numMasked1: libc::c_uint,
        escFreq: *mut UInt32,
    ) -> *mut CPpmd_See {
        let mut see: *mut CPpmd_See = std::ptr::null_mut::<CPpmd_See>();
        let mc: *const CPpmd8_Context = self.min_context;
        let numStats: libc::c_uint = (*mc).num_stats as libc::c_uint;
        if numStats != 0xff_i32 as libc::c_uint {
            // (3 <= numStats + 2 <= 256)   (3 <= ns2indx[3] and ns2indx[256] === 26)
            see = self.see[(self.ns2indx
                [(numStats as size_t).wrapping_add(2_i32 as libc::c_ulong) as usize]
                as libc::c_uint as size_t)
                .wrapping_sub(3_i32 as libc::c_ulong) as usize]
                .as_mut_ptr()
                .offset(
                    ((*mc).union2.summ_freq as libc::c_uint
                        > (11_i32 as libc::c_uint)
                            .wrapping_mul(numStats.wrapping_add(1_i32 as libc::c_uint)))
                        as libc::c_int as isize,
                )
                .offset(
                    (2_i32 as libc::c_uint).wrapping_mul(
                        ((2_i32 as libc::c_uint).wrapping_mul(numStats)
                            < ((*(self.base.offset((*mc).suffix as isize) as *mut libc::c_void
                                as *mut CPpmd8_Context))
                                .num_stats as libc::c_uint)
                                .wrapping_add(numMasked1)) as libc::c_int
                            as libc::c_uint,
                    ) as isize,
                )
                .offset((*mc).flags as libc::c_int as isize);
            // if (see->summ) field is larger than 16-bit, we need only low 16 bits of summ
            let summ: libc::c_uint = (*see).summ as libc::c_uint; // & 0xFFFF
            let r: libc::c_uint = summ >> (*see).shift as libc::c_int; // Ppmd8 (>=)
            (*see).summ = summ.wrapping_sub(r) as UInt16;
            *escFreq = r.wrapping_add((r == 0_i32 as libc::c_uint) as libc::c_int as libc::c_uint)
        } else {
            see = &mut self.dummy_see;
            *escFreq = 1_i32 as UInt32
        }
        see
    }
    unsafe fn NextContext(&mut self) {
        let c: CTX_PTR = self.base.offset(
            ((*self.found_state).successor_0 as libc::c_uint
                | ((*self.found_state).successor_1 as UInt32) << 16_i32) as isize,
        ) as *mut libc::c_void as *mut CPpmd8_Context;
        if self.order_fall == 0_i32 as libc::c_uint && c as *const Byte >= self.units_start {
            self.min_context = c;
            self.max_context = self.min_context
        } else {
            self.Ppmd8_UpdateModel();
        };
    }

    pub unsafe fn Ppmd8_Update1(&mut self) {
        let mut s: *mut CPpmd_State = self.found_state;
        let mut freq: libc::c_uint = (*s).freq as libc::c_uint;
        freq = freq.wrapping_add(4_i32 as libc::c_uint);
        (*self.min_context).union2.summ_freq =
            ((*self.min_context).union2.summ_freq as libc::c_int + 4_i32) as UInt16;
        (*s).freq = freq as Byte;
        if freq > (*s.offset(-1_i32 as isize)).freq as libc::c_uint {
            SwapStates(s, &mut *s.offset(-1_i32 as isize));
            s = s.offset(-1);
            self.found_state = s;
            if freq > 124_i32 as libc::c_uint {
                self.rescale();
            }
        }
        self.NextContext();
    }

    pub unsafe fn Ppmd8_Update1_0(&mut self) {
        let mut s: *mut CPpmd_State = self.found_state;
        let mut mc: *mut CPpmd8_Context = self.min_context;
        let mut freq: libc::c_uint = (*s).freq as libc::c_uint;
        let summFreq: libc::c_uint = (*mc).union2.summ_freq as libc::c_uint;
        self.prev_success =
            ((2_i32 as libc::c_uint).wrapping_mul(freq) >= summFreq) as libc::c_int as libc::c_uint;
        self.run_length += self.prev_success as libc::c_int;
        (*mc).union2.summ_freq = summFreq.wrapping_add(4_i32 as libc::c_uint) as UInt16;
        freq = freq.wrapping_add(4_i32 as libc::c_uint);
        (*s).freq = freq as Byte;
        if freq > 124_i32 as libc::c_uint {
            self.rescale();
        }
        self.NextContext();
    }
    /*
    void Ppmd8_UpdateBin(CPpmd8 *p)
    {
      unsigned freq = p->found_state->freq;
      p->found_state->freq = (Byte)(freq + (freq < 196)); // Ppmd8 (196)
      p->prev_success = 1;
      p->run_length++;
      self.NextContext(p);
    }
    */

    pub unsafe fn Ppmd8_Update2(&mut self) {
        let mut s: *mut CPpmd_State = self.found_state;
        let mut freq: libc::c_uint = (*s).freq as libc::c_uint;
        freq = freq.wrapping_add(4_i32 as libc::c_uint);
        self.run_length = self.init_rl;
        (*self.min_context).union2.summ_freq =
            ((*self.min_context).union2.summ_freq as libc::c_int + 4_i32) as UInt16;
        (*s).freq = freq as Byte;
        if freq > 124_i32 as libc::c_uint {
            self.rescale();
        }
        self.Ppmd8_UpdateModel();
    }
    /* H->I changes:
      ns2indx
      glue_count, and Glue method
      BinSum
      see / EscFreq
      self.CreateSuccessors updates more suffix contexts
      self.Ppmd8_UpdateModel consts.
      prev_success Update

    flags:
      (1 << 2) - the Context was self.rescaled
      (1 << 3) - there is symbol in stats with (sym >= 0x40) in
      (1 << 4) - main symbol of context is (sym >= 0x40)
    */

    /* ---------- Encode ---------- */

    pub unsafe fn Ppmd8_Flush_RangeEnc(&mut self) {
        let mut i: libc::c_uint = 0;
        i = 0_i32 as libc::c_uint;
        while i < 4_i32 as libc::c_uint {
            (*self.stream.out).write.expect("non-null function pointer")(
                self.stream.out,
                (self.low >> 24_i32) as Byte,
            );
            i = i.wrapping_add(1);
            self.low <<= 8_i32
        }
    }
    // MY_NO_INLINE
    unsafe fn RangeEnc_Encode(&mut self, start: UInt32, size: UInt32, total: UInt32) {
        self.range = (self.range as libc::c_uint).wrapping_div(total) as UInt32 as UInt32;
        self.low = (self.low as libc::c_uint).wrapping_add(start.wrapping_mul(self.range)) as UInt32
            as UInt32;
        self.range = (self.range as libc::c_uint).wrapping_mul(size) as UInt32 as UInt32;
    }
    // MY_FORCE_INLINE
    // static

    pub unsafe fn Ppmd8_EncodeSymbol(&mut self, symbol: libc::c_int) {
        let mut charMask: [size_t; 32] = [0; 32];
        if (*self.min_context).num_stats as libc::c_int != 0_i32 {
            let mut s: *mut CPpmd_State =
                self.base.offset((*self.min_context).union4.stats as isize) as *mut libc::c_void
                    as *mut CPpmd_State;
            let mut sum: UInt32 = 0;
            let mut i: libc::c_uint = 0;
            let mut summFreq: UInt32 = (*self.min_context).union2.summ_freq as UInt32;
            if summFreq > self.range {
                summFreq = self.range
            }
            // RC_PRE(summFreq);
            if (*s).symbol as libc::c_int == symbol {
                self.RangeEnc_Encode(0_i32 as UInt32, (*s).freq as UInt32, summFreq);
                while self.low ^ self.low.wrapping_add(self.range)
                    < (1_i32 << 24_i32) as libc::c_uint
                    || self.range < (1_i32 << 15_i32) as libc::c_uint && {
                        self.range = (0_i32 as libc::c_uint).wrapping_sub(self.low)
                            & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                        1_i32 != 0
                    }
                {
                    (*self.stream.out).write.expect("non-null function pointer")(
                        self.stream.out,
                        (self.low >> 24_i32) as Byte,
                    );
                    self.range <<= 8_i32;
                    self.low <<= 8_i32
                }
                self.found_state = s;
                self.Ppmd8_Update1_0();
                return;
            }
            self.prev_success = 0_i32 as libc::c_uint;
            sum = (*s).freq as UInt32;
            i = (*self.min_context).num_stats as libc::c_uint;
            loop {
                s = s.offset(1);
                if (*s).symbol as libc::c_int == symbol {
                    self.RangeEnc_Encode(sum, (*s).freq as UInt32, summFreq);
                    while self.low ^ self.low.wrapping_add(self.range)
                        < (1_i32 << 24_i32) as libc::c_uint
                        || self.range < (1_i32 << 15_i32) as libc::c_uint && {
                            self.range = (0_i32 as libc::c_uint).wrapping_sub(self.low)
                                & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                            1_i32 != 0
                        }
                    {
                        (*self.stream.out).write.expect("non-null function pointer")(
                            self.stream.out,
                            (self.low >> 24_i32) as Byte,
                        );
                        self.range <<= 8_i32;
                        self.low <<= 8_i32
                    }
                    self.found_state = s;
                    self.Ppmd8_Update1();
                    return;
                }
                sum = (sum as libc::c_uint).wrapping_add((*s).freq as libc::c_uint) as UInt32
                    as UInt32;
                i = i.wrapping_sub(1);
                if i == 0 {
                    break;
                }
            }
            self.RangeEnc_Encode(sum, summFreq.wrapping_sub(sum), summFreq);
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
            // MASK(s->symbol) = 0;
            // i = p->min_context->num_stats;
            // do { MASK((--s)->symbol) = 0; } while (--i);
            let mut s2: *mut CPpmd_State =
                self.base.offset((*self.min_context).union4.stats as isize) as *mut libc::c_void
                    as *mut CPpmd_State;
            *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset((*s).symbol as isize) =
                0_i32 as libc::c_uchar;
            loop {
                let sym0: libc::c_uint = (*s2.offset(0_i32 as isize)).symbol as libc::c_uint;
                let sym1: libc::c_uint = (*s2.offset(1_i32 as isize)).symbol as libc::c_uint;
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
            let prob: *mut UInt16 = &mut *(*self.bin_summ.as_mut_ptr().offset(
                *self.ns2indx.as_mut_ptr().offset(
                    ((*(&mut (*self.min_context).union2 as *mut C2RustUnnamed_0
                        as *mut CPpmd_State))
                        .freq as size_t)
                        .wrapping_sub(1_i32 as libc::c_ulong) as isize,
                ) as isize,
            ))
            .as_mut_ptr()
            .offset(
                self.prev_success
                    .wrapping_add((self.run_length >> 26_i32 & 0x20_i32) as libc::c_uint)
                    .wrapping_add(
                        *self.ns2bsindx.as_mut_ptr().offset(
                            (*(self.base.offset((*self.min_context).suffix as isize)
                                as *mut libc::c_void
                                as *mut CPpmd8_Context))
                                .num_stats as isize,
                        ) as libc::c_uint,
                    )
                    .wrapping_add((*self.min_context).flags as libc::c_int as libc::c_uint)
                    as isize,
            ) as *mut UInt16;
            let mut s_0: *mut CPpmd_State =
                &mut (*self.min_context).union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State;
            let mut pr: UInt32 = *prob as UInt32;
            let bound: UInt32 = (self.range >> 14_i32).wrapping_mul(pr);
            pr = pr
                .wrapping_sub(pr.wrapping_add((1_i32 << (7_i32 - 2_i32)) as libc::c_uint) >> 7_i32);
            if (*s_0).symbol as libc::c_int == symbol {
                *prob = pr.wrapping_add((1_i32 << 7_i32) as libc::c_uint) as UInt16;
                // self.RangeEnc_EncodeBit_0(p, bound);
                self.range = bound;
                while self.low ^ self.low.wrapping_add(self.range)
                    < (1_i32 << 24_i32) as libc::c_uint
                    || self.range < (1_i32 << 15_i32) as libc::c_uint && {
                        self.range = (0_i32 as libc::c_uint).wrapping_sub(self.low)
                            & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                        1_i32 != 0
                    }
                {
                    (*self.stream.out).write.expect("non-null function pointer")(
                        self.stream.out,
                        (self.low >> 24_i32) as Byte,
                    );
                    self.range <<= 8_i32;
                    self.low <<= 8_i32
                }
                // p->found_state = s;
                // Ppmd8_UpdateBin(p);
                let freq: libc::c_uint = (*s_0).freq as libc::c_uint; // Ppmd8 (196)
                let c: CTX_PTR = self.base.offset(
                    ((*s_0).successor_0 as libc::c_uint | ((*s_0).successor_1 as UInt32) << 16_i32)
                        as isize,
                ) as *mut libc::c_void as *mut CPpmd8_Context;
                self.found_state = s_0;
                self.prev_success = 1_i32 as libc::c_uint;
                self.run_length += 1;
                (*s_0).freq = freq
                    .wrapping_add((freq < 196_i32 as libc::c_uint) as libc::c_int as libc::c_uint)
                    as Byte;
                // self.NextContext(p);
                if self.order_fall == 0_i32 as libc::c_uint && c as *const Byte >= self.units_start
                {
                    self.min_context = c;
                    self.max_context = self.min_context
                } else {
                    self.Ppmd8_UpdateModel();
                }
                return;
            }
            *prob = pr as UInt16;
            self.init_esc = self.exp_escape[(pr >> 10_i32) as usize] as libc::c_uint;
            // self.RangeEnc_EncodeBit_1(p, bound);
            self.low = (self.low as libc::c_uint).wrapping_add(bound) as UInt32 as UInt32; /* EndMarker (symbol = -1) */
            self.range = (self.range
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
                z_0 =
                    (z_0 as libc::c_ulong).wrapping_add(8_i32 as libc::c_ulong) as size_t as size_t
            }
            *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset((*s_0).symbol as isize) =
                0_i32 as libc::c_uchar;
            self.prev_success = 0_i32 as libc::c_uint
        }
        loop {
            let mut see: *mut CPpmd_See = std::ptr::null_mut::<CPpmd_See>();
            let mut s_1: *mut CPpmd_State = std::ptr::null_mut::<CPpmd_State>();
            let mut sum_0: UInt32 = 0;
            let mut escFreq: UInt32 = 0;
            let mut mc: *mut CPpmd8_Context = std::ptr::null_mut::<CPpmd8_Context>();
            let mut i_0: libc::c_uint = 0;
            let mut numMasked: libc::c_uint = 0;
            while self.low ^ self.low.wrapping_add(self.range) < (1_i32 << 24_i32) as libc::c_uint
                || self.range < (1_i32 << 15_i32) as libc::c_uint && {
                    self.range = (0_i32 as libc::c_uint).wrapping_sub(self.low)
                        & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                    1_i32 != 0
                }
            {
                (*self.stream.out).write.expect("non-null function pointer")(
                    self.stream.out,
                    (self.low >> 24_i32) as Byte,
                );
                self.range <<= 8_i32;
                self.low <<= 8_i32
            }
            mc = self.min_context;
            numMasked = (*mc).num_stats as libc::c_uint;
            loop {
                self.order_fall = self.order_fall.wrapping_add(1);
                if (*mc).suffix == 0 {
                    return;
                }
                mc = self.base.offset((*mc).suffix as isize) as *mut libc::c_void
                    as *mut CPpmd8_Context;
                if (*mc).num_stats as libc::c_uint != numMasked {
                    break;
                }
            }
            self.min_context = mc;
            see = self.Ppmd8_MakeEscFreq(numMasked, &mut escFreq);
            s_1 = self.base.offset((*self.min_context).union4.stats as isize) as *mut libc::c_void
                as *mut CPpmd_State;
            sum_0 = 0_i32 as UInt32;
            i_0 =
                ((*self.min_context).num_stats as libc::c_uint).wrapping_add(1_i32 as libc::c_uint);
            loop {
                let cur: libc::c_uint = (*s_1).symbol as libc::c_uint;
                if cur as libc::c_int == symbol {
                    let low: UInt32 = sum_0;
                    let freq_0: UInt32 = (*s_1).freq as UInt32;
                    let mut num2: libc::c_uint = 0;
                    if ((*see).shift as libc::c_int) < 7_i32 && {
                        (*see).count = (*see).count.wrapping_sub(1);
                        ((*see).count as libc::c_int) == 0_i32
                    } {
                        (*see).summ = (((*see).summ as libc::c_int) << 1_i32) as UInt16;
                        let fresh0 = (*see).shift;
                        (*see).shift = (*see).shift.wrapping_add(1);
                        (*see).count = (3_i32 << fresh0 as libc::c_int) as Byte
                    }
                    self.found_state = s_1;
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
                                (*s_1.offset(0_i32 as isize)).symbol as libc::c_uint;
                            let sym1_0: libc::c_uint =
                                (*s_1.offset(1_i32 as isize)).symbol as libc::c_uint;
                            s_1 = s_1.offset(2_i32 as isize);
                            sum_0 = (sum_0 as libc::c_uint).wrapping_add(
                                (*s_1.offset(-2_i32 as isize)).freq as libc::c_uint
                                    & *(charMask.as_mut_ptr() as *mut libc::c_uchar)
                                        .offset(sym0_0 as isize)
                                        as libc::c_uint,
                            ) as UInt32 as UInt32;
                            sum_0 = (sum_0 as libc::c_uint).wrapping_add(
                                (*s_1.offset(-1_i32 as isize)).freq as libc::c_uint
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
                    if sum_0 > self.range {
                        sum_0 = self.range
                    }
                    self.RangeEnc_Encode(low, freq_0, sum_0);
                    while self.low ^ self.low.wrapping_add(self.range)
                        < (1_i32 << 24_i32) as libc::c_uint
                        || self.range < (1_i32 << 15_i32) as libc::c_uint && {
                            self.range = (0_i32 as libc::c_uint).wrapping_sub(self.low)
                                & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                            1_i32 != 0
                        }
                    {
                        (*self.stream.out).write.expect("non-null function pointer")(
                            self.stream.out,
                            (self.low >> 24_i32) as Byte,
                        );
                        self.range <<= 8_i32;
                        self.low <<= 8_i32
                    }
                    self.Ppmd8_Update2();
                    return;
                }
                sum_0 = (sum_0 as libc::c_uint).wrapping_add(
                    (*s_1).freq as libc::c_uint
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
            (*see).summ = ((*see).summ as libc::c_uint).wrapping_add(total) as UInt16;
            if total > self.range {
                total = self.range
            }
            self.RangeEnc_Encode(sum_0, total.wrapping_sub(sum_0), total);
            let mut s2_0: *mut CPpmd_State =
                self.base.offset((*self.min_context).union4.stats as isize) as *mut libc::c_void
                    as *mut CPpmd_State;
            s_1 = s_1.offset(-1);
            *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset((*s_1).symbol as isize) =
                0_i32 as libc::c_uchar;
            loop {
                let sym0_1: libc::c_uint = (*s2_0.offset(0_i32 as isize)).symbol as libc::c_uint;
                let sym1_1: libc::c_uint = (*s2_0.offset(1_i32 as isize)).symbol as libc::c_uint;
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
    You must set (CPpmd8::stream.in) before Ppmd8_RangeDec_Init()

    Ppmd8_DecodeSymbol()
    out:
      >= 0 : decoded byte
        -1 : PPMD8_SYM_END   : End of payload marker
        -2 : PPMD8_SYM_ERROR : Data error
    */

    pub unsafe fn Ppmd8_Init_RangeDec(&mut self) -> BoolInt {
        let mut i: libc::c_uint = 0;
        self.code = 0_i32 as UInt32;
        self.range = 0xffffffff_u32;
        self.low = 0_i32 as UInt32;
        i = 0_i32 as libc::c_uint;
        while i < 4_i32 as libc::c_uint {
            self.code = self.code << 8_i32
                | (*self.stream.r#in).read.expect("non-null function pointer")(self.stream.r#in)
                    as libc::c_uint;
            i = i.wrapping_add(1)
        }
        (self.code < 0xffffffff_u32) as libc::c_int
    }
    // MY_NO_INLINE
    unsafe fn RangeDec_Decode(&mut self, mut start: UInt32, size: UInt32) {
        start = (start as libc::c_uint).wrapping_mul(self.range) as UInt32 as UInt32;
        self.low = (self.low as libc::c_uint).wrapping_add(start) as UInt32 as UInt32;
        self.code = (self.code as libc::c_uint).wrapping_sub(start) as UInt32 as UInt32;
        self.range = (self.range as libc::c_uint).wrapping_mul(size) as UInt32 as UInt32;
    }

    pub unsafe fn Ppmd8_DecodeSymbol(&mut self) -> libc::c_int {
        let mut charMask: [size_t; 32] = [0; 32];
        if (*self.min_context).num_stats as libc::c_int != 0_i32 {
            let mut s: *mut CPpmd_State =
                self.base.offset((*self.min_context).union4.stats as isize) as *mut libc::c_void
                    as *mut CPpmd_State;
            let mut i: libc::c_uint = 0;
            let mut count: UInt32 = 0;
            let mut hiCnt: UInt32 = 0;
            let mut summFreq: UInt32 = (*self.min_context).union2.summ_freq as UInt32;
            if summFreq > self.range {
                summFreq = self.range
            }
            self.range = (self.range as libc::c_uint).wrapping_div(summFreq) as UInt32 as UInt32;
            count = self.code.wrapping_div(self.range);
            hiCnt = count;
            count =
                (count as libc::c_uint).wrapping_sub((*s).freq as libc::c_uint) as UInt32 as UInt32;
            if (count as Int32) < 0_i32 {
                let mut sym: Byte = 0;
                self.RangeDec_Decode(0_i32 as UInt32, (*s).freq as UInt32);
                while self.low ^ self.low.wrapping_add(self.range)
                    < (1_i32 << 24_i32) as libc::c_uint
                    || self.range < (1_i32 << 15_i32) as libc::c_uint && {
                        self.range = (0_i32 as libc::c_uint).wrapping_sub(self.low)
                            & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                        1_i32 != 0
                    }
                {
                    self.code = self.code << 8_i32
                        | (*self.stream.r#in).read.expect("non-null function pointer")(
                            self.stream.r#in,
                        ) as libc::c_uint;
                    self.range <<= 8_i32;
                    self.low <<= 8_i32
                }
                self.found_state = s;
                sym = (*s).symbol;
                self.Ppmd8_Update1_0();
                return sym as libc::c_int;
            }
            self.prev_success = 0_i32 as libc::c_uint;
            i = (*self.min_context).num_stats as libc::c_uint;
            loop {
                s = s.offset(1);
                count = (count as libc::c_uint).wrapping_sub((*s).freq as libc::c_uint) as UInt32
                    as UInt32;
                if (count as Int32) < 0_i32 {
                    let mut sym_0: Byte = 0;
                    self.RangeDec_Decode(
                        hiCnt
                            .wrapping_sub(count)
                            .wrapping_sub((*s).freq as libc::c_uint),
                        (*s).freq as UInt32,
                    );
                    while self.low ^ self.low.wrapping_add(self.range)
                        < (1_i32 << 24_i32) as libc::c_uint
                        || self.range < (1_i32 << 15_i32) as libc::c_uint && {
                            self.range = (0_i32 as libc::c_uint).wrapping_sub(self.low)
                                & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                            1_i32 != 0
                        }
                    {
                        self.code = self.code << 8_i32
                            | (*self.stream.r#in).read.expect("non-null function pointer")(
                                self.stream.r#in,
                            ) as libc::c_uint;
                        self.range <<= 8_i32;
                        self.low <<= 8_i32
                    }
                    self.found_state = s;
                    sym_0 = (*s).symbol;
                    self.Ppmd8_Update1();
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
            self.RangeDec_Decode(hiCnt, summFreq.wrapping_sub(hiCnt));
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
            // i = p->min_context->num_stats - 1;
            // do { MASK((--s)->symbol) = 0; } while (--i);
            let mut s2: *mut CPpmd_State =
                self.base.offset((*self.min_context).union4.stats as isize) as *mut libc::c_void
                    as *mut CPpmd_State;
            *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset((*s).symbol as isize) =
                0_i32 as libc::c_uchar;
            loop {
                let sym0: libc::c_uint = (*s2.offset(0_i32 as isize)).symbol as libc::c_uint;
                let sym1: libc::c_uint = (*s2.offset(1_i32 as isize)).symbol as libc::c_uint;
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
                &mut (*self.min_context).union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State;
            let prob: *mut UInt16 = &mut *(*self.bin_summ.as_mut_ptr().offset(
                *self.ns2indx.as_mut_ptr().offset(
                    ((*(&mut (*self.min_context).union2 as *mut C2RustUnnamed_0
                        as *mut CPpmd_State))
                        .freq as size_t)
                        .wrapping_sub(1_i32 as libc::c_ulong) as isize,
                ) as isize,
            ))
            .as_mut_ptr()
            .offset(
                self.prev_success
                    .wrapping_add((self.run_length >> 26_i32 & 0x20_i32) as libc::c_uint)
                    .wrapping_add(
                        *self.ns2bsindx.as_mut_ptr().offset(
                            (*(self.base.offset((*self.min_context).suffix as isize)
                                as *mut libc::c_void
                                as *mut CPpmd8_Context))
                                .num_stats as isize,
                        ) as libc::c_uint,
                    )
                    .wrapping_add((*self.min_context).flags as libc::c_int as libc::c_uint)
                    as isize,
            ) as *mut UInt16;
            let mut pr: UInt32 = *prob as UInt32;
            let size0: UInt32 = (self.range >> 14_i32).wrapping_mul(pr);
            pr = pr
                .wrapping_sub(pr.wrapping_add((1_i32 << (7_i32 - 2_i32)) as libc::c_uint) >> 7_i32);
            if self.code < size0 {
                let mut sym_1: Byte = 0;
                *prob = pr.wrapping_add((1_i32 << 7_i32) as libc::c_uint) as UInt16;
                // self.RangeDec_DecodeBit0(size0);
                self.range = size0;
                while self.low ^ self.low.wrapping_add(self.range)
                    < (1_i32 << 24_i32) as libc::c_uint
                    || self.range < (1_i32 << 15_i32) as libc::c_uint && {
                        self.range = (0_i32 as libc::c_uint).wrapping_sub(self.low)
                            & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                        1_i32 != 0
                    }
                {
                    self.code = self.code << 8_i32
                        | (*self.stream.r#in).read.expect("non-null function pointer")(
                            self.stream.r#in,
                        ) as libc::c_uint;
                    self.range <<= 8_i32;
                    self.low <<= 8_i32
                }
                // sym = (p->found_state = Ppmd8Context_OneState(p->min_context))->symbol;
                // Ppmd8_UpdateBin(p);
                let freq: libc::c_uint = (*s_0).freq as libc::c_uint;
                let c: CTX_PTR = self.base.offset(
                    ((*s_0).successor_0 as libc::c_uint | ((*s_0).successor_1 as UInt32) << 16_i32)
                        as isize,
                ) as *mut libc::c_void as *mut CPpmd8_Context;
                sym_1 = (*s_0).symbol;
                self.found_state = s_0;
                self.prev_success = 1_i32 as libc::c_uint;
                self.run_length += 1;
                (*s_0).freq = freq
                    .wrapping_add((freq < 196_i32 as libc::c_uint) as libc::c_int as libc::c_uint)
                    as Byte;
                // self.NextContext(p);
                if self.order_fall == 0_i32 as libc::c_uint && c as *const Byte >= self.units_start
                {
                    self.min_context = c;
                    self.max_context = self.min_context
                } else {
                    self.Ppmd8_UpdateModel();
                }
                return sym_1 as libc::c_int;
            }
            *prob = pr as UInt16;
            self.init_esc = self.exp_escape[(pr >> 10_i32) as usize] as libc::c_uint;
            // self.RangeDec_DecodeBit1(rc2, size0);
            self.low = (self.low as libc::c_uint).wrapping_add(size0) as UInt32 as UInt32;
            self.code = (self.code as libc::c_uint).wrapping_sub(size0) as UInt32 as UInt32;
            self.range = (self.range
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
                z_0 =
                    (z_0 as libc::c_ulong).wrapping_add(8_i32 as libc::c_ulong) as size_t as size_t
            }
            *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset(
                (*(&mut (*self.min_context).union2 as *mut C2RustUnnamed_0 as *mut CPpmd_State))
                    .symbol as isize,
            ) = 0_i32 as libc::c_uchar;
            self.prev_success = 0_i32 as libc::c_uint
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
            while self.low ^ self.low.wrapping_add(self.range) < (1_i32 << 24_i32) as libc::c_uint
                || self.range < (1_i32 << 15_i32) as libc::c_uint && {
                    self.range = (0_i32 as libc::c_uint).wrapping_sub(self.low)
                        & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                    1_i32 != 0
                }
            {
                self.code = self.code << 8_i32
                    | (*self.stream.r#in).read.expect("non-null function pointer")(self.stream.r#in)
                        as libc::c_uint;
                self.range <<= 8_i32;
                self.low <<= 8_i32
            }
            mc = self.min_context;
            numMasked = (*mc).num_stats as libc::c_uint;
            loop {
                self.order_fall = self.order_fall.wrapping_add(1);
                if (*mc).suffix == 0 {
                    return -1_i32;
                }
                mc = self.base.offset((*mc).suffix as isize) as *mut libc::c_void
                    as *mut CPpmd8_Context;
                if (*mc).num_stats as libc::c_uint != numMasked {
                    break;
                }
            }
            s_1 = self.base.offset((*mc).union4.stats as isize) as *mut libc::c_void
                as *mut CPpmd_State;
            let mut num: libc::c_uint =
                ((*mc).num_stats as libc::c_uint).wrapping_add(1_i32 as libc::c_uint);
            let mut num2: libc::c_uint = num.wrapping_div(2_i32 as libc::c_uint);
            num &= 1_i32 as libc::c_uint;
            hiCnt_0 = (*s_1).freq as libc::c_uint
                & *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset((*s_1).symbol as isize)
                    as libc::c_uint
                & (0_i32 as libc::c_uint).wrapping_sub(num);
            s_1 = s_1.offset(num as isize);
            self.min_context = mc;
            loop {
                let sym0_0: libc::c_uint = (*s_1.offset(0_i32 as isize)).symbol as libc::c_uint;
                let sym1_0: libc::c_uint = (*s_1.offset(1_i32 as isize)).symbol as libc::c_uint;
                s_1 = s_1.offset(2_i32 as isize);
                hiCnt_0 = (hiCnt_0 as libc::c_uint).wrapping_add(
                    (*s_1.offset(-2_i32 as isize)).freq as libc::c_uint
                        & *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset(sym0_0 as isize)
                            as libc::c_uint,
                ) as UInt32 as UInt32;
                hiCnt_0 = (hiCnt_0 as libc::c_uint).wrapping_add(
                    (*s_1.offset(-1_i32 as isize)).freq as libc::c_uint
                        & *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset(sym1_0 as isize)
                            as libc::c_uint,
                ) as UInt32 as UInt32;
                num2 = num2.wrapping_sub(1);
                if num2 == 0 {
                    break;
                }
            }
            see = self.Ppmd8_MakeEscFreq(numMasked, &mut freqSum);
            freqSum = (freqSum as libc::c_uint).wrapping_add(hiCnt_0) as UInt32 as UInt32;
            freqSum2 = freqSum;
            if freqSum2 > self.range {
                freqSum2 = self.range
            }
            self.range = (self.range as libc::c_uint).wrapping_div(freqSum2) as UInt32 as UInt32;
            count_0 = self.code.wrapping_div(self.range);
            if count_0 < hiCnt_0 {
                let mut sym_2: Byte = 0;
                // Ppmd_See_Update(see); // new (see->summ) value can overflow over 16-bits in some rare cases
                s_1 = self.base.offset((*self.min_context).union4.stats as isize)
                    as *mut libc::c_void as *mut CPpmd_State;
                hiCnt_0 = count_0;
                loop {
                    count_0 = (count_0 as libc::c_uint).wrapping_sub(
                        (*s_1).freq as libc::c_uint
                            & *(charMask.as_mut_ptr() as *mut libc::c_uchar)
                                .offset((*s_1).symbol as isize)
                                as libc::c_uint,
                    ) as UInt32 as UInt32;
                    s_1 = s_1.offset(1);
                    if (count_0 as Int32) < 0_i32 {
                        break;
                    }
                    // count -= s->freq & (unsigned)(MASK((s)->symbol)); s++; if ((Int32)count < 0) break;
                }
                s_1 = s_1.offset(-1);
                self.RangeDec_Decode(
                    hiCnt_0
                        .wrapping_sub(count_0)
                        .wrapping_sub((*s_1).freq as libc::c_uint),
                    (*s_1).freq as UInt32,
                );
                while self.low ^ self.low.wrapping_add(self.range)
                    < (1_i32 << 24_i32) as libc::c_uint
                    || self.range < (1_i32 << 15_i32) as libc::c_uint && {
                        self.range = (0_i32 as libc::c_uint).wrapping_sub(self.low)
                            & ((1_i32 << 15_i32) - 1_i32) as libc::c_uint;
                        1_i32 != 0
                    }
                {
                    self.code = self.code << 8_i32
                        | (*self.stream.r#in).read.expect("non-null function pointer")(
                            self.stream.r#in,
                        ) as libc::c_uint;
                    self.range <<= 8_i32;
                    self.low <<= 8_i32
                }
                // new (see->summ) value can overflow over 16-bits in some rare cases
                if ((*see).shift as libc::c_int) < 7_i32 && {
                    (*see).count = (*see).count.wrapping_sub(1);
                    ((*see).count as libc::c_int) == 0_i32
                } {
                    (*see).summ = (((*see).summ as libc::c_int) << 1_i32) as UInt16;
                    let fresh0 = (*see).shift;
                    (*see).shift = (*see).shift.wrapping_add(1);
                    (*see).count = (3_i32 << fresh0 as libc::c_int) as Byte
                }
                self.found_state = s_1;
                sym_2 = (*s_1).symbol;
                self.Ppmd8_Update2();
                return sym_2 as libc::c_int;
            }
            if count_0 >= freqSum2 {
                return -2_i32;
            }
            self.RangeDec_Decode(hiCnt_0, freqSum2.wrapping_sub(hiCnt_0));
            // We increase (see->summ) for sum of Freqs of all non_Masked symbols.
            // new (see->summ) value can overflow over 16-bits in some rare cases
            (*see).summ = ((*see).summ as libc::c_uint).wrapping_add(freqSum) as UInt16;
            s_1 = self.base.offset((*self.min_context).union4.stats as isize) as *mut libc::c_void
                as *mut CPpmd_State;
            s2_0 = s_1
                .offset((*self.min_context).num_stats as libc::c_int as isize)
                .offset(1_i32 as isize);
            loop {
                *(charMask.as_mut_ptr() as *mut libc::c_uchar).offset((*s_1).symbol as isize) =
                    0_i32 as libc::c_uchar;
                s_1 = s_1.offset(1);
                if s_1 == s2_0 {
                    break;
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub r#in: *mut IByteIn,
    pub out: *mut IByteOut,
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

unsafe fn SetSuccessor(mut p: *mut CPpmd_State, v: CPpmd_Void_Ref) {
    (*p).successor_0 = v as UInt16;
    (*p).successor_1 = (v >> 16_i32) as UInt16;
}

unsafe fn SwapStates(t1: *mut CPpmd_State, t2: *mut CPpmd_State) {
    std::mem::swap(&mut (*t1), &mut (*t2));
}

unsafe fn pmalloc(_: ISzAllocPtr, size: u64) -> *mut libc::c_void {
    libc::malloc(size.try_into().unwrap()) /* EndMark */
}
unsafe fn pfree(_: ISzAllocPtr, addr: *mut libc::c_void) {
    libc::free(addr);
}

pub static mut IALLOC: ISzAlloc = {
    {
        ISzAlloc {
            alloc: Some(pmalloc as unsafe fn(_: ISzAllocPtr, _: u64) -> *mut libc::c_void),
            free: Some(pfree as unsafe fn(_: ISzAllocPtr, _: *mut libc::c_void) -> ()),
        }
    }
};
