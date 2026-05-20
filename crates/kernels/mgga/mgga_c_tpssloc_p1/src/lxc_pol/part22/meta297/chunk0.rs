//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1459/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1459<F: Float>(t2960: F, t4528: F, t1599: F, t698: F, t973: F, t135: F, t4542: F, t13552: F, t13550: F, t13644: F, t1036: F, t4622: F) -> (F, F, F, F, F, F, F, F) {
    let t13907 = F::cast_from(0.14814814814814814814e-2_f64) * t2960 * t4528;
    let t13908 = t698 * t1599;
    let t13909 = t973 * t13908;
    let t13913 = t135 * t4542;
    let t13915 = F::cast_from(0.55555555555555555554e-3_f64) * t973 * t13913;
    let t13921 = F::new(2.0) / F::new(27.0) * t13552;
    let t13922 = F::new(4.0) / F::new(9.0) * t13550;
    let t13923 = F::new(2.0) / F::new(9.0) * t13644;
    let t13946 = t4622 * t1036 / F::new(432.0);
    (t13907, t13908, t13909, t13915, t13921, t13922, t13923, t13946)
}
