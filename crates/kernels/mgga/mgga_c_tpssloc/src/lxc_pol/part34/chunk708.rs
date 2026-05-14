//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 708/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk708<F: Float>(t1314: F, t2566: F, t3732: F, t792: F, t782: F, t1365: F, t154: F, t205: F, t116: F, t547: F, t535: F, t9534: F, t9538: F, t1337: F) -> (F, F, F, F, F, F, F) {
    let t12199 = t2566 * t1314;
    let t12202 = t792 * t3732;
    let t12211 = t782 * t3732;
    let t12214 = t154 * t1365;
    let t12215 = t205 * t12214;
    let t12225 = t547 * t116;
    let t12236 = 0.13888888888888888889e-3 * t9534 * t535 * t9538;
    let t12247 = t1337 * t1337;
    let t12248 = 1.0 / t12247;
    (t12199, t12202, t12211, t12215, t12225, t12236, t12248)
}
