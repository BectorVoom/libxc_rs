//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1263/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1263<F: Float>(t1354: F, t80914: F, t1339: F, t55003: F, t6936: F, t22770: F, t22779: F, t22773: F, t12178: F, t12168: F, t12303: F, t221: F, t26284: F) -> (F, F, F, F, F, F, F) {
    let t80915 = t80914 * t1354;
    let t80918 = t6936 * t1339 * t55003;
    let t80920 = t22779 * t22770;
    let t80922 = t22779 * t22773;
    let t80925 = t6936 * t1339 * t12178;
    let t80928 = t6936 * t1339 * t12168;
    let t80931 = t26284 * t221 * t12303;
    (t80915, t80918, t80920, t80922, t80925, t80928, t80931)
}
