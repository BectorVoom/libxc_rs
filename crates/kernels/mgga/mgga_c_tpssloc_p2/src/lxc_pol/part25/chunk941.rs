//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 941/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk941<F: Float>(t3824: F, t588: F, t1287: F, t2225: F, t12083: F, t184: F, t17: F, t3681: F, t750: F, t1284: F, t2516: F, t521: F, t9861: F) -> (F, F, F, F, F, F) {
    let t12120 = t588 * t3824;
    let t12121 = F::cast_from(12.0_f64) * t12120;
    let t12123 = F::cast_from(60.0_f64) * t2225 * t1287;
    let t12124 = t12083 * t184;
    let t12125 = t17 * t12124;
    let t12126 = t3681 * t750;
    let t12127 = t17 * t12126;
    let t12128 = F::cast_from(3.0_f64) * t12127;
    let t12129 = t1284 * t2516;
    let t12130 = t17 * t12129;
    let t12131 = F::cast_from(3.0_f64) * t12130;
    let t12132 = t521 * t9861;
    (t12121, t12123, t12125, t12128, t12131, t12132)
}
