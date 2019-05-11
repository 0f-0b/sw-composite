use crate::*;

fn pack_argb32(a: u32, r: u32, g: u32, b: u32) -> u32 {
    assert!(r <= a);
    assert!(g <= a);
    assert!(b <= a);

    return (a << A32_SHIFT) | (r << R32_SHIFT) |
        (g << G32_SHIFT) | (b << B32_SHIFT);
}

fn get_packed_a32(packed: u32) -> u32 { ((packed) << (24 - A32_SHIFT)) >> 24 }
fn get_packed_r32(packed: u32) -> u32 { ((packed) << (24 - R32_SHIFT)) >> 24 }
fn get_packed_g32(packed: u32) -> u32 { ((packed) << (24 - G32_SHIFT)) >> 24 }
fn get_packed_b32(packed: u32) -> u32 { ((packed) << (24 - B32_SHIFT)) >> 24 }

fn alpha_mul(x: u32, a: u32) -> u32 {
    let mask = 0xFF00FF;

    let src_rb = ((x & mask) * a) >> 8;
    let src_ag = ((x >> 8) & mask) * a;

    return (src_rb & mask) | (src_ag & !mask)
}

pub fn dst(_src: u32, dst: u32) -> u32 {
    dst
}

pub fn src(src: u32, _dst: u32) -> u32 {
    src
}

pub fn clear(_src: u32, _dst: u32) -> u32 {
    0
}

pub fn src_over(src: u32, dst: u32) -> u32 {
    over(src, dst)
}

pub fn dst_over(src: u32, dst: u32) -> u32 {
    over(dst, src)
}

pub fn src_in(src: u32, dst: u32) -> u32 {
    alpha_mul(src, alpha_to_alpha256(packed_alpha(dst)))
}

pub fn dst_in(src: u32, dst: u32) -> u32 {
    alpha_mul(dst, alpha_to_alpha256(packed_alpha(src)))
}

pub fn src_out(src: u32, dst: u32) -> u32 {
    alpha_mul(src, alpha_to_alpha256(255 - packed_alpha(dst)))
}

pub fn dst_out(src: u32, dst: u32) -> u32 {
    alpha_mul(dst, alpha_to_alpha256(255 - packed_alpha(src)))
}

pub fn src_atop(src: u32, dst: u32) -> u32 {
    let sa = packed_alpha(src);
    let da = packed_alpha(dst);
    let isa = 255 - sa;

    return pack_argb32(da,
                       muldiv255(da, get_packed_r32(src)) +
                           muldiv255(isa, get_packed_r32(dst)),
                       muldiv255(da, get_packed_g32(src)) +
                           muldiv255(isa, get_packed_g32(dst)),
                       muldiv255(da, get_packed_b32(src)) +
                           muldiv255(isa, get_packed_b32(dst)));
}

pub fn dst_atop(src: u32, dst: u32) -> u32 {
    let sa = packed_alpha(src);
    let da = packed_alpha(dst);
    let ida = 255 - da;

    return pack_argb32(sa,
                       muldiv255(ida, get_packed_r32(src)) +
                           muldiv255(sa, get_packed_r32(dst)),
                       muldiv255(ida, get_packed_g32(src)) +
                           muldiv255(sa, get_packed_g32(dst)),
                       muldiv255(ida, get_packed_b32(src)) +
                           muldiv255(sa, get_packed_b32(dst)));
}

pub fn xor(src: u32, dst: u32) -> u32 {
    let sa = packed_alpha(src);
    let da = packed_alpha(dst);
    let isa = 255 - da;
    let ida = 255 - da;

    return pack_argb32(sa + da - (muldiv255(sa, da) * 2),
                       muldiv255(ida, get_packed_r32(src)) +
                           muldiv255(isa, get_packed_r32(dst)),
                       muldiv255(ida, get_packed_g32(src)) +
                           muldiv255(isa, get_packed_g32(dst)),
                       muldiv255(ida, get_packed_b32(src)) +
                           muldiv255(isa, get_packed_b32(dst)));
}

fn saturated_add(a: u32, b: u32) -> u32 {
    debug_assert!(a <= 255);
    debug_assert!(b <= 255);
    let sum = a + b;
    if sum > 255 {
        255
    } else {
        sum
    }
}

pub fn add(src: u32, dst: u32) -> u32 {
    return pack_argb32(saturated_add(get_packed_a32(src), get_packed_a32(dst)),
                       saturated_add(get_packed_r32(src), get_packed_r32(dst)),
                       saturated_add(get_packed_g32(src), get_packed_g32(dst)),
                       saturated_add(get_packed_b32(src), get_packed_b32(dst)));
}