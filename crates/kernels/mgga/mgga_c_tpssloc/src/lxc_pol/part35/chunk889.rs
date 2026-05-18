//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 889/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk889<F: Float>(t14159: F, t973: F, t10868: F, t1539: F, t248: F, t1041: F, t1615: F, t3131: F, t1573: F, t2904: F, t1561: F, t2885: F) -> (F, F, F, F, F) {
    let t14160 = t973 * t14159;
    let t14202 = t248 * t10868 * t1539;
    let t14203 = t1041 * t14202;
    let t14211 = t1615 * t3131;
    let t14263 = t1573 * t2904;
    let t14271 = t1561 * t2885;
    (t14160, t14203, t14211, t14263, t14271)
}
