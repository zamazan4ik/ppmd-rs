use libc::{free, malloc};

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
            magic: 0x84acaf8f as u32,
            attr: 0x80 as i32 as u32,
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
    pub write: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_uchar) -> ()>,
    pub fp: *mut libc::FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharReader {
    pub read: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_uchar>,
    pub fp: *mut libc::FILE,
    pub eof: bool,
}

pub unsafe extern "C" fn write(p: *mut libc::c_void, b: libc::c_uchar) {
    let cw: *mut CharWriter = p as *mut CharWriter;
    libc::fputc(b as i32, (*cw).fp as *mut libc::FILE);
}
pub unsafe extern "C" fn read(p: *mut libc::c_void) -> libc::c_uchar {
    let mut cr: *mut CharReader = p as *mut CharReader;
    if (*cr).eof {
        return 0 as i32 as libc::c_uchar;
    }
    let c: i32 = libc::fgetc((*cr).fp as *mut libc::FILE);
    if c == -(1 as i32) {
        (*cr).eof = 1 as i32 != 0;
        return 0 as i32 as libc::c_uchar;
    }
    return c as libc::c_uchar;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct IByteIn {
    pub read: Option<unsafe extern "C" fn(_: *const IByteIn) -> u8>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IByteOut {
    pub write: Option<unsafe extern "C" fn(_: *const IByteOut, _: u8) -> ()>,
}
/* Returns: result. (result != SZ_OK) means break.
Value (UInt64)(Int64)-1 for size means unknown value. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ISzAlloc {
    pub alloc: Option<unsafe extern "C" fn(_: ISzAllocPtr, _: u64) -> *mut libc::c_void>,
    pub free: Option<unsafe extern "C" fn(_: ISzAllocPtr, _: *mut libc::c_void) -> ()>,
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
    pub summ: u16,
    pub shift: u8,
    pub count: u8,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct CPpmd_State {
    pub symbol: u8,
    pub freq: u8,
    pub successor_low: u16,
    pub successor_high: u16,
}
pub type CPpmdStateRef = u32;
pub type CPpmdVoidRef = u32;
pub type CPpmdByteRef = u32;
/* Ppmd8.h -- PPMdI codec
2017-04-03 : Igor Pavlov : Public domain
This code is based on:
  PPMd var.I (2002): Dmitry Shkarin : Public domain
  Carryless rangecoder (1999): Dmitry Subbotin : Public domain */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd8_Context_ {
    pub num_stats: u8,
    pub flags: u8,
    pub summ_freq: u16,
    pub stats: CPpmdStateRef,
    pub suffix: CPpmd8ContextRef,
}
pub type CPpmd8ContextRef = u32;
pub type CPpmd8Context = CPpmd8_Context_;
pub type C2RustUnnamed = u32;
#[allow(dead_code)]
pub const PPMD8_RESTORE_METHOD_FREEZE: C2RustUnnamed = 2;
#[allow(dead_code)]
pub const PPMD8_RESTORE_METHOD_CUT_OFF: C2RustUnnamed = 1;
pub const PPMD8_RESTORE_METHOD_RESTART: C2RustUnnamed = 0;

#[derive(Copy, Clone)]
pub struct CPpmd8 {
    pub min_context: *mut CPpmd8Context,
    pub max_context: *mut CPpmd8Context,
    pub found_state: *mut CPpmd_State,
    pub order_fall: u32,
    pub init_esc: u32,
    pub prev_success: u32,
    pub max_order: u32,
    pub run_length: i32,
    pub init_rl: i32,
    pub size: u32,
    pub glue_count: u32,
    pub base: *mut u8,
    pub lo_unit: *mut u8,
    pub hi_unit: *mut u8,
    pub text: *mut u8,
    pub units_start: *mut u8,
    pub align_offset: u32,
    pub restore_method: u32,
    pub range: u32,
    pub code: u32,
    pub low: u32,
    pub stream: C2RustUnnamed_0,
    pub indx2units: [u8; 38],
    pub units2indx: [u8; 128],
    pub free_list: [CPpmdVoidRef; 38],
    pub stamps: [u32; 38],
    pub ns2bsindx: [u8; 256],
    pub ns2indx: [u8; 260],
    pub dummy_see: CPpmd_See,
    pub see: [[CPpmd_See; 32]; 24],
    pub bin_summ: [[u16; 64]; 25],
}

impl CPpmd8 {
    fn default_encoder(char_writer: &mut CharWriter) -> Self {
        Self {
            min_context: 0 as *mut CPpmd8Context,
            max_context: 0 as *mut CPpmd8Context,
            found_state: 0 as *mut CPpmd_State,
            order_fall: 0,
            init_esc: 0,
            prev_success: 0,
            max_order: 0,
            run_length: 0,
            init_rl: 0,
            size: 0,
            glue_count: 0,
            base: 0 as *mut u8,
            lo_unit: 0 as *mut u8,
            hi_unit: 0 as *mut u8,
            text: 0 as *mut u8,
            units_start: 0 as *mut u8,
            align_offset: 0,
            restore_method: 0,
            range: 0,
            code: 0,
            low: 0,
            stream: C2RustUnnamed_0 {
                out: char_writer as *mut CharWriter as *mut IByteOut,
            },
            indx2units: [0; 38],
            units2indx: [0; 128],
            free_list: [0; 38],
            stamps: [0; 38],
            ns2bsindx: [0; 256],
            ns2indx: [0; 260],
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
            min_context: 0 as *mut CPpmd8Context,
            max_context: 0 as *mut CPpmd8Context,
            found_state: 0 as *mut CPpmd_State,
            order_fall: 0,
            init_esc: 0,
            prev_success: 0,
            max_order: 0,
            run_length: 0,
            init_rl: 0,
            size: 0,
            glue_count: 0,
            base: 0 as *mut u8,
            lo_unit: 0 as *mut u8,
            hi_unit: 0 as *mut u8,
            text: 0 as *mut u8,
            units_start: 0 as *mut u8,
            align_offset: 0,
            restore_method: 0,
            range: 0,
            code: 0,
            low: 0,
            stream: C2RustUnnamed_0 {
                r#in: char_reader as *mut CharReader as *mut IByteIn,
            },
            indx2units: [0; 38],
            units2indx: [0; 128],
            free_list: [0; 38],
            stamps: [0; 38],
            ns2bsindx: [0; 256],
            ns2indx: [0; 260],
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

    unsafe fn construct(mut self) -> Self {
        let mut i: u32 = 0;
        let mut k: u32 = 0;
        self.base = 0 as *mut u8;
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
                self.units2indx[fresh0 as usize] = i as u8;
                step = step.wrapping_sub(1);
                if !(step != 0) {
                    break;
                }
            }
            self.indx2units[i as usize] = k as u8;
            i = i.wrapping_add(1)
        }
        self.ns2bsindx[0 as i32 as usize] = ((0 as i32) << 1 as i32) as u8;
        self.ns2bsindx[1 as i32 as usize] = ((1 as i32) << 1 as i32) as u8;
        libc::memset(
            self.ns2bsindx.as_mut_ptr().offset(2 as i32 as isize) as *mut libc::c_void,
            (2 as i32) << 1 as i32,
            9,
        );
        libc::memset(
            self.ns2bsindx.as_mut_ptr().offset(11 as i32 as isize) as *mut libc::c_void,
            (3 as i32) << 1 as i32,
            256 - 11,
        );
        i = 0 as i32 as u32;
        while i < 5 as i32 as u32 {
            self.ns2indx[i as usize] = i as u8;
            i = i.wrapping_add(1)
        }
        let mut m = i;
        k = 1 as i32 as u32;
        while i < 260 as i32 as u32 {
            self.ns2indx[i as usize] = m as u8;
            k = k.wrapping_sub(1);
            if k == 0 as i32 as u32 {
                m = m.wrapping_add(1);
                k = m.wrapping_sub(4 as i32 as u32)
            }
            i = i.wrapping_add(1)
        }

        self
    }

    pub unsafe fn new_encoder(char_writer: &mut CharWriter) -> Self {
        let initial_state = Self::default_encoder(char_writer);
        initial_state.construct()
    }

    pub unsafe fn new_decoder(char_reader: &mut CharReader) -> Self {
        let initial_state = Self::default_decoder(char_reader);
        initial_state.construct()
    }

    pub unsafe fn allocate(&mut self, size: u32, alloc: ISzAllocPtr) -> i32 {
        if self.base.is_null() || self.size != size {
            self.free(alloc);
            self.align_offset = (4 as i32 as u32).wrapping_sub(size & 3 as i32 as u32);
            self.base = (*alloc).alloc.expect("non-null function pointer")(
                alloc,
                self.align_offset.wrapping_add(size) as u64,
            ) as *mut u8;
            if self.base.is_null() {
                return 0 as i32;
            }
            self.size = size
        }

        1
    }

    pub unsafe fn range_decoder_init(&mut self) -> i32 {
        let mut i: u32 = 0;
        self.low = 0;
        self.range = 0xffffffff as u32;
        self.code = 0;
        while i < 4 as i32 as u32 {
            self.code = self.code << 8 as i32
                | (*self.stream.r#in).read.expect("non-null function pointer")(self.stream.r#in)
                    as u32;
            i = i.wrapping_add(1)
        }
        return (self.code < 0xffffffff as u32) as i32;
    }

    pub unsafe fn init(&mut self, max_order: u32, restore_method: u32) {
        self.max_order = max_order;
        self.restore_method = restore_method;
        self.restart_model();
        self.dummy_see.shift = 7 as i32 as u8;
        self.dummy_see.summ = 0 as i32 as u16;
        self.dummy_see.count = 64 as i32 as u8;
    }

    pub unsafe fn update2(&mut self) {
        (*self.min_context).summ_freq = ((*self.min_context).summ_freq as i32 + 4 as i32) as u16;
        (*self.found_state).freq = ((*self.found_state).freq as i32 + 4 as i32) as u8;
        if (*self.found_state).freq as i32 > 124 as i32 {
            self.rescale();
        }
        self.run_length = self.init_rl;
        self.update_model();
        self.min_context = self.max_context;
    }

    unsafe fn restore_model(&mut self, c1: CtxPtr) {
        self.text = self
            .base
            .offset(self.align_offset as isize)
            .offset(0 as i32 as isize);
        let mut c = self.max_context;
        while c != c1 {
            (*c).num_stats = (*c).num_stats.wrapping_sub(1);
            if (*c).num_stats as i32 == 0 as i32 {
                let s =
                    self.base.offset((*c).stats as isize) as *mut libc::c_void as *mut CPpmd_State;
                (*c).flags = (((*c).flags as i32 & 0x10 as i32)
                    + 0x8 as i32 * ((*s).symbol as i32 >= 0x40 as i32) as i32)
                    as u8;
                *(&mut (*c).summ_freq as *mut u16 as *mut CPpmd_State) = *s;
                self.special_free_unit(s as *mut libc::c_void);
                (*(&mut (*c).summ_freq as *mut u16 as *mut CPpmd_State)).freq =
                    (((*(&mut (*c).summ_freq as *mut u16 as *mut CPpmd_State)).freq as u32)
                        .wrapping_add(11 as i32 as u32)
                        >> 3 as i32) as u8
            } else {
                self.refresh(
                    c,
                    ((*c).num_stats as i32 + 3 as i32 >> 1 as i32) as u32,
                    0 as i32 as u32,
                );
            }
            c = self.base.offset((*c).suffix as isize) as *mut libc::c_void as *mut CPpmd8Context
        }
        while c != self.min_context {
            if (*c).num_stats == 0 {
                (*(&mut (*c).summ_freq as *mut u16 as *mut CPpmd_State)).freq =
                    ((*(&mut (*c).summ_freq as *mut u16 as *mut CPpmd_State)).freq as i32
                        - ((*(&mut (*c).summ_freq as *mut u16 as *mut CPpmd_State)).freq as i32
                            >> 1 as i32)) as u8
            } else {
                (*c).summ_freq = ((*c).summ_freq as i32 + 4 as i32) as u16;
                if (*c).summ_freq as i32 > 128 as i32 + 4 as i32 * (*c).num_stats as i32 {
                    self.refresh(
                        c,
                        ((*c).num_stats as i32 + 2 as i32 >> 1 as i32) as u32,
                        1 as i32 as u32,
                    );
                }
            }
            c = self.base.offset((*c).suffix as isize) as *mut libc::c_void as *mut CPpmd8Context
        }
        if self.restore_method == PPMD8_RESTORE_METHOD_RESTART as i32 as u32
            || self.get_used_memory() < self.size >> 1 as i32
        {
            self.restart_model();
        } else {
            while (*self.max_context).suffix != 0 {
                self.max_context = self.base.offset((*self.max_context).suffix as isize)
                    as *mut libc::c_void as *mut CPpmd8Context
            }
            loop {
                self.cut_off(self.max_context, 0 as i32 as u32);
                self.expand_text_area();
                if !(self.get_used_memory() > (3 as i32 as u32).wrapping_mul(self.size >> 2 as i32))
                {
                    break;
                }
            }
            self.glue_count = 0 as i32 as u32;
            self.order_fall = self.max_order
        };
    }

    unsafe fn create_successors(
        &mut self,
        skip: i32,
        mut s1: *mut CPpmd_State,
        mut c: CtxPtr,
    ) -> CtxPtr {
        let mut up_state: CPpmd_State = CPpmd_State {
            symbol: 0,
            freq: 0,
            successor_low: 0,
            successor_high: 0,
        };
        let up_branch: CPpmdByteRef = (*self.found_state).successor_low as u32
            | ((*self.found_state).successor_high as u32) << 16 as i32;
        /* fixed over Shkarin's code. Maybe it could work without + 1 too. */
        let mut ps: [*mut CPpmd_State; 17] = [0 as *mut CPpmd_State; 17];
        let mut num_ps: u32 = 0 as i32 as u32;
        if skip == 0 {
            let fresh2 = num_ps;
            num_ps = num_ps.wrapping_add(1);
            ps[fresh2 as usize] = self.found_state
        }
        while (*c).suffix != 0 {
            let mut s;
            c = self.base.offset((*c).suffix as isize) as *mut libc::c_void as *mut CPpmd8Context;
            if !s1.is_null() {
                s = s1;
                s1 = 0 as *mut CPpmd_State
            } else if (*c).num_stats as i32 != 0 as i32 {
                s = self.base.offset((*c).stats as isize) as *mut libc::c_void as *mut CPpmd_State;
                while (*s).symbol as i32 != (*self.found_state).symbol as i32 {
                    s = s.offset(1)
                }
                if ((*s).freq as i32) < 124 as i32 - 9 as i32 {
                    (*s).freq = (*s).freq.wrapping_add(1);
                    (*c).summ_freq = (*c).summ_freq.wrapping_add(1)
                }
            } else {
                s = &mut (*c).summ_freq as *mut u16 as *mut CPpmd_State;
                (*s).freq = ((*s).freq as i32
                    + (((*(self.base.offset((*c).suffix as isize) as *mut libc::c_void
                        as *mut CPpmd8Context))
                        .num_stats
                        == 0) as i32
                        & (((*s).freq as i32) < 24 as i32) as i32))
                    as u8
            }
            let successor = (*s).successor_low as u32 | ((*s).successor_high as u32) << 16 as i32;
            if successor != up_branch {
                c = self.base.offset(successor as isize) as *mut libc::c_void as *mut CPpmd8Context;
                if num_ps == 0 as i32 as u32 {
                    return c;
                }
                break;
            } else {
                let fresh3 = num_ps;
                num_ps = num_ps.wrapping_add(1);
                ps[fresh3 as usize] = s
            }
        }
        up_state.symbol = *(self.base.offset(up_branch as isize) as *mut libc::c_void as *const u8);
        set_successor(&mut up_state, up_branch.wrapping_add(1 as i32 as u32));
        let flags = (0x10 as i32 * ((*self.found_state).symbol as i32 >= 0x40 as i32) as i32
            + 0x8 as i32 * (up_state.symbol as i32 >= 0x40 as i32) as i32)
            as u8;
        if (*c).num_stats as i32 == 0 as i32 {
            up_state.freq = (*(&mut (*c).summ_freq as *mut u16 as *mut CPpmd_State)).freq
        } else {
            let mut s_0 =
                self.base.offset((*c).stats as isize) as *mut libc::c_void as *mut CPpmd_State;
            while (*s_0).symbol as i32 != up_state.symbol as i32 {
                s_0 = s_0.offset(1)
            }
            let cf = ((*s_0).freq as i32 - 1 as i32) as u32;
            let s0 = (((*c).summ_freq as i32 - (*c).num_stats as i32) as u32).wrapping_sub(cf);
            up_state.freq =
                (1 as i32 as u32).wrapping_add(if (2 as i32 as u32).wrapping_mul(cf) <= s0 {
                    ((5 as i32 as u32).wrapping_mul(cf) > s0) as i32 as u32
                } else {
                    cf.wrapping_add((2 as i32 as u32).wrapping_mul(s0))
                        .wrapping_sub(3 as i32 as u32)
                        .wrapping_div(s0)
                }) as u8
        }
        loop {
            /* Create Child */
            let mut c1; /* = AllocContext(p); */
            if self.hi_unit != self.lo_unit {
                self.hi_unit = self.hi_unit.offset(-(12 as i32 as isize)); /* check it */
                c1 = self.hi_unit as CtxPtr
            } else if self.free_list[0 as i32 as usize] != 0 as i32 as u32 {
                c1 = self.remove_node(0) as CtxPtr
            } else {
                c1 = self.alloc_units_rare(0) as CtxPtr;
                if c1.is_null() {
                    return 0 as CtxPtr;
                }
            }
            (*c1).num_stats = 0 as i32 as u8;
            (*c1).flags = flags;
            *(&mut (*c1).summ_freq as *mut u16 as *mut CPpmd_State) = up_state;
            (*c1).suffix = (c as *mut u8).offset_from(self.base) as libc::c_long as u32;
            num_ps = num_ps.wrapping_sub(1);
            set_successor(
                ps[num_ps as usize],
                (c1 as *mut u8).offset_from(self.base) as libc::c_long as u32,
            );
            c = c1;
            if !(num_ps != 0 as i32 as u32) {
                break;
            }
        }
        return c;
    }

    unsafe fn reduce_order(&mut self, mut s1: *mut CPpmd_State, mut c: CtxPtr) -> CtxPtr {
        let mut s;
        let c1: CtxPtr = c;
        let up_branch: CPpmdVoidRef = self.text.offset_from(self.base) as libc::c_long as u32;
        set_successor(self.found_state, up_branch);
        self.order_fall = self.order_fall.wrapping_add(1);
        loop {
            if !s1.is_null() {
                c = self.base.offset((*c).suffix as isize) as *mut libc::c_void
                    as *mut CPpmd8Context;
                s = s1;
                s1 = 0 as *mut CPpmd_State
            } else {
                if (*c).suffix == 0 {
                    return c;
                }
                c = self.base.offset((*c).suffix as isize) as *mut libc::c_void
                    as *mut CPpmd8Context;
                if (*c).num_stats != 0 {
                    s = self.base.offset((*c).stats as isize) as *mut libc::c_void
                        as *mut CPpmd_State;
                    if (*s).symbol as i32 != (*self.found_state).symbol as i32 {
                        loop {
                            s = s.offset(1);
                            if !((*s).symbol as i32 != (*self.found_state).symbol as i32) {
                                break;
                            }
                        }
                    }
                    if ((*s).freq as i32) < 124 as i32 - 9 as i32 {
                        (*s).freq = ((*s).freq as i32 + 2 as i32) as u8;
                        (*c).summ_freq = ((*c).summ_freq as i32 + 2 as i32) as u16
                    }
                } else {
                    s = &mut (*c).summ_freq as *mut u16 as *mut CPpmd_State;
                    (*s).freq = ((*s).freq as i32 + (((*s).freq as i32) < 32 as i32) as i32) as u8
                }
            }
            if (*s).successor_low as u32 | ((*s).successor_high as u32) << 16 as i32 != 0 {
                break;
            }
            set_successor(s, up_branch);
            self.order_fall = self.order_fall.wrapping_add(1)
        }
        if (*s).successor_low as u32 | ((*s).successor_high as u32) << 16 as i32 <= up_branch {
            let s2: *mut CPpmd_State = self.found_state;
            self.found_state = s;
            let successor = self.create_successors(0, 0 as *mut CPpmd_State, c);
            if successor.is_null() {
                set_successor(s, 0 as i32 as CPpmdVoidRef);
            } else {
                set_successor(
                    s,
                    (successor as *mut u8).offset_from(self.base) as libc::c_long as u32,
                );
            }
            self.found_state = s2
        }
        if self.order_fall == 1 as i32 as u32 && c1 == self.max_context {
            set_successor(
                self.found_state,
                (*s).successor_low as u32 | ((*s).successor_high as u32) << 16 as i32,
            );
            self.text = self.text.offset(-1)
        }
        if (*s).successor_low as u32 | ((*s).successor_high as u32) << 16 as i32 == 0 as i32 as u32
        {
            return 0 as CtxPtr;
        }
        return self.base.offset(
            ((*s).successor_low as u32 | ((*s).successor_high as u32) << 16 as i32) as isize,
        ) as *mut libc::c_void as *mut CPpmd8Context;
    }

    unsafe fn update_model(&mut self) {
        let mut f_successor: CPpmdVoidRef = (*self.found_state).successor_low as u32
            | ((*self.found_state).successor_high as u32) << 16 as i32;
        let mut c;
        let ns: u32;
        let f_freq: u32 = (*self.found_state).freq as u32;
        let flag;
        let f_symbol: u8 = (*self.found_state).symbol;
        let mut s: *mut CPpmd_State = 0 as *mut CPpmd_State;
        if ((*self.found_state).freq as i32) < 124 as i32 / 4 as i32
            && (*self.min_context).suffix != 0 as i32 as u32
        {
            c = self.base.offset((*self.min_context).suffix as isize) as *mut libc::c_void
                as *mut CPpmd8Context;
            if (*c).num_stats as i32 == 0 as i32 {
                s = &mut (*c).summ_freq as *mut u16 as *mut CPpmd_State;
                if ((*s).freq as i32) < 32 as i32 {
                    (*s).freq = (*s).freq.wrapping_add(1)
                }
            } else {
                s = self.base.offset((*c).stats as isize) as *mut libc::c_void as *mut CPpmd_State;
                if (*s).symbol as i32 != (*self.found_state).symbol as i32 {
                    loop {
                        s = s.offset(1);
                        if !((*s).symbol as i32 != (*self.found_state).symbol as i32) {
                            break;
                        }
                    }
                    if (*s.offset(0 as i32 as isize)).freq as i32
                        >= (*s.offset(-(1 as i32) as isize)).freq as i32
                    {
                        swap_states(
                            &mut *s.offset(0 as i32 as isize),
                            &mut *s.offset(-(1 as i32) as isize),
                        );
                        s = s.offset(-1)
                    }
                }
                if ((*s).freq as i32) < 124 as i32 - 9 as i32 {
                    (*s).freq = ((*s).freq as i32 + 2 as i32) as u8;
                    (*c).summ_freq = ((*c).summ_freq as i32 + 2 as i32) as u16
                }
            }
        }
        c = self.max_context;
        if self.order_fall == 0 as i32 as u32 && f_successor != 0 {
            let cs: CtxPtr = self.create_successors(1 as i32, s, self.min_context);
            if cs.is_null() {
                set_successor(self.found_state, 0 as i32 as CPpmdVoidRef);
                self.restore_model(c);
            } else {
                set_successor(
                    self.found_state,
                    (cs as *mut u8).offset_from(self.base) as libc::c_long as u32,
                );
                self.max_context = cs
            }
            return;
        }
        let fresh4 = self.text;
        self.text = self.text.offset(1);
        *fresh4 = (*self.found_state).symbol;
        let mut successor = self.text.offset_from(self.base) as libc::c_long as u32;
        if self.text >= self.units_start {
            self.restore_model(c);
            return;
        }
        if f_successor == 0 {
            let cs_0: CtxPtr = self.reduce_order(s, self.min_context);
            if cs_0.is_null() {
                self.restore_model(c);
                return;
            }
            f_successor = (cs_0 as *mut u8).offset_from(self.base) as libc::c_long as u32
        } else if (self.base.offset(f_successor as isize) as *mut libc::c_void as *mut u8)
            < self.units_start
        {
            let cs_1: CtxPtr = self.create_successors(0, s, self.min_context);
            if cs_1.is_null() {
                self.restore_model(c);
                return;
            }
            f_successor = (cs_1 as *mut u8).offset_from(self.base) as libc::c_long as u32
        }
        self.order_fall = self.order_fall.wrapping_sub(1);
        if self.order_fall == 0 {
            successor = f_successor;
            self.text = self
                .text
                .offset(-((self.max_context != self.min_context) as i32 as isize))
        }
        ns = (*self.min_context).num_stats as u32;
        let s0 = ((*self.min_context).summ_freq as u32)
            .wrapping_sub(ns)
            .wrapping_sub(f_freq);
        flag = (0x8 as i32 * (f_symbol as i32 >= 0x40 as i32) as i32) as u8;
        while c != self.min_context {
            let ns1;
            let mut cf;
            let sf;
            ns1 = (*c).num_stats as u32;
            if ns1 != 0 as i32 as u32 {
                if ns1 & 1 as i32 as u32 != 0 as i32 as u32 {
                    /* Expand for one UNIT */
                    let old_nu: u32 = ns1.wrapping_add(1 as i32 as u32) >> 1 as i32;
                    let i: u32 = self.units2indx
                        [(old_nu as u64).wrapping_sub(1 as i32 as u64) as usize]
                        as u32;
                    if i != self.units2indx[(old_nu as u64)
                        .wrapping_add(1 as i32 as u64)
                        .wrapping_sub(1 as i32 as u64)
                        as usize] as u32
                    {
                        let ptr: *mut libc::c_void =
                            self.alloc_units(i.wrapping_add(1 as i32 as u32));
                        let old_ptr;
                        if ptr.is_null() {
                            self.restore_model(c);
                            return;
                        }
                        old_ptr = self.base.offset((*c).stats as isize) as *mut libc::c_void
                            as *mut CPpmd_State
                            as *mut libc::c_void;
                        let mut d: *mut u32 = ptr as *mut u32;
                        let mut z: *const u32 = old_ptr as *const u32;
                        let mut n: u32 = old_nu;
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
                        self.insert_node(old_ptr, i);
                        (*c).stats = (ptr as *mut u8).offset_from(self.base) as libc::c_long as u32
                    }
                }
                (*c).summ_freq = ((*c).summ_freq as i32
                    + ((3 as i32 as u32)
                        .wrapping_mul(ns1)
                        .wrapping_add(1 as i32 as u32)
                        < ns) as i32) as u16
            } else {
                let mut s2: *mut CPpmd_State = self.alloc_units(0) as *mut CPpmd_State;
                if s2.is_null() {
                    self.restore_model(c);
                    return;
                }
                *s2 = *(&mut (*c).summ_freq as *mut u16 as *mut CPpmd_State);
                (*c).stats = (s2 as *mut u8).offset_from(self.base) as libc::c_long as u32;
                if ((*s2).freq as i32) < 124 as i32 / 4 as i32 - 1 as i32 {
                    (*s2).freq = (((*s2).freq as i32) << 1 as i32) as u8
                } else {
                    (*s2).freq = (124 as i32 - 4 as i32) as u8
                }
                (*c).summ_freq = ((*s2).freq as u32)
                    .wrapping_add(self.init_esc)
                    .wrapping_add((ns > 2 as i32 as u32) as i32 as u32)
                    as u16
            }
            cf = (2 as i32 as u32)
                .wrapping_mul(f_freq)
                .wrapping_mul(((*c).summ_freq as i32 + 6 as i32) as u32);
            sf = s0.wrapping_add((*c).summ_freq as u32);
            if cf < (6 as i32 as u32).wrapping_mul(sf) {
                cf = (1 as i32
                    + (cf > sf) as i32
                    + (cf >= (4 as i32 as u32).wrapping_mul(sf)) as i32)
                    as u32;
                (*c).summ_freq = ((*c).summ_freq as i32 + 4 as i32) as u16
            } else {
                cf = (4 as i32
                    + (cf > (9 as i32 as u32).wrapping_mul(sf)) as i32
                    + (cf > (12 as i32 as u32).wrapping_mul(sf)) as i32
                    + (cf > (15 as i32 as u32).wrapping_mul(sf)) as i32)
                    as u32;
                (*c).summ_freq = ((*c).summ_freq as u32).wrapping_add(cf) as u16
            }
            let mut s2_0: *mut CPpmd_State =
                (self.base.offset((*c).stats as isize) as *mut libc::c_void as *mut CPpmd_State)
                    .offset(ns1 as isize)
                    .offset(1 as i32 as isize);
            set_successor(s2_0, successor);
            (*s2_0).symbol = f_symbol;
            (*s2_0).freq = cf as u8;
            (*c).flags = ((*c).flags as i32 | flag as i32) as u8;
            (*c).num_stats = ns1.wrapping_add(1 as i32 as u32) as u8;
            c = self.base.offset((*c).suffix as isize) as *mut libc::c_void as *mut CPpmd8Context
        }
        self.min_context =
            self.base.offset(f_successor as isize) as *mut libc::c_void as *mut CPpmd8Context;
        self.max_context = self.min_context;
    }

    unsafe fn rescale(&mut self) {
        let mut i: u32;
        let adder: u32;
        let mut sum_freq: u32;
        let mut esc_freq: u32;
        let stats: *mut CPpmd_State = self.base.offset((*self.min_context).stats as isize)
            as *mut libc::c_void as *mut CPpmd_State;
        let mut s: *mut CPpmd_State = self.found_state;
        let tmp: CPpmd_State = *s;
        while s != stats {
            *s.offset(0 as i32 as isize) = *s.offset(-(1 as i32) as isize);
            s = s.offset(-1)
        }
        *s = tmp;
        esc_freq = ((*self.min_context).summ_freq as i32 - (*s).freq as i32) as u32;
        (*s).freq = ((*s).freq as i32 + 4 as i32) as u8;
        adder = (self.order_fall != 0 as i32 as u32) as i32 as u32;
        (*s).freq = (((*s).freq as u32).wrapping_add(adder) >> 1 as i32) as u8;
        sum_freq = (*s).freq as u32;
        i = (*self.min_context).num_stats as u32;
        loop {
            s = s.offset(1);
            esc_freq = esc_freq.wrapping_sub((*s).freq as u32);
            (*s).freq = (((*s).freq as u32).wrapping_add(adder) >> 1 as i32) as u8;
            sum_freq = sum_freq.wrapping_add((*s).freq as u32);
            if (*s.offset(0 as i32 as isize)).freq as i32
                > (*s.offset(-(1 as i32) as isize)).freq as i32
            {
                let mut s1: *mut CPpmd_State = s;
                let tmp_0: CPpmd_State = *s1;
                loop {
                    *s1.offset(0 as i32 as isize) = *s1.offset(-(1 as i32) as isize);
                    s1 = s1.offset(-1);
                    if !(s1 != stats
                        && tmp_0.freq as i32 > (*s1.offset(-(1 as i32) as isize)).freq as i32)
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
        if (*s).freq as i32 == 0 as i32 {
            let num_stats: u32 = (*self.min_context).num_stats as u32;
            let n0: u32;
            let n1: u32;
            loop {
                i = i.wrapping_add(1);
                s = s.offset(-1);
                if !((*s).freq as i32 == 0 as i32) {
                    break;
                }
            }
            esc_freq = esc_freq.wrapping_add(i);
            (*self.min_context).num_stats =
                ((*self.min_context).num_stats as u32).wrapping_sub(i) as u8;
            if (*self.min_context).num_stats as i32 == 0 as i32 {
                let mut tmp_1: CPpmd_State = *stats;
                tmp_1.freq = ((2 as i32 * tmp_1.freq as i32) as u32)
                    .wrapping_add(esc_freq)
                    .wrapping_sub(1 as i32 as u32)
                    .wrapping_div(esc_freq) as u8;
                if tmp_1.freq as i32 > 124 as i32 / 3 as i32 {
                    tmp_1.freq = (124 as i32 / 3 as i32) as u8
                }
                self.insert_node(
                    stats as *mut libc::c_void,
                    self.units2indx[((num_stats.wrapping_add(2 as i32 as u32) >> 1 as i32) as u64)
                        .wrapping_sub(1 as i32 as u64) as usize] as u32,
                );
                (*self.min_context).flags = (((*self.min_context).flags as i32 & 0x10 as i32)
                    + 0x8 as i32 * (tmp_1.symbol as i32 >= 0x40 as i32) as i32)
                    as u8;
                self.found_state =
                    &mut (*self.min_context).summ_freq as *mut u16 as *mut CPpmd_State;
                *self.found_state = tmp_1;
                return;
            }
            n0 = num_stats.wrapping_add(2 as i32 as u32) >> 1 as i32;
            n1 = ((*self.min_context).num_stats as i32 + 2 as i32 >> 1 as i32) as u32;
            if n0 != n1 {
                (*self.min_context).stats =
                    (self.shrink_units(stats as *mut libc::c_void, n0, n1) as *mut u8)
                        .offset_from(self.base) as libc::c_long as u32
            }
            (*self.min_context).flags = ((*self.min_context).flags as i32 & !(0x8 as i32)) as u8;
            s = self.base.offset((*self.min_context).stats as isize) as *mut libc::c_void
                as *mut CPpmd_State;
            (*self.min_context).flags = ((*self.min_context).flags as i32
                | 0x8 as i32 * ((*s).symbol as i32 >= 0x40 as i32) as i32)
                as u8;
            i = (*self.min_context).num_stats as u32;
            loop {
                s = s.offset(1);
                (*self.min_context).flags = ((*self.min_context).flags as i32
                    | 0x8 as i32 * ((*s).symbol as i32 >= 0x40 as i32) as i32)
                    as u8;
                i = i.wrapping_sub(1);
                if !(i != 0) {
                    break;
                }
            }
        }
        (*self.min_context).summ_freq = sum_freq
            .wrapping_add(esc_freq)
            .wrapping_sub(esc_freq >> 1 as i32) as u16;
        (*self.min_context).flags = ((*self.min_context).flags as i32 | 0x4 as i32) as u8;
        self.found_state = self.base.offset((*self.min_context).stats as isize) as *mut libc::c_void
            as *mut CPpmd_State;
    }

    pub unsafe fn make_esc_freq(&mut self, num_masked1: u32, esc_freq: *mut u32) -> *mut CPpmd_See {
        let mut see;
        if (*self.min_context).num_stats as i32 != 0xff as i32 {
            see = self.see[(self.ns2indx[((*self.min_context).num_stats as u32 as u64)
                .wrapping_add(2 as i32 as u64) as usize] as u32 as u64)
                .wrapping_sub(3 as i32 as u64) as usize]
                .as_mut_ptr()
                .offset(
                    ((*self.min_context).summ_freq as u32
                        > (11 as i32 as u32).wrapping_mul(
                            ((*self.min_context).num_stats as u32).wrapping_add(1 as i32 as u32),
                        )) as i32 as isize,
                )
                .offset(
                    (2 as i32 as u32).wrapping_mul(
                        ((2 as i32 as u32).wrapping_mul((*self.min_context).num_stats as u32)
                            < ((*(self.base.offset((*self.min_context).suffix as isize)
                                as *mut libc::c_void
                                as *mut CPpmd8Context))
                                .num_stats as u32)
                                .wrapping_add(num_masked1)) as i32 as u32,
                    ) as isize,
                )
                .offset((*self.min_context).flags as i32 as isize);
            let r: u32 = ((*see).summ as i32 >> (*see).shift as i32) as u32;
            (*see).summ = ((*see).summ as u32).wrapping_sub(r) as u16;
            *esc_freq = r.wrapping_add((r == 0 as i32 as u32) as i32 as u32)
        } else {
            see = &mut self.dummy_see;
            *esc_freq = 1
        }
        return see;
    }

    unsafe fn next_context(&mut self) {
        let c: CtxPtr = self.base.offset(
            ((*self.found_state).successor_low as u32
                | ((*self.found_state).successor_high as u32) << 16 as i32) as isize,
        ) as *mut libc::c_void as *mut CPpmd8Context;
        if self.order_fall == 0 as i32 as u32 && c as *mut u8 >= self.units_start {
            self.max_context = c;
            self.min_context = self.max_context
        } else {
            self.update_model();
            self.min_context = self.max_context
        };
    }

    pub unsafe fn free(&mut self, alloc: ISzAllocPtr) {
        (*alloc).free.expect("non-null function pointer")(alloc, self.base as *mut libc::c_void);
        self.size = 0 as i32 as u32;
        self.base = 0 as *mut u8;
    }

    unsafe fn insert_node(&mut self, node: *mut libc::c_void, indx: u32) {
        (*(node as *mut Cppmd8Node)).stamp = 0xffffffff as u32;
        (*(node as *mut Cppmd8Node)).next = self.free_list[indx as usize];
        (*(node as *mut Cppmd8Node)).nu = self.indx2units[indx as usize] as u32;
        self.free_list[indx as usize] =
            (node as *mut u8).offset_from(self.base) as libc::c_long as u32;
        self.stamps[indx as usize] = self.stamps[indx as usize].wrapping_add(1);
    }
    unsafe fn remove_node(&mut self, indx: u32) -> *mut libc::c_void {
        let node: *mut Cppmd8Node =
            self.base.offset(self.free_list[indx as usize] as isize) as *mut Cppmd8Node;
        self.free_list[indx as usize] = (*node).next;
        self.stamps[indx as usize] = self.stamps[indx as usize].wrapping_sub(1);
        return node as *mut libc::c_void;
    }
    unsafe fn split_block(&mut self, mut ptr: *mut libc::c_void, old_indx: u32, new_indx: u32) {
        let mut i: u32;
        let nu: u32 = (self.indx2units[old_indx as usize] as i32
            - self.indx2units[new_indx as usize] as i32) as u32;
        ptr = (ptr as *mut u8).offset(
            (self.indx2units[new_indx as usize] as u32).wrapping_mul(12 as i32 as u32) as isize,
        ) as *mut libc::c_void;
        i = self.units2indx[(nu as u64).wrapping_sub(1 as i32 as u64) as usize] as u32;
        if self.indx2units[i as usize] as u32 != nu {
            i = i.wrapping_sub(1);
            let k: u32 = self.indx2units[i as usize] as u32;
            self.insert_node(
                (ptr as *mut u8).offset(k.wrapping_mul(12 as i32 as u32) as isize)
                    as *mut libc::c_void,
                nu.wrapping_sub(k).wrapping_sub(1 as i32 as u32),
            );
        }
        self.insert_node(ptr, i);
    }

    unsafe fn glue_free_blocks(&mut self) {
        let mut head: Cppmd8NodeRef = 0 as i32 as Cppmd8NodeRef;
        let mut prev: *mut Cppmd8NodeRef = &mut head;
        let mut i: u32;
        self.glue_count = ((1 as i32) << 13 as i32) as u32;
        libc::memset(
            self.stamps.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            (::std::mem::size_of::<[u32; 38]>() as u64)
                .try_into()
                .unwrap(),
        );
        /* Order-0 context is always at top UNIT, so we don't need guard NODE at the end.
        All blocks up to p->lo_unit can be free, so we need guard NODE at lo_unit. */
        if self.lo_unit != self.hi_unit {
            (*(self.lo_unit as *mut Cppmd8Node)).stamp = 0 as i32 as u32
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
            let mut next: Cppmd8NodeRef = self.free_list[i as usize];
            self.free_list[i as usize] = 0 as i32 as CPpmdVoidRef;
            while next != 0 as i32 as u32 {
                let mut node: *mut Cppmd8Node = self.base.offset(next as isize) as *mut Cppmd8Node;
                if (*node).nu != 0 as i32 as u32 {
                    let mut node2;
                    *prev = next;
                    prev = &mut (*node).next;
                    loop {
                        node2 = node.offset((*node).nu as isize);
                        if !((*node2).stamp == 0xffffffff as u32) {
                            break;
                        }
                        (*node).nu = ((*node).nu as u32).wrapping_add((*node2).nu) as u32 as u32;
                        (*node2).nu = 0 as i32 as u32
                    }
                }
                next = (*node).next
            }
            i = i.wrapping_add(1)
        }
        *prev = 0 as i32 as Cppmd8NodeRef;
        /* Fill lists of free blocks */
        while head != 0 as i32 as u32 {
            let mut node_0: *mut Cppmd8Node = self.base.offset(head as isize) as *mut Cppmd8Node; /* AllocContext(p); */
            let mut nu: u32; /* alloc_units(p, PPMD_NUM_INDEXES - 1); */
            head = (*node_0).next; /* unused */
            nu = (*node_0).nu;
            if nu == 0 as i32 as u32 {
                continue;
            }
            while nu > 128 as i32 as u32 {
                self.insert_node(
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
            i = self.units2indx[(nu as u64).wrapping_sub(1 as i32 as u64) as usize] as u32;
            if self.indx2units[i as usize] as u32 != nu {
                i = i.wrapping_sub(1);
                let k: u32 = self.indx2units[i as usize] as u32;
                self.insert_node(
                    node_0.offset(k as isize) as *mut libc::c_void,
                    nu.wrapping_sub(k).wrapping_sub(1 as i32 as u32),
                );
            }
            self.insert_node(node_0 as *mut libc::c_void, i);
        }
    }
    unsafe fn alloc_units_rare(&mut self, indx: u32) -> *mut libc::c_void {
        let mut i: u32;
        let ret_val;
        if self.glue_count == 0 as i32 as u32 {
            self.glue_free_blocks();
            if self.free_list[indx as usize] != 0 as i32 as u32 {
                return self.remove_node(indx);
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
                let num_bytes: u32 =
                    (self.indx2units[indx as usize] as u32).wrapping_mul(12 as i32 as u32);
                self.glue_count = self.glue_count.wrapping_sub(1);
                return if self.units_start.offset_from(self.text) as libc::c_long as u32 > num_bytes
                {
                    self.units_start = self.units_start.offset(-(num_bytes as isize));
                    self.units_start
                } else {
                    0 as *mut u8
                } as *mut libc::c_void;
            }
            if !(self.free_list[i as usize] == 0 as i32 as u32) {
                break;
            }
        }
        ret_val = self.remove_node(i);
        self.split_block(ret_val, i, indx);
        return ret_val;
    }
    unsafe fn alloc_units(&mut self, indx: u32) -> *mut libc::c_void {
        if self.free_list[indx as usize] != 0 as i32 as u32 {
            return self.remove_node(indx);
        }
        let num_bytes = (self.indx2units[indx as usize] as u32).wrapping_mul(12 as i32 as u32);
        if num_bytes <= self.hi_unit.offset_from(self.lo_unit) as libc::c_long as u32 {
            let ret_val: *mut libc::c_void = self.lo_unit as *mut libc::c_void;
            self.lo_unit = self.lo_unit.offset(num_bytes as isize);
            return ret_val;
        }
        return self.alloc_units_rare(indx);
    }
    unsafe fn shrink_units(
        &mut self,
        old_ptr: *mut libc::c_void,
        old_nu: u32,
        new_nu: u32,
    ) -> *mut libc::c_void {
        let i0: u32 =
            self.units2indx[(old_nu as u64).wrapping_sub(1 as i32 as u64) as usize] as u32;
        let i1: u32 =
            self.units2indx[(new_nu as u64).wrapping_sub(1 as i32 as u64) as usize] as u32;
        if i0 == i1 {
            return old_ptr;
        }
        if self.free_list[i1 as usize] != 0 as i32 as u32 {
            let ptr: *mut libc::c_void = self.remove_node(i1);
            let mut d: *mut u32 = ptr as *mut u32;
            let mut z: *const u32 = old_ptr as *const u32;
            let mut n: u32 = new_nu;
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
            self.insert_node(old_ptr, i0);
            return ptr;
        }
        self.split_block(old_ptr, i0, i1);
        return old_ptr;
    }
    unsafe fn free_units(&mut self, ptr: *mut libc::c_void, nu: u32) {
        self.insert_node(
            ptr,
            self.units2indx[(nu as u64).wrapping_sub(1 as i32 as u64) as usize] as u32,
        );
    }
    unsafe fn special_free_unit(&mut self, ptr: *mut libc::c_void) {
        if ptr as *mut u8 != self.units_start {
            self.insert_node(ptr, 0 as i32 as u32);
        } else {
            self.units_start = self.units_start.offset(12 as i32 as isize)
        };
    }
    unsafe fn move_units_up(&mut self, old_ptr: *mut libc::c_void, nu: u32) -> *mut libc::c_void {
        let indx: u32 = self.units2indx[(nu as u64).wrapping_sub(1 as i32 as u64) as usize] as u32;
        let ptr;
        if old_ptr as *mut u8 > self.units_start.offset((16 as i32 * 1024 as i32) as isize)
            || (old_ptr as *mut u8).offset_from(self.base) as libc::c_long as u32
                > self.free_list[indx as usize]
        {
            return old_ptr;
        }
        ptr = self.remove_node(indx);
        let mut d: *mut u32 = ptr as *mut u32;
        let mut z: *const u32 = old_ptr as *const u32;
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
        if old_ptr as *mut u8 != self.units_start {
            self.insert_node(old_ptr, indx);
        } else {
            self.units_start = self.units_start.offset(
                (self.indx2units[indx as usize] as u32).wrapping_mul(12 as i32 as u32) as isize,
            )
        }
        return ptr;
    }
    unsafe fn expand_text_area(&mut self) {
        let mut count: [u32; 38] = [0; 38];
        let mut i: u32;
        libc::memset(
            count.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            (::std::mem::size_of::<[u32; 38]>() as u64)
                .try_into()
                .unwrap(),
        );
        if self.lo_unit != self.hi_unit {
            (*(self.lo_unit as *mut Cppmd8Node)).stamp = 0 as i32 as u32
        }
        let mut node: *mut Cppmd8Node = self.units_start as *mut Cppmd8Node;
        while (*node).stamp == 0xffffffff as u32 {
            (*node).stamp = 0 as i32 as u32;
            count[self.units2indx[((*node).nu as u64).wrapping_sub(1 as i32 as u64) as usize]
                as usize] = count[self.units2indx
                [((*node).nu as u64).wrapping_sub(1 as i32 as u64) as usize]
                as usize]
                .wrapping_add(1);
            node = node.offset((*node).nu as isize)
        }
        self.units_start = node as *mut u8;
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
            let mut next: *mut Cppmd8NodeRef = &mut *self.free_list.as_mut_ptr().offset(i as isize)
                as *mut CPpmdVoidRef
                as *mut Cppmd8NodeRef;
            while count[i as usize] != 0 as i32 as u32 {
                let mut node_0: *mut Cppmd8Node =
                    self.base.offset(*next as isize) as *mut Cppmd8Node;
                while (*node_0).stamp == 0 as i32 as u32 {
                    *next = (*node_0).next;
                    node_0 = self.base.offset(*next as isize) as *mut Cppmd8Node;
                    self.stamps[i as usize] = self.stamps[i as usize].wrapping_sub(1);
                    count[i as usize] = count[i as usize].wrapping_sub(1);
                    if count[i as usize] == 0 as i32 as u32 {
                        break;
                    }
                }
                next = &mut (*node_0).next
            }
            i = i.wrapping_add(1)
        }
    }

    unsafe fn restart_model(&mut self) {
        let mut i: u32;
        let mut k: u32;
        let mut m: u32;
        let mut r: u32;
        libc::memset(
            self.free_list.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            (::std::mem::size_of::<[CPpmdVoidRef; 38]>() as u64)
                .try_into()
                .unwrap(),
        );
        libc::memset(
            self.stamps.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            (::std::mem::size_of::<[u32; 38]>() as u64)
                .try_into()
                .unwrap(),
        );
        self.text = self
            .base
            .offset(self.align_offset as isize)
            .offset(0 as i32 as isize);
        self.hi_unit = self.text.offset(self.size as isize);
        self.units_start = self.hi_unit.offset(
            -(self
                .size
                .wrapping_div(8 as i32 as u32)
                .wrapping_div(12 as i32 as u32)
                .wrapping_mul(7 as i32 as u32)
                .wrapping_mul(12 as i32 as u32) as isize),
        );
        self.lo_unit = self.units_start;
        self.glue_count = 0 as i32 as u32;
        self.order_fall = self.max_order;
        self.init_rl = -((if self.max_order < 12 as i32 as u32 {
            self.max_order
        } else {
            12 as i32 as u32
        }) as i32)
            - 1 as i32;
        self.run_length = self.init_rl;
        self.prev_success = 0 as i32 as u32;
        self.hi_unit = self.hi_unit.offset(-(12 as i32 as isize));
        self.max_context = self.hi_unit as CtxPtr;
        self.min_context = self.max_context;
        (*self.min_context).suffix = 0 as i32 as CPpmd8ContextRef;
        (*self.min_context).num_stats = 255 as i32 as u8;
        (*self.min_context).flags = 0 as i32 as u8;
        (*self.min_context).summ_freq = (256 as i32 + 1 as i32) as u16;
        self.found_state = self.lo_unit as *mut CPpmd_State;
        self.lo_unit = self
            .lo_unit
            .offset(((256 as i32 / 2 as i32) as u32).wrapping_mul(12 as i32 as u32) as isize);
        (*self.min_context).stats =
            (self.found_state as *mut u8).offset_from(self.base) as libc::c_long as u32;
        i = 0 as i32 as u32;
        while i < 256 as i32 as u32 {
            let mut s: *mut CPpmd_State =
                &mut *self.found_state.offset(i as isize) as *mut CPpmd_State;
            (*s).symbol = i as u8;
            (*s).freq = 1 as i32 as u8;
            set_successor(s, 0 as i32 as CPpmdVoidRef);
            i = i.wrapping_add(1)
        }
        m = 0 as i32 as u32;
        i = m;
        while m < 25 as i32 as u32 {
            while self.ns2indx[i as usize] as u32 == m {
                i = i.wrapping_add(1)
            }
            k = 0 as i32 as u32;
            while k < 8 as i32 as u32 {
                let val: u16 = (((1 as i32) << 7 as i32 + 7 as i32) as u32).wrapping_sub(
                    (K_INIT_BIN_ESC[k as usize] as u32)
                        .wrapping_div(i.wrapping_add(1 as i32 as u32)),
                ) as u16;
                let dest: *mut u16 = self.bin_summ[m as usize].as_mut_ptr().offset(k as isize);
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
            while self.ns2indx[(i as u64).wrapping_add(3 as i32 as u64) as usize] as u32
                == m.wrapping_add(3 as i32 as u32)
            {
                i = i.wrapping_add(1)
            }
            k = 0 as i32 as u32;
            while k < 32 as i32 as u32 {
                let mut s_0: *mut CPpmd_See = &mut *(*self.see.as_mut_ptr().offset(m as isize))
                    .as_mut_ptr()
                    .offset(k as isize)
                    as *mut CPpmd_See;
                (*s_0).shift = (7 as i32 - 4 as i32) as u8;
                (*s_0).summ = ((2 as i32 as u32)
                    .wrapping_mul(i)
                    .wrapping_add(5 as i32 as u32)
                    << (*s_0).shift as i32) as u16;
                (*s_0).count = 7 as i32 as u8;
                k = k.wrapping_add(1)
            }
            m = m.wrapping_add(1)
        }
    }
    unsafe fn refresh(&mut self, mut ctx: CtxPtr, old_nu: u32, scale: u32) {
        let mut i: u32 = (*ctx).num_stats as u32;
        let mut esc_freq: u32;
        let mut sum_freq: u32;
        let mut flags: u32;
        let mut s: *mut CPpmd_State = self.shrink_units(
            self.base.offset((*ctx).stats as isize) as *mut libc::c_void as *mut CPpmd_State
                as *mut libc::c_void,
            old_nu,
            i.wrapping_add(2 as i32 as u32) >> 1 as i32,
        ) as *mut CPpmd_State;
        (*ctx).stats = (s as *mut u8).offset_from(self.base) as libc::c_long as u32;
        flags = ((*ctx).flags as u32
            & (0x10 as i32 as u32).wrapping_add((0x4 as i32 as u32).wrapping_mul(scale)))
        .wrapping_add((0x8 as i32 * ((*s).symbol as i32 >= 0x40 as i32) as i32) as u32);
        esc_freq = ((*ctx).summ_freq as i32 - (*s).freq as i32) as u32;
        (*s).freq = (((*s).freq as u32).wrapping_add(scale) >> scale) as u8;
        sum_freq = (*s).freq as u32;
        loop {
            s = s.offset(1);
            esc_freq = esc_freq.wrapping_sub((*s).freq as u32);
            (*s).freq = (((*s).freq as u32).wrapping_add(scale) >> scale) as u8;
            sum_freq = sum_freq.wrapping_add((*s).freq as u32);
            flags |= (0x8 as i32 * ((*s).symbol as i32 >= 0x40 as i32) as i32) as u32;
            i = i.wrapping_sub(1);
            if !(i != 0) {
                break;
            }
        }
        (*ctx).summ_freq = sum_freq.wrapping_add(esc_freq.wrapping_add(scale) >> scale) as u16;
        (*ctx).flags = flags as u8;
    }

    unsafe fn cut_off(&mut self, mut ctx: CtxPtr, order: u32) -> CPpmdVoidRef {
        let mut i: i32;
        let tmp: u32;
        let mut s;
        if (*ctx).num_stats == 0 {
            s = &mut (*ctx).summ_freq as *mut u16 as *mut CPpmd_State;
            if self.base.offset(
                ((*s).successor_low as u32 | ((*s).successor_high as u32) << 16 as i32) as isize,
            ) as *mut libc::c_void as *mut u8
                >= self.units_start
            {
                if order < self.max_order {
                    set_successor(
                        s,
                        self.cut_off(
                            self.base.offset(
                                ((*s).successor_low as u32
                                    | ((*s).successor_high as u32) << 16 as i32)
                                    as isize,
                            ) as *mut libc::c_void
                                as *mut CPpmd8Context,
                            order.wrapping_add(1 as i32 as u32),
                        ),
                    );
                } else {
                    set_successor(s, 0 as i32 as CPpmdVoidRef);
                }
                if (*s).successor_low as u32 | ((*s).successor_high as u32) << 16 as i32 != 0
                    || order <= 9 as i32 as u32
                {
                    /* O_BOUND */
                    return (ctx as *mut u8).offset_from(self.base) as libc::c_long as u32;
                }
            }
            self.special_free_unit(ctx as *mut libc::c_void);
            return 0 as i32 as CPpmdVoidRef;
        }
        tmp = ((*ctx).num_stats as u32).wrapping_add(2 as i32 as u32) >> 1 as i32;
        (*ctx).stats = (self.move_units_up(
            self.base.offset((*ctx).stats as isize) as *mut libc::c_void as *mut CPpmd_State
                as *mut libc::c_void,
            tmp,
        ) as *mut u8)
            .offset_from(self.base) as libc::c_long as u32;
        i = (*ctx).num_stats as i32;
        s = (self.base.offset((*ctx).stats as isize) as *mut libc::c_void as *mut CPpmd_State)
            .offset(i as isize);
        while s >= self.base.offset((*ctx).stats as isize) as *mut libc::c_void as *mut CPpmd_State
        {
            if (self.base.offset(
                ((*s).successor_low as u32 | ((*s).successor_high as u32) << 16 as i32) as isize,
            ) as *mut libc::c_void as *mut u8)
                < self.units_start
            {
                let fresh1 = i;
                i = i - 1;
                let s2: *mut CPpmd_State = (self.base.offset((*ctx).stats as isize)
                    as *mut libc::c_void
                    as *mut CPpmd_State)
                    .offset(fresh1 as isize);
                set_successor(s, 0 as i32 as CPpmdVoidRef);
                swap_states(s, s2);
            } else if order < self.max_order {
                set_successor(
                    s,
                    self.cut_off(
                        self.base.offset(
                            ((*s).successor_low as u32 | ((*s).successor_high as u32) << 16 as i32)
                                as isize,
                        ) as *mut libc::c_void as *mut CPpmd8Context,
                        order.wrapping_add(1 as i32 as u32),
                    ),
                );
            } else {
                set_successor(s, 0 as i32 as CPpmdVoidRef);
            }
            s = s.offset(-1)
        }
        if i != (*ctx).num_stats as i32 && order != 0 {
            (*ctx).num_stats = i as u8;
            s = self.base.offset((*ctx).stats as isize) as *mut libc::c_void as *mut CPpmd_State;
            if i < 0 as i32 {
                self.free_units(s as *mut libc::c_void, tmp);
                self.special_free_unit(ctx as *mut libc::c_void);
                return 0 as i32 as CPpmdVoidRef;
            }
            if i == 0 as i32 {
                (*ctx).flags = (((*ctx).flags as i32 & 0x10 as i32)
                    + 0x8 as i32 * ((*s).symbol as i32 >= 0x40 as i32) as i32)
                    as u8;
                *(&mut (*ctx).summ_freq as *mut u16 as *mut CPpmd_State) = *s;
                self.free_units(s as *mut libc::c_void, tmp);
                /* 9.31: the code was fixed. It's was not BUG, if freq <= MAX_FREQ = 124 */
                (*(&mut (*ctx).summ_freq as *mut u16 as *mut CPpmd_State)).freq =
                    (((*(&mut (*ctx).summ_freq as *mut u16 as *mut CPpmd_State)).freq as u32)
                        .wrapping_add(11 as i32 as u32)
                        >> 3 as i32) as u8
            } else {
                self.refresh(
                    ctx,
                    tmp,
                    ((*ctx).summ_freq as i32 > 16 as i32 * i) as i32 as u32,
                );
            }
        }
        return (ctx as *mut u8).offset_from(self.base) as libc::c_long as u32;
    }

    pub unsafe fn update1(&mut self) {
        let mut s: *mut CPpmd_State = self.found_state;
        (*s).freq = ((*s).freq as i32 + 4 as i32) as u8;
        (*self.min_context).summ_freq = ((*self.min_context).summ_freq as i32 + 4 as i32) as u16;
        if (*s.offset(0 as i32 as isize)).freq as i32
            > (*s.offset(-(1 as i32) as isize)).freq as i32
        {
            swap_states(
                &mut *s.offset(0 as i32 as isize),
                &mut *s.offset(-(1 as i32) as isize),
            );
            s = s.offset(-1);
            self.found_state = s;
            if (*s).freq as i32 > 124 as i32 {
                self.rescale();
            }
        }
        self.next_context();
    }

    pub unsafe fn update1_0(&mut self) {
        self.prev_success = (2 as i32 * (*self.found_state).freq as i32
            >= (*self.min_context).summ_freq as i32) as i32 as u32;
        self.run_length = (self.run_length as u32).wrapping_add(self.prev_success) as i32;
        (*self.min_context).summ_freq = ((*self.min_context).summ_freq as i32 + 4 as i32) as u16;
        (*self.found_state).freq = ((*self.found_state).freq as i32 + 4 as i32) as u8;
        if (*self.found_state).freq as i32 > 124 as i32 {
            self.rescale();
        }
        self.next_context();
    }

    pub unsafe fn update_bin(&mut self) {
        (*self.found_state).freq = ((*self.found_state).freq as i32
            + (((*self.found_state).freq as i32) < 196 as i32) as i32)
            as u8;
        self.prev_success = 1 as i32 as u32;
        self.run_length += 1;
        self.next_context();
    }

    unsafe fn get_used_memory(&mut self) -> u32 {
        let mut v = 0u32;
        let mut i = 0u32;
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
            v = (v as u32).wrapping_add(
                self.stamps[i as usize].wrapping_mul(self.indx2units[i as usize] as u32),
            ) as u32 as u32;
            i = i.wrapping_add(1)
        }
        return self
            .size
            .wrapping_sub(self.hi_unit.offset_from(self.lo_unit) as libc::c_long as u32)
            .wrapping_sub(self.units_start.offset_from(self.text) as libc::c_long as u32)
            .wrapping_sub(v.wrapping_mul(12 as i32 as u32));
    }

    /* ---------- Encode ---------- */
    pub unsafe fn range_enc_flush_data(&mut self) {
        let mut i: u32 = 0; /* EndMarker (symbol = -1) */
        while i < 4 {
            (*self.stream.out).write.expect("non-null function pointer")(
                self.stream.out,
                (self.low >> 24 as i32) as libc::c_uchar,
            );
            i = i.wrapping_add(1);
            self.low <<= 8 as i32
        }
    }
    unsafe fn range_enc_normalize(&mut self) {
        while self.low ^ self.low.wrapping_add(self.range) < ((1 as i32) << 24 as i32) as u32
            || self.range < ((1 as i32) << 15 as i32) as u32 && {
                self.range = (0 as i32 as u32).wrapping_sub(self.low)
                    & (((1 as i32) << 15 as i32) - 1 as i32) as u32;
                (1 as i32) != 0
            }
        {
            (*self.stream.out).write.expect("non-null function pointer")(
                self.stream.out,
                (self.low >> 24 as i32) as libc::c_uchar,
            );
            self.range <<= 8 as i32;
            self.low <<= 8 as i32
        }
    }
    unsafe fn range_enc_encode(&mut self, start: u32, size: u32, total: u32) {
        self.range = (self.range as u32).wrapping_div(total) as u32;
        self.low = (self.low as u32).wrapping_add(start.wrapping_mul(self.range)) as u32;
        self.range = (self.range as u32).wrapping_mul(size) as u32;
        self.range_enc_normalize();
    }
    unsafe fn range_enc_encode_bit_0(&mut self, size0: u32) {
        self.range >>= 14 as i32;
        self.range = (self.range as u32).wrapping_mul(size0) as u32;
        self.range_enc_normalize();
    }
    unsafe fn range_enc_encode_bit_1(&mut self, size0: u32) {
        self.range >>= 14 as i32;
        self.low = (self.low as u32).wrapping_add(size0.wrapping_mul(self.range)) as u32 as u32;
        self.range = (self.range as u32)
            .wrapping_mul((((1 as i32) << 14 as i32) as u32).wrapping_sub(size0))
            as u32 as u32;
        self.range_enc_normalize();
    }

    pub unsafe fn encode_symbol(&mut self, symbol: i32) {
        let mut char_mask: [u64; 32] = [0; 32];
        if (*self.min_context).num_stats as i32 != 0 as i32 {
            let mut s: *mut CPpmd_State = self.base.offset((*self.min_context).stats as isize)
                as *mut libc::c_void as *mut CPpmd_State;
            let mut sum: u32;
            let mut i: u32;
            if (*s).symbol as i32 == symbol {
                self.range_enc_encode(0, (*s).freq as u32, (*self.min_context).summ_freq as u32);
                self.found_state = s;
                self.update1_0();
                return;
            }
            self.prev_success = 0 as i32 as u32;
            sum = (*s).freq as u32;
            i = (*self.min_context).num_stats as u32;
            loop {
                s = s.offset(1);
                if (*s).symbol as i32 == symbol {
                    self.range_enc_encode(
                        sum,
                        (*s).freq as u32,
                        (*self.min_context).summ_freq as u32,
                    );
                    self.found_state = s;
                    self.update1();
                    return;
                }
                sum = (sum as u32).wrapping_add((*s).freq as u32) as u32 as u32;
                i = i.wrapping_sub(1);
                if !(i != 0) {
                    break;
                }
            }
            let mut z: u64;
            z = 0 as i32 as u64;
            while z < (256 as i32 as u64).wrapping_div(::std::mem::size_of::<u64>() as u64) {
                char_mask[z.wrapping_add(0 as i32 as u64) as usize] = !(0 as i32 as u64);
                char_mask[z.wrapping_add(1 as i32 as u64) as usize] =
                    char_mask[z.wrapping_add(0 as i32 as u64) as usize];
                char_mask[z.wrapping_add(2 as i32 as u64) as usize] =
                    char_mask[z.wrapping_add(1 as i32 as u64) as usize];
                char_mask[z.wrapping_add(3 as i32 as u64) as usize] =
                    char_mask[z.wrapping_add(2 as i32 as u64) as usize];
                char_mask[z.wrapping_add(4 as i32 as u64) as usize] =
                    char_mask[z.wrapping_add(3 as i32 as u64) as usize];
                char_mask[z.wrapping_add(5 as i32 as u64) as usize] =
                    char_mask[z.wrapping_add(4 as i32 as u64) as usize];
                char_mask[z.wrapping_add(6 as i32 as u64) as usize] =
                    char_mask[z.wrapping_add(5 as i32 as u64) as usize];
                char_mask[z.wrapping_add(7 as i32 as u64) as usize] =
                    char_mask[z.wrapping_add(6 as i32 as u64) as usize];
                z = (z as u64).wrapping_add(8 as i32 as u64) as u64 as u64
            }
            *(char_mask.as_mut_ptr() as *mut libc::c_schar).offset((*s).symbol as isize) =
                0 as i32 as libc::c_schar;
            i = (*self.min_context).num_stats as u32;
            loop {
                s = s.offset(-1);
                *(char_mask.as_mut_ptr() as *mut libc::c_schar).offset((*s).symbol as isize) =
                    0 as i32 as libc::c_schar;
                i = i.wrapping_sub(1);
                if !(i != 0) {
                    break;
                }
            }
            self.range_enc_encode(
                sum,
                ((*self.min_context).summ_freq as u32).wrapping_sub(sum),
                (*self.min_context).summ_freq as u32,
            );
        } else {
            let prob: *mut u16 = &mut *(*self.bin_summ.as_mut_ptr().offset(
                *self.ns2indx.as_mut_ptr().offset(
                    ((*(&mut (*self.min_context).summ_freq as *mut u16 as *mut CPpmd_State)).freq
                        as u64)
                        .wrapping_sub(1 as i32 as u64) as isize,
                ) as isize,
            ))
            .as_mut_ptr()
            .offset(
                (*self.ns2bsindx.as_mut_ptr().offset(
                    (*(self.base.offset((*self.min_context).suffix as isize) as *mut libc::c_void
                        as *mut CPpmd8Context))
                        .num_stats as isize,
                ) as u32)
                    .wrapping_add(self.prev_success)
                    .wrapping_add((*self.min_context).flags as u32)
                    .wrapping_add((self.run_length >> 26 as i32 & 0x20 as i32) as u32)
                    as isize,
            ) as *mut u16;
            let s_0: *mut CPpmd_State =
                &mut (*self.min_context).summ_freq as *mut u16 as *mut CPpmd_State;
            if (*s_0).symbol as i32 == symbol {
                self.range_enc_encode_bit_0(*prob as u32);
                *prob = (*prob as i32 + ((1 as i32) << 7 as i32)
                    - (*prob as i32 + ((1 as i32) << 7 as i32 - 2 as i32) >> 7 as i32))
                    as u16;
                self.found_state = s_0;
                self.update_bin();
                return;
            } else {
                self.range_enc_encode_bit_1(*prob as u32);
                *prob = (*prob as i32
                    - (*prob as i32 + ((1 as i32) << 7 as i32 - 2 as i32) >> 7 as i32))
                    as u16;
                self.init_esc = PPMD8_K_EXP_ESCAPE[(*prob as i32 >> 10 as i32) as usize] as u32;
                let mut z_0: u64 = 0;
                while z_0 < (256 as i32 as u64).wrapping_div(::std::mem::size_of::<u64>() as u64) {
                    char_mask[z_0.wrapping_add(0) as usize] = !(0);
                    char_mask[z_0.wrapping_add(1) as usize] =
                        char_mask[z_0.wrapping_add(0) as usize];
                    char_mask[z_0.wrapping_add(2) as usize] =
                        char_mask[z_0.wrapping_add(1) as usize];
                    char_mask[z_0.wrapping_add(3) as usize] =
                        char_mask[z_0.wrapping_add(2) as usize];
                    char_mask[z_0.wrapping_add(4) as usize] =
                        char_mask[z_0.wrapping_add(3) as usize];
                    char_mask[z_0.wrapping_add(5) as usize] =
                        char_mask[z_0.wrapping_add(4) as usize];
                    char_mask[z_0.wrapping_add(6) as usize] =
                        char_mask[z_0.wrapping_add(5) as usize];
                    char_mask[z_0.wrapping_add(7) as usize] =
                        char_mask[z_0.wrapping_add(6) as usize];
                    z_0 = (z_0 as u64).wrapping_add(8)
                }
                *(char_mask.as_mut_ptr() as *mut libc::c_schar).offset((*s_0).symbol as isize) =
                    0 as i32 as libc::c_schar;
                self.prev_success = 0 as i32 as u32
            }
        }
        loop {
            let mut esc_freq: u32 = 0;
            let mut see;
            let mut s_1;
            let mut sum_0: u32;
            let mut i_0: u32;
            let num_masked: u32 = (*self.min_context).num_stats as u32;
            loop {
                self.order_fall = self.order_fall.wrapping_add(1);
                if (*self.min_context).suffix == 0 {
                    return;
                }
                self.min_context = self.base.offset((*self.min_context).suffix as isize)
                    as *mut libc::c_void as *mut CPpmd8Context;
                if !((*self.min_context).num_stats as u32 == num_masked) {
                    break;
                }
            }
            see = self.make_esc_freq(num_masked, &mut esc_freq);
            s_1 = self.base.offset((*self.min_context).stats as isize) as *mut libc::c_void
                as *mut CPpmd_State;
            sum_0 = 0 as i32 as u32;
            i_0 = ((*self.min_context).num_stats as i32 + 1 as i32) as u32;
            loop {
                let cur: i32 = (*s_1).symbol as i32;
                if cur == symbol {
                    let low: u32 = sum_0;
                    let s1: *mut CPpmd_State = s_1;
                    loop {
                        sum_0 = (sum_0 as u32).wrapping_add(
                            ((*s_1).freq as i32
                                & *(char_mask.as_mut_ptr() as *mut libc::c_schar)
                                    .offset((*s_1).symbol as isize)
                                    as i32) as u32,
                        ) as u32 as u32;
                        s_1 = s_1.offset(1);
                        i_0 = i_0.wrapping_sub(1);
                        if !(i_0 != 0) {
                            break;
                        }
                    }
                    self.range_enc_encode(low, (*s1).freq as u32, sum_0.wrapping_add(esc_freq));
                    if ((*see).shift as i32) < 7 as i32 && {
                        (*see).count = (*see).count.wrapping_sub(1);
                        ((*see).count as i32) == 0 as i32
                    } {
                        (*see).summ = (((*see).summ as i32) << 1 as i32) as u16;
                        let fresh0 = (*see).shift;
                        (*see).shift = (*see).shift.wrapping_add(1);
                        (*see).count = ((3 as i32) << fresh0 as i32) as libc::c_uchar
                    }
                    self.found_state = s1;
                    self.update2();
                    return;
                }
                sum_0 = (sum_0 as u32).wrapping_add(
                    ((*s_1).freq as i32
                        & *(char_mask.as_mut_ptr() as *mut libc::c_schar).offset(cur as isize)
                            as i32) as u32,
                ) as u32 as u32;
                *(char_mask.as_mut_ptr() as *mut libc::c_schar).offset(cur as isize) =
                    0 as i32 as libc::c_schar;
                s_1 = s_1.offset(1);
                i_0 = i_0.wrapping_sub(1);
                if !(i_0 != 0) {
                    break;
                }
            }
            self.range_enc_encode(sum_0, esc_freq, sum_0.wrapping_add(esc_freq));
            (*see).summ = ((*see).summ as u32)
                .wrapping_add(sum_0)
                .wrapping_add(esc_freq) as u16
        }
    }

    /* ---------- Decode ---------- */
    unsafe fn range_dec_get_threshold(&mut self, total: u32) -> u32 {
        self.range = (self.range as u32).wrapping_div(total) as u32;
        self.code.wrapping_div(self.range)
    }
    unsafe fn range_dec_decode(&mut self, mut start: u32, size: u32) {
        start = (start as u32).wrapping_mul(self.range) as u32;
        self.low = (self.low as u32).wrapping_add(start) as u32;
        self.code = (self.code as u32).wrapping_sub(start) as u32;
        self.range = (self.range as u32).wrapping_mul(size) as u32;
        while self.low ^ self.low.wrapping_add(self.range) < ((1 as i32) << 24 as i32) as u32
            || self.range < ((1 as i32) << 15 as i32) as u32 && {
                self.range = (0 as i32 as u32).wrapping_sub(self.low)
                    & (((1 as i32) << 15 as i32) - 1 as i32) as u32;
                (1 as i32) != 0
            }
        {
            self.code = self.code << 8 as i32
                | (*self.stream.r#in).read.expect("non-null function pointer")(self.stream.r#in)
                    as u32;
            self.range <<= 8 as i32;
            self.low <<= 8 as i32
        }
    }

    pub unsafe fn decode_symbol(&mut self) -> i32 {
        let mut char_mask: [u64; 32] = [0; 32];
        if (*self.min_context).num_stats as i32 != 0 {
            let mut s: *mut CPpmd_State = self.base.offset((*self.min_context).stats as isize)
                as *mut libc::c_void as *mut CPpmd_State;
            let mut i: u32;
            let mut hi_cnt: u32;
            let count = self.range_dec_get_threshold((*self.min_context).summ_freq as u32);
            hi_cnt = (*s).freq as u32;
            if count < hi_cnt {
                self.range_dec_decode(0, (*s).freq as u32);
                self.found_state = s;
                let symbol = (*s).symbol;
                self.update1_0();
                return symbol as i32;
            }
            self.prev_success = 0 as i32 as u32;
            i = (*self.min_context).num_stats as u32;
            loop {
                s = s.offset(1);
                hi_cnt = (hi_cnt as u32).wrapping_add((*s).freq as u32) as u32;
                if hi_cnt > count {
                    self.range_dec_decode(hi_cnt.wrapping_sub((*s).freq as u32), (*s).freq as u32);
                    self.found_state = s;
                    let symbol_0 = (*s).symbol;
                    self.update1();
                    return symbol_0 as i32;
                }
                i = i.wrapping_sub(1);
                if !(i != 0) {
                    break;
                }
            }
            if count >= (*self.min_context).summ_freq as u32 {
                return -(2 as i32);
            }
            self.range_dec_decode(
                hi_cnt,
                ((*self.min_context).summ_freq as u32).wrapping_sub(hi_cnt),
            );
            let mut z: u64 = 0;
            while z < (256 as i32 as u64).wrapping_div(::std::mem::size_of::<u64>() as u64) {
                char_mask[z.wrapping_add(0) as usize] = !(0 as i32 as u64);
                char_mask[z.wrapping_add(1) as usize] = char_mask[z.wrapping_add(0) as usize];
                char_mask[z.wrapping_add(2) as usize] = char_mask[z.wrapping_add(1) as usize];
                char_mask[z.wrapping_add(3) as usize] = char_mask[z.wrapping_add(2) as usize];
                char_mask[z.wrapping_add(4) as usize] = char_mask[z.wrapping_add(3) as usize];
                char_mask[z.wrapping_add(5) as usize] = char_mask[z.wrapping_add(4) as usize];
                char_mask[z.wrapping_add(6) as usize] = char_mask[z.wrapping_add(5) as usize];
                char_mask[z.wrapping_add(7) as usize] = char_mask[z.wrapping_add(6) as usize];
                z = z.wrapping_add(8)
            }
            *(char_mask.as_mut_ptr() as *mut libc::c_schar).offset((*s).symbol as isize) =
                0 as i32 as libc::c_schar;
            i = (*self.min_context).num_stats as u32;
            loop {
                s = s.offset(-1);
                *(char_mask.as_mut_ptr() as *mut libc::c_schar).offset((*s).symbol as isize) =
                    0 as i32 as libc::c_schar;
                i = i.wrapping_sub(1);
                if !(i != 0) {
                    break;
                }
            }
        } else {
            let prob: *mut u16 = &mut *(*self.bin_summ.as_mut_ptr().offset(
                *self.ns2indx.as_mut_ptr().offset(
                    ((*(&mut (*self.min_context).summ_freq as *mut u16 as *mut CPpmd_State)).freq
                        as u64)
                        .wrapping_sub(1 as i32 as u64) as isize,
                ) as isize,
            ))
            .as_mut_ptr()
            .offset(
                (*self.ns2bsindx.as_mut_ptr().offset(
                    (*(self.base.offset((*self.min_context).suffix as isize) as *mut libc::c_void
                        as *mut CPpmd8Context))
                        .num_stats as isize,
                ) as u32)
                    .wrapping_add(self.prev_success)
                    .wrapping_add((*self.min_context).flags as u32)
                    .wrapping_add((self.run_length >> 26 as i32 & 0x20 as i32) as u32)
                    as isize,
            ) as *mut u16;
            self.range >>= 14 as i32;
            if self.code.wrapping_div(self.range) < *prob as u32 {
                self.range_dec_decode(0, *prob as u32);
                *prob = (*prob as i32 + ((1 as i32) << 7 as i32)
                    - (*prob as i32 + ((1 as i32) << 7 as i32 - 2 as i32) >> 7 as i32))
                    as u16;
                self.found_state =
                    &mut (*self.min_context).summ_freq as *mut u16 as *mut CPpmd_State;
                let symbol_1 = (*self.found_state).symbol;
                self.update_bin();
                return symbol_1 as i32;
            }
            self.range_dec_decode(
                *prob as u32,
                (((1 as i32) << 14 as i32) - *prob as i32) as u32,
            );
            *prob = (*prob as i32
                - (*prob as i32 + ((1 as i32) << 7 as i32 - 2 as i32) >> 7 as i32))
                as u16;
            self.init_esc = PPMD8_K_EXP_ESCAPE[(*prob as i32 >> 10 as i32) as usize] as u32;
            let mut z_0: u64 = 0;
            while z_0 < (256 as u64).wrapping_div(::std::mem::size_of::<u64>() as u64) {
                char_mask[z_0.wrapping_add(0) as usize] = !(0);
                char_mask[z_0.wrapping_add(1) as usize] = char_mask[z_0.wrapping_add(0) as usize];
                char_mask[z_0.wrapping_add(2) as usize] = char_mask[z_0.wrapping_add(1) as usize];
                char_mask[z_0.wrapping_add(3) as usize] = char_mask[z_0.wrapping_add(2) as usize];
                char_mask[z_0.wrapping_add(4) as usize] = char_mask[z_0.wrapping_add(3) as usize];
                char_mask[z_0.wrapping_add(5) as usize] = char_mask[z_0.wrapping_add(4) as usize];
                char_mask[z_0.wrapping_add(6) as usize] = char_mask[z_0.wrapping_add(5) as usize];
                char_mask[z_0.wrapping_add(7) as usize] = char_mask[z_0.wrapping_add(6) as usize];
                z_0 = z_0.wrapping_add(8)
            }
            *(char_mask.as_mut_ptr() as *mut libc::c_schar).offset(
                (*(&mut (*self.min_context).summ_freq as *mut u16 as *mut CPpmd_State)).symbol
                    as isize,
            ) = 0 as i32 as libc::c_schar;
            self.prev_success = 0 as i32 as u32
        }
        loop {
            let mut ps: [*mut CPpmd_State; 256] = [0 as *mut CPpmd_State; 256];
            let mut s_0;
            let mut freq_sum: u32 = 0;
            let count_0: u32;
            let mut hi_cnt_0: u32;
            let mut see;
            let num_masked: u32 = (*self.min_context).num_stats as u32;
            loop {
                self.order_fall = self.order_fall.wrapping_add(1);
                if (*self.min_context).suffix == 0 {
                    return -(1 as i32);
                }
                self.min_context = self.base.offset((*self.min_context).suffix as isize)
                    as *mut libc::c_void as *mut CPpmd8Context;
                if !((*self.min_context).num_stats as u32 == num_masked) {
                    break;
                }
            }
            hi_cnt_0 = 0;
            s_0 = self.base.offset((*self.min_context).stats as isize) as *mut libc::c_void
                as *mut CPpmd_State;
            let mut i_0 = 0u32;
            let num = ((*self.min_context).num_stats as u32).wrapping_sub(num_masked);
            loop {
                let k: i32 = *(char_mask.as_mut_ptr() as *mut libc::c_schar)
                    .offset((*s_0).symbol as isize) as i32;
                hi_cnt_0 =
                    (hi_cnt_0 as u32).wrapping_add(((*s_0).freq as i32 & k) as u32) as u32 as u32;
                let fresh0 = s_0;
                s_0 = s_0.offset(1);
                ps[i_0 as usize] = fresh0;
                i_0 = i_0.wrapping_sub(k as u32);
                if !(i_0 != num) {
                    break;
                }
            }
            see = self.make_esc_freq(num_masked, &mut freq_sum);
            freq_sum = (freq_sum as u32).wrapping_add(hi_cnt_0) as u32 as u32;
            count_0 = self.range_dec_get_threshold(freq_sum);
            if count_0 < hi_cnt_0 {
                let mut pps: *mut *mut CPpmd_State = ps.as_mut_ptr();
                hi_cnt_0 = 0 as i32 as u32;
                loop {
                    hi_cnt_0 = (hi_cnt_0 as u32).wrapping_add((**pps).freq as u32) as u32 as u32;
                    if !(hi_cnt_0 <= count_0) {
                        break;
                    }
                    pps = pps.offset(1)
                }
                s_0 = *pps;
                self.range_dec_decode(
                    hi_cnt_0.wrapping_sub((*s_0).freq as u32),
                    (*s_0).freq as u32,
                );
                if ((*see).shift as i32) < 7 as i32 && {
                    (*see).count = (*see).count.wrapping_sub(1);
                    ((*see).count as i32) == 0 as i32
                } {
                    (*see).summ = (((*see).summ as i32) << 1 as i32) as u16;
                    let fresh1 = (*see).shift;
                    (*see).shift = (*see).shift.wrapping_add(1);
                    (*see).count = ((3 as i32) << fresh1 as i32) as u8
                }
                self.found_state = s_0;
                let symbol_2 = (*s_0).symbol;
                self.update2();
                return symbol_2 as i32;
            }
            if count_0 >= freq_sum {
                return -(2 as i32);
            }
            self.range_dec_decode(hi_cnt_0, freq_sum.wrapping_sub(hi_cnt_0));
            (*see).summ = ((*see).summ as u32).wrapping_add(freq_sum) as u16;
            loop {
                i_0 = i_0.wrapping_sub(1);
                *(char_mask.as_mut_ptr() as *mut libc::c_schar)
                    .offset((*ps[i_0 as usize]).symbol as isize) = 0 as i32 as libc::c_schar;
                if !(i_0 != 0) {
                    break;
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub r#in: *mut IByteIn,
    pub out: *mut IByteOut,
}
pub type CtxPtr = *mut CPpmd8Context;
pub type Cppmd8Node = CPpmd8_Node_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd8_Node_ {
    pub stamp: u32,
    pub next: Cppmd8NodeRef,
    pub nu: u32,
}
pub type Cppmd8NodeRef = u32;
/* Ppmd8.c -- PPMdI codec
2017-04-03 : Igor Pavlov : Public domain
This code is based on PPMd var.I (2002): Dmitry Shkarin : Public domain */
#[no_mangle]
pub static mut PPMD8_K_EXP_ESCAPE: [u8; 16] = [25, 14, 9, 7, 5, 5, 4, 4, 4, 3, 3, 3, 2, 2, 2, 2];
static mut K_INIT_BIN_ESC: [u16; 8] = [
    0x3cdd, 0x1f3f, 0x59bf, 0x48f3, 0x64a1, 0x5abc, 0x6632, 0x6051,
];

/* The BUG in Shkarin's code for FREEZE mode was fixed, but that fixed
code is not compatible with original code for some files compressed
in FREEZE mode. So we disable FREEZE mode support. */
/* must be 32-bit at least */
/* range Coder */

/* H->I changes:
  ns2indx
  GlewCount, and Glue method
  BinSum
  see / EscFreq
  create_successors updates more suffix contexts
  update_model consts.
  prev_success Update
*/

unsafe extern "C" fn pmalloc(_: ISzAllocPtr, size: u64) -> *mut libc::c_void {
    return malloc(size.try_into().unwrap()); /* EndMark */
}
unsafe extern "C" fn pfree(_: ISzAllocPtr, addr: *mut libc::c_void) {
    free(addr);
}
pub static mut IALLOC: ISzAlloc = {
    {
        let init = ISzAlloc {
            alloc: Some(
                pmalloc as unsafe extern "C" fn(_: ISzAllocPtr, _: u64) -> *mut libc::c_void,
            ),
            free: Some(pfree as unsafe extern "C" fn(_: ISzAllocPtr, _: *mut libc::c_void) -> ()),
        };
        init
    }
};

unsafe fn set_successor(mut p: *mut CPpmd_State, v: CPpmdVoidRef) {
    (*p).successor_low = (v & 0xffff as i32 as u32) as u16;
    (*p).successor_high = (v >> 16 as i32 & 0xffff as i32 as u32) as u16;
}

unsafe fn swap_states(t1: *mut CPpmd_State, t2: *mut CPpmd_State) {
    let tmp: CPpmd_State = *t1;
    *t1 = *t2;
    *t2 = tmp;
}
