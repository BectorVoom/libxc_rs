//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 860/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk860<F: Float>(t22927: F, t6897: F, t6883: F, t6903: F, t1914: F, t193: F, t201: F, t25: F, t2752: F) -> (F, F, F, F, F) {
    let t22928 = t6897 * t22927;
    let t22940 = t6883 * t6903;
    let t22941 = 0.38381794893125283518e-1 * t22940;
    let t22959 = t193 * t201 * t1914;
    let t22960 = t2752 * t25;
    (t22928, t22940, t22941, t22959, t22960)
}
