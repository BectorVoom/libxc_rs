//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 926/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk926<F: Float>(t2740: F, t8984: F, t348: F, t956: F, t2738: F, t983: F, t140: F, t2692: F, t925: F, t242: F, t2465: F, t2751: F, t967: F, t2657: F, t962: F, t2650: F, t958: F) -> (F, F, F, F, F, F) {
    let t8985 = t2740 * t8984;
    let t8987 = t348 * t956;
    let t8989 = t983 * t8987 * t2738;
    let t8997 = t140 * t2692;
    let t8998 = t925 * t8997;
    let t9003 = t242 * t2751 * t2465;
    let t9004 = t967 * t9003;
    let t9031 = t2657 * t962;
    let t9033 = t958 * t2650;
    (t8985, t8989, t8998, t9004, t9031, t9033)
}
