//! MMX image filters

extern crate c_vec;

use std::mem;
use std::ptr::Unique;
use libc;
use libc::{size_t, c_void, c_uint, c_int};
use sdl2::SdlResult;
use self::c_vec::CVec;

mod ll {
    /* automatically generated by rust-bindgen */

    use libc::*;
    extern "C" {
        pub fn SDL_imageFilterMMXdetect() -> c_int;
        pub fn SDL_imageFilterMMXoff();
        pub fn SDL_imageFilterMMXon();
        pub fn SDL_imageFilterAdd(Src1: *mut u8, Src2: *mut u8,
                                  Dest: *mut u8, length: c_uint) -> c_int;
        pub fn SDL_imageFilterMean(Src1: *mut u8, Src2: *mut u8,
                                   Dest: *mut u8, length: c_uint) -> c_int;
        pub fn SDL_imageFilterSub(Src1: *mut u8, Src2: *mut u8,
                                  Dest: *mut u8, length: c_uint) -> c_int;
        pub fn SDL_imageFilterAbsDiff(Src1: *mut u8, Src2: *mut u8,
                                      Dest: *mut u8, length: c_uint) ->
            c_int;
        pub fn SDL_imageFilterMult(Src1: *mut u8, Src2: *mut u8,
                                   Dest: *mut u8, length: c_uint) -> c_int;
        pub fn SDL_imageFilterMultNor(Src1: *mut u8, Src2: *mut u8,
                                      Dest: *mut u8, length: c_uint) ->
            c_int;
        pub fn SDL_imageFilterMultDivby2(Src1: *mut u8, Src2: *mut u8,
                                         Dest: *mut u8, length: c_uint) ->
            c_int;
        pub fn SDL_imageFilterMultDivby4(Src1: *mut u8, Src2: *mut u8,
                                         Dest: *mut u8, length: c_uint) ->
            c_int;
        pub fn SDL_imageFilterBitAnd(Src1: *mut u8, Src2: *mut u8,
                                     Dest: *mut u8, length: c_uint) -> c_int;
        pub fn SDL_imageFilterBitOr(Src1: *mut u8, Src2: *mut u8,
                                    Dest: *mut u8, length: c_uint) -> c_int;
        pub fn SDL_imageFilterDiv(Src1: *mut u8, Src2: *mut u8,
                                  Dest: *mut u8, length: c_uint) -> c_int;
        pub fn SDL_imageFilterBitNegation(Src1: *mut u8, Dest: *mut u8,
                                          length: c_uint) -> c_int;
        pub fn SDL_imageFilterAddByte(Src1: *mut u8, Dest: *mut u8,
                                      length: c_uint, C: u8) -> c_int;
        pub fn SDL_imageFilterAddUint(Src1: *mut u8, Dest: *mut u8,
                                      length: c_uint, C: c_uint) -> c_int;
        pub fn SDL_imageFilterAddByteToHalf(Src1: *mut u8,
                                            Dest: *mut u8, length: c_uint,
                                            C: u8) -> c_int;
        pub fn SDL_imageFilterSubByte(Src1: *mut u8, Dest: *mut u8,
                                      length: c_uint, C: u8) -> c_int;
        pub fn SDL_imageFilterSubUint(Src1: *mut u8, Dest: *mut u8,
                                      length: c_uint, C: c_uint) -> c_int;
        pub fn SDL_imageFilterShiftRight(Src1: *mut u8, Dest: *mut u8,
                                         length: c_uint, N: u8) -> c_int;
        pub fn SDL_imageFilterShiftRightUint(Src1: *mut u8,
                                             Dest: *mut u8, length: c_uint,
                                             N: u8) -> c_int;
        pub fn SDL_imageFilterMultByByte(Src1: *mut u8, Dest: *mut u8,
                                         length: c_uint, C: u8) -> c_int;
        pub fn SDL_imageFilterShiftRightAndMultByByte(Src1: *mut u8,
                                                      Dest: *mut u8,
                                                      length: c_uint, N: u8,
                                                      C: u8) -> c_int;
        pub fn SDL_imageFilterShiftLeftByte(Src1: *mut u8,
                                            Dest: *mut u8, length: c_uint,
                                            N: u8) -> c_int;
        pub fn SDL_imageFilterShiftLeftUint(Src1: *mut u8,
                                            Dest: *mut u8, length: c_uint,
                                            N: u8) -> c_int;
        pub fn SDL_imageFilterShiftLeft(Src1: *mut u8, Dest: *mut u8,
                                        length: c_uint, N: u8) -> c_int;
        pub fn SDL_imageFilterBinarizeUsingThreshold(Src1: *mut u8,
                                                     Dest: *mut u8,
                                                     length: c_uint, T: u8)
                                                     -> c_int;
        pub fn SDL_imageFilterClipToRange(Src1: *mut u8, Dest: *mut u8,
                                          length: c_uint, Tmin: u8,
                                          Tmax: u8) -> c_int;
        pub fn SDL_imageFilterNormalizeLinear(Src: *mut u8,
                                              Dest: *mut u8, length: c_uint,
                                              Cmin: c_int, Cmax: c_int,
                                              Nmin: c_int, Nmax: c_int) -> c_int;
    }
}

/// MMX detection routine (with override flag).
pub fn mmx_detect() -> bool {
    unsafe { ll::SDL_imageFilterMMXdetect() == 1 }
}

/// Disable MMX check for filter functions and and force to use non-MMX C based code.
pub fn mmx_off() {
    unsafe { ll::SDL_imageFilterMMXoff() }
}

/// Enable MMX check for filter functions and use MMX code if available.
pub fn mmx_on() {
    unsafe { ll::SDL_imageFilterMMXon() }
}

#[inline]
fn cvec_with_size(sz: usize) -> CVec<u8> {
    unsafe {
        let p = libc::malloc(sz as size_t) as *mut u8;
        CVec::new_with_dtor(Unique::new(p), sz, move |p| {
            libc::free(p as *mut c_void)
        })
    }
}

/// Filter using Add: D = saturation255(S1 + S2).
pub fn add(src1: CVec<u8>, src2: CVec<u8>) -> SdlResult<CVec<u8>> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterAdd(mem::transmute(src1.get(0)),
                                              mem::transmute(src2.get(0)),
                                              mem::transmute(dest.get(0)),
                                              size as c_uint) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterAdd error".to_string()) }
}

/// Filter using Mean: D = S1/2 + S2/2.
pub fn mean(src1: CVec<u8>, src2: CVec<u8>) -> SdlResult<CVec<u8>> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterMean(mem::transmute(src1.get(0)),
                                               mem::transmute(src2.get(0)),
                                               mem::transmute(dest.get(0)),
                                               size as c_uint) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterMean error".to_string()) }
}

/// Filter using Sub: D = saturation0(S1 - S2).
pub fn sub(src1: CVec<u8>, src2: CVec<u8>) -> SdlResult<CVec<u8>> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterSub(mem::transmute(src1.get(0)),
                                              mem::transmute(src2.get(0)),
                                              mem::transmute(dest.get(0)),
                                              size as c_uint) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterSub error".to_string()) }
}

/// Filter using AbsDiff: D = | S1 - S2 |.
pub fn abs_diff(src1: CVec<u8>, src2: CVec<u8>) -> SdlResult<CVec<u8>> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterAbsDiff(mem::transmute(src1.get(0)),
                                                  mem::transmute(src2.get(0)),
                                                  mem::transmute(dest.get(0)),
                                                  size as c_uint) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterAbsDiff error".to_string()) }
}

/// Filter using Mult: D = saturation255(S1 * S2).
pub fn mult(src1: CVec<u8>, src2: CVec<u8>) -> SdlResult<CVec<u8>> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterMult(mem::transmute(src1.get(0)),
                                               mem::transmute(src2.get(0)),
                                               mem::transmute(dest.get(0)),
                                               size as c_uint) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterMult error".to_string()) }
}

/// Filter using MultNor: D = S1 * S2.
pub fn mult_nor(src1: CVec<u8>, src2: CVec<u8>) -> SdlResult<CVec<u8>> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterMultNor(mem::transmute(src1.get(0)),
                                                  mem::transmute(src2.get(0)),
                                                  mem::transmute(dest.get(0)),
                                                  size as c_uint) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterMultNor error".to_string()) }
}

/// Filter using MultDivby2: D = saturation255(S1/2 * S2).
pub fn mult_div_by2(src1: CVec<u8>, src2: CVec<u8>) -> SdlResult<CVec<u8>> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterMultDivby2(mem::transmute(src1.get(0)),
                                                     mem::transmute(src2.get(0)),
                                                     mem::transmute(dest.get(0)),
                                                     size as c_uint) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterMultDivby2 error".to_string()) }
}

/// Filter using MultDivby4: D = saturation255(S1/2 * S2/2).
pub fn mult_div_by4(src1: CVec<u8>, src2: CVec<u8>) -> SdlResult<CVec<u8>> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterMultDivby4(mem::transmute(src1.get(0)),
                                                     mem::transmute(src2.get(0)),
                                                     mem::transmute(dest.get(0)),
                                                     size as c_uint) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterMultDivby4 error".to_string()) }
}

/// Filter using BitAnd: D = S1 & S2.
pub fn bit_and(src1: CVec<u8>, src2: CVec<u8>) -> SdlResult<CVec<u8>> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterBitAnd(mem::transmute(src1.get(0)),
                                                 mem::transmute(src2.get(0)),
                                                 mem::transmute(dest.get(0)),
                                                 size as c_uint) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterBitAnd error".to_string()) }
}

/// Filter using BitOr: D = S1 | S2.
pub fn bit_or(src1: CVec<u8>, src2: CVec<u8>) -> SdlResult<CVec<u8>> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterBitOr(mem::transmute(src1.get(0)),
                                                mem::transmute(src2.get(0)),
                                                mem::transmute(dest.get(0)),
                                                size as c_uint) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterBitOr error".to_string()) }
}

/// Filter using Div: D = S1 / S2.
pub fn div(src1: CVec<u8>, src2: CVec<u8>) -> SdlResult<CVec<u8>> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterDiv(mem::transmute(src1.get(0)),
                                              mem::transmute(src2.get(0)),
                                              mem::transmute(dest.get(0)),
                                              size as c_uint) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterDiv error".to_string()) }
}

/// Filter using BitNegation: D = !S.
pub fn bit_negation(src1: CVec<u8>) -> SdlResult<CVec<u8>> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterBitNegation(mem::transmute(src1.get(0)),
                                                      mem::transmute(dest.get(0)),
                                                      size as c_uint) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterBitNegation error".to_string()) }
}

/// Filter using AddByte: D = saturation255(S + C).
pub fn add_byte(src1: CVec<u8>, c: u8) -> SdlResult<CVec<u8>> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterAddByte(mem::transmute(src1.get(0)),
                                                  mem::transmute(dest.get(0)),
                                                  size as c_uint, c) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterAddByte error".to_string()) }
}

/// Filter using AddUint: D = saturation255((S[i] + Cs[i % 4]), Cs=Swap32((uint)C).
pub fn add_uint(src1: CVec<u8>, c: u32) -> SdlResult<CVec<u8>> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterAddUint(mem::transmute(src1.get(0)),
                                                  mem::transmute(dest.get(0)),
                                                  size as c_uint, c) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterAddUint error".to_string()) }
}

/// Filter using AddByteToHalf: D = saturation255(S/2 + C).
pub fn add_byte_to_half(src1: CVec<u8>, c: u8) -> SdlResult<CVec<u8>> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterAddByteToHalf(mem::transmute(src1.get(0)),
                                                        mem::transmute(dest.get(0)),
                                                        size as c_uint, c) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterAddByteToHalf error".to_string()) }
}

/// Filter using SubByte: D = saturation0(S - C).
pub fn sub_byte(src1: CVec<u8>, c: u8) -> SdlResult<CVec<u8>> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterSubByte(mem::transmute(src1.get(0)),
                                                  mem::transmute(dest.get(0)),
                                                  size as c_uint, c) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterSubByte error".to_string()) }
}

/// Filter using SubUint: D = saturation0(S[i] - Cs[i % 4]), Cs=Swap32((uint)C).
pub fn sub_uint(src1: CVec<u8>, c: u32) -> SdlResult<CVec<u8>> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterSubUint(mem::transmute(src1.get(0)),
                                                  mem::transmute(dest.get(0)),
                                                  size as c_uint, c) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterSubUint error".to_string()) }
}

/// Filter using ShiftRight: D = saturation0(S >> N).
pub fn shift_right(src1: CVec<u8>, n: u8) -> SdlResult<CVec<u8>> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterShiftRight(mem::transmute(src1.get(0)),
                                                     mem::transmute(dest.get(0)),
                                                     size as c_uint, n) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterShiftRight error".to_string()) }
}

/// Filter using ShiftRightUint: D = saturation0((uint)S[i] >> N).
pub fn shift_right_uint(src1: CVec<u8>, n: u8) -> SdlResult<CVec<u8>> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterShiftRightUint(mem::transmute(src1.get(0)),
                                                         mem::transmute(dest.get(0)),
                                                         size as c_uint, n) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterShiftRightUint error".to_string()) }
}

/// Filter using MultByByte: D = saturation255(S * C).
pub fn mult_by_byte(src1: CVec<u8>, c: u8) -> SdlResult<CVec<u8>> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterMultByByte(mem::transmute(src1.get(0)),
                                                     mem::transmute(dest.get(0)),
                                                     size as c_uint, c) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterMultByByte error".to_string()) }
}

/// Filter using ShiftRightAndMultByByte: D = saturation255((S >> N) * C).
pub fn shift_right_and_mult_by_byte(src1: CVec<u8>, n: u8, c: u8) -> SdlResult<CVec<u8>> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterShiftRightAndMultByByte(mem::transmute(src1.get(0)),
                                                                  mem::transmute(dest.get(0)),
                                                                  size as c_uint, n, c) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterShiftRightAndMultByByte error".to_string()) }
}

/// Filter using ShiftLeftByte: D = (S << N).
pub fn shift_left_byte(src1: CVec<u8>, n: u8) -> SdlResult<CVec<u8>> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterShiftLeftByte(mem::transmute(src1.get(0)),
                                                        mem::transmute(dest.get(0)),
                                                        size as c_uint, n) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterShiftLeftByte error".to_string()) }
}

/// Filter using ShiftLeftUint: D = ((uint)S << N).
pub fn shift_left_uint(src1: CVec<u8>, n: u8) -> SdlResult<CVec<u8>> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterShiftLeftUint(mem::transmute(src1.get(0)),
                                                        mem::transmute(dest.get(0)),
                                                        size as c_uint, n) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterShiftLeftUint error".to_string()) }
}

/// Filter ShiftLeft: D = saturation255(S << N).
pub fn shift_left(src1: CVec<u8>, n: u8) -> SdlResult<CVec<u8>> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterShiftLeft(mem::transmute(src1.get(0)),
                                                    mem::transmute(dest.get(0)),
                                                    size as c_uint, n) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterShiftLeft error".to_string()) }
}

/// Filter using BinarizeUsingThreshold: D = (S >= T) ? 255:0.
pub fn binarize_using_threshold(src1: CVec<u8>, t: u8) -> SdlResult<CVec<u8>> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterBinarizeUsingThreshold(mem::transmute(src1.get(0)),
                                                                 mem::transmute(dest.get(0)),
                                                                 size as c_uint, t) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterBinarizeUsingThreshold error".to_string()) }
}

/// Filter using ClipToRange: D = (S >= Tmin) & (S <= Tmax) S:Tmin | Tmax.
pub fn clip_to_range(src1: CVec<u8>, tmin: u8, tmax: u8) -> SdlResult<CVec<u8>> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterClipToRange(mem::transmute(src1.get(0)),
                                                      mem::transmute(dest.get(0)),
                                                      size as c_uint, tmin, tmax) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterClipToRange error".to_string()) }
}

/// Filter using NormalizeLinear: D = saturation255((Nmax - Nmin)/(Cmax - Cmin)*(S - Cmin) + Nmin).
pub fn normalize_linear(src1: CVec<u8>, cmin: isize, cmax: isize, nmin: isize, nmax: isize) -> SdlResult<CVec<u8>> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe { ll::SDL_imageFilterNormalizeLinear(mem::transmute(src1.get(0)),
                                                          mem::transmute(dest.get(0)),
                                                          size as c_uint,
                                                          cmin as c_int, cmax as c_int,
                                                          nmin as c_int, nmax as c_int) };
    if ret == 0 { Ok(dest) }
    else { Err("SDL_imageFilterNormalizeLinear error".to_string()) }
}
