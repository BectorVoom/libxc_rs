//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2303/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2303<F: Float>(t67064: F, t67082: F, t157: F, t182: F, t46130: F, t57887: F, t46132: F, t46134: F, t57897: F, t40667: F, t40682: F, t172: F, t20742: F, t763: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t67083 = t67064 + t67082;
    let t67086 = F::cast_from(0.19751673498613801407e-1_f64) * t67083 * t157 * t182;
    let t67087 = F::cast_from(0.15584273195113317383e3_f64) * t46130;
    let t67088 = F::new(3.0) * t57887;
    let t67089 = F::cast_from(0.97592231702715658578e-1_f64) * t46132;
    let t67090 = F::cast_from(0.14447919941302971323e1_f64) * t46134;
    let t67095 = F::new(3.0) * t57897;
    let t67096 = F::cast_from(0.51947577317044391277e2_f64) * t40667;
    let t67097 = F::cast_from(0.35089341735807877242e1_f64) * t40682;
    let t67099 = t20742 * t172 * t763;
    (t67083, t67086, t67087, t67088, t67089, t67090, t67095, t67096, t67097, t67099)
}
