//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1186/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1186<F: Float>(t192: F, t531: F, t1982: F, t1914: F, t193: F, t200: F, t25: F, t870: F, t1887: F, t23056: F, t23046: F, t242: F) -> (F, F, F, F, F) {
    let t24994 = t192 * t531;
    let t24995 = t1982 * t24994;
    let t25013 = t193 * t200 * t1914;
    let t25014 = t870 * t25;
    let t25038 = t23056 * t1887;
    let t25083 = t23046 * t242;
    (t24995, t25013, t25014, t25038, t25083)
}
