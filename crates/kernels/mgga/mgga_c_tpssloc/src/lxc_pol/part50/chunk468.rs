//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 468/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk468<F: Float>(t323: F, t300: F, t938: F, t964: F, t969: F, t615: F, t972: F, t340: F, t697: F, t344: F) -> (F, F, F, F, F, F) {
    let t2931 = t323 * t323;
    let t2932 = 1.0 / t2931;
    let t2940 = t300 * t938;
    let t2958 = t964 * t969;
    let t2960 = t615 * t972;
    let t2965 = t697 * t340;
    let t2966 = t2965 * t344;
    (t2932, t2940, t2958, t2960, t2965, t2966)
}
