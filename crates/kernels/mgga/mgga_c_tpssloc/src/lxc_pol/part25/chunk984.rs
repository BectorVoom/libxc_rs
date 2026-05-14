//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 984/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk984<F: Float>(t22813: F, t6924: F, t80782: F, t22794: F, t22843: F, t281: F, t6597: F, t1361: F, t22690: F, t3734: F, t154: F, t8705: F, t1887: F, t534: F, t12267: F, t6951: F) -> (F, F, F, F, F) {
    let t80836 = t22813 * t6924 * t80782;
    let t80837 = t80836 * t22794;
    let t80840 = t6597 * t22843 * t281;
    let t80843 = t80840 * t22690 * t1361 * t3734;
    let t80845 = t8705 * t154;
    let t80847 = t80845 * t534 * t1887;
    let t80849 = t12267 * t6951;
    (t80837, t80843, t80845, t80847, t80849)
}
