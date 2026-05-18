//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 575/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk575<F: Float>(t135: F, t999: F, t973: F, t2770: F, t2978: F, t2775: F, t976: F, t1005: F, t1036: F, t221: F, t2965: F, t339: F) -> (F, F, F, F, F, F) {
    let t3139 = t135 * t999;
    let t3140 = t973 * t3139;
    let t3146 = t2978 * t2770;
    let t3151 = t976 * t2775;
    let t3156 = t1005 * t1036;
    let t3158 = t221 * t2965;
    let t3160 = t339 * t3158 / F::new(432.0);
    (t3140, t3146, t3151, t3156, t3158, t3160)
}
