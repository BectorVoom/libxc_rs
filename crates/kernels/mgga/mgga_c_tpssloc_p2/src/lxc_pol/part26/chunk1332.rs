//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1332/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1332<F: Float>(t12823: F, t6525: F, t12734: F, t1983: F, t22578: F, t6996: F, t22480: F, t2314: F, t22947: F, t532: F, t6879: F, t1874: F, t39235: F) -> (F, F, F, F, F, F) {
    let t83919 = F::new(6.0) * t12823 * t6525;
    let t83921 = F::new(12.0) * t12734 * t6525;
    let t83924 = F::new(3.0) * t1983 * t6996 * t22578;
    let t83928 = F::new(6.0) * t2314 * t22480;
    let t83929 = t532 * t22947;
    let t83932 = F::new(9.0) * t1983 * t83929 * t6879;
    let t83939 = F::new(2.0) * t39235 * t1874;
    (t83919, t83921, t83924, t83928, t83932, t83939)
}
