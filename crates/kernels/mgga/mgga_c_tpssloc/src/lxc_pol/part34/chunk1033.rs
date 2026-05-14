//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1033/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1033<F: Float>(t20947: F, t221: F, t25154: F, t20857: F, t6605: F, t9972: F, t20853: F, t815: F, t20944: F, t81959: F, t1894: F, t20756: F, t236: F, t81969: F, t20994: F, t6581: F) -> (F, F, F, F, F, F) {
    let t105345 = t25154 * t221 * t20947;
    let t105348 = t6605 * t9972 * t20857;
    let t105353 = t6605 * t815 * t20853;
    let t105366 = t81959 * t20944;
    let t105370 = t81969 * t1894 * t236 * t20756;
    let t105372 = t6581 * t20994;
    (t105345, t105348, t105353, t105366, t105370, t105372)
}
