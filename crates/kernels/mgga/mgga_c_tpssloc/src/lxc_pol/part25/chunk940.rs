//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 940/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk940<F: Float>(t109: F, t2749: F, t28: F, t1081: F, t868: F, t2745: F, t12461: F, t3698: F, t2039: F, t3652: F, t22468: F, t22471: F, t22474: F, t22476: F) -> (F, F, F, F, F, F) {
    let t110 = 1.0 < t109;
    let t23807 = t28 * t2749;
    let t23810 = t1081 * t868;
    let t23813 = t28 * t2745;
    let t23857 = t12461 * t3698;
    let t23909 = t3652 * t2039;
    let t23912 = 22.0 / 9.0 * t22468;
    let t23917 = piecewise3(t110, 0.0, t23912 + 4.0 / 3.0 * t22471 + t22474 / 2.0 - t22476 / 4.0);
    (t23807, t23810, t23813, t23857, t23909, t23917)
}
