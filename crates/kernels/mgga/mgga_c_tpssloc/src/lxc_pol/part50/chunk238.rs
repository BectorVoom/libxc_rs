//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 238/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk238<F: Float>(t310: F, t931: F, t880: F, t886: F, t324: F, t320: F) -> (F, F, F, F, F, F, F) {
    let t932 = 1.0 / t310;
    let t933 = t931 * t932;
    let t936 = 0.92708333333333333333e-2 * t880;
    let t938 = -t936 - 0.92708333333333333333e-2 * t886;
    let t939 = t938 * t324;
    let t941 = t320 * t320;
    let t942 = 1.0 / t941;
    (t932, t933, t936, t938, t939, t941, t942)
}
