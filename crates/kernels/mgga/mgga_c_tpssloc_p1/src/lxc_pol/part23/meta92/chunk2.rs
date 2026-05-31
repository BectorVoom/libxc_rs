//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 525/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk525<F: Float>(t2770: F, t2978: F, t2775: F, t976: F, t221: F, t2965: F, t339: F, t1053: F, t386: F, t68: F, t3032: F, t3127: F) -> (F, F, F, F, F) {
    let t3146 = t2978 * t2770;
    let t3151 = t976 * t2775;
    let t3158 = t221 * t2965;
    let t3160 = t339 * t3158 / F::cast_from(432.0_f64);
    let t3173 = F::cast_from(1.0_f64) / t1053 / t386;
    let t3174 = t68 * t3173;
    let t3185 = t3032 * t3127;
    (t3146, t3151, t3160, t3174, t3185)
}
