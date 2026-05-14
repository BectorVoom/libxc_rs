//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1012/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1012<F: Float>(t10224: F, t1592: F, t973: F, t2960: F, t4528: F, t1599: F, t698: F, t135: F, t4542: F, t13552: F, t13550: F, t13644: F, t10295: F, t10296: F, t10298: F, t10300: F, t10302: F, t13530: F, t13534: F, t13539: F, t13544: F, t13548: F, t13557: F, t13561: F, t13642: F, t13647: F) -> (F, F, F, F, F) {
    let t13895 = t10224 * t1592;
    let t13896 = t973 * t13895;
    let t13907 = 0.14814814814814814814e-2 * t2960 * t4528;
    let t13908 = t698 * t1599;
    let t13909 = t973 * t13908;
    let t13913 = t135 * t4542;
    let t13915 = 0.55555555555555555554e-3 * t973 * t13913;
    let t13921 = 2.0 / 27.0 * t13552;
    let t13922 = 4.0 / 9.0 * t13550;
    let t13923 = 2.0 / 9.0 * t13644;
    let t13931 = t10295 + 10.0 / 27.0 * t10296 - t10298 / 27.0 + 2.0 / 9.0 * t10300 - t10302 / 9.0 + 5.0 / 27.0 * t13642 - t13921 + t13922 - t13923 + 2.0 / 27.0 * t13539 - t13557 / 3.0 + t13530 / 9.0 + t13534 / 18.0 + t13561 - 2.0 / 3.0 * t13544 - t13548 / 3.0 + t13647 / 6.0;
    (t13896, t13907, t13909, t13915, t13931)
}
