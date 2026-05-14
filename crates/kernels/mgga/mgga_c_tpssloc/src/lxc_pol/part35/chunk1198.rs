//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1198/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1198<F: Float>(t8015: F, t94490: F, t24574: F, t29682: F, t29691: F, t29554: F, t1240: F, t6267: F, t2122: F, t29817: F, t3597: F, t6243: F, t6238: F, t7299: F, t7284: F, t29546: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t103286 = t94490 * t8015;
    let t103291 = t24574 * t29682;
    let t103293 = t24574 * t29691;
    let t103304 = t24574 * t29554;
    let t103314 = t1240 * t6267;
    let t103315 = t2122 * t103314;
    let t103332 = t24574 * t29817;
    let t103345 = t3597 * t6243;
    let t103363 = t7299 * t6238;
    let t103391 = t7284 * t6238;
    let t103413 = t24574 * t29546;
    (t103286, t103291, t103293, t103304, t103314, t103315, t103332, t103345, t103363, t103391, t103413)
}
