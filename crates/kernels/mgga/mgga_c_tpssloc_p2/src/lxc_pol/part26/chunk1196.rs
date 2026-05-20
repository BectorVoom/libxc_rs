//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1196/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1196<F: Float>(t2678: F, t852: F, t225: F, t9520: F, t3639: F, t11923: F, t11931: F, t11604: F, t496: F, t68: F, t11601: F, t11599: F) -> (F, F, F, F, F, F, F, F) {
    let t40955 = t852 * t2678;
    let t41554 = t9520 * t225;
    let t43705 = t3639 * t3639;
    let t43706 = F::new(1.0) / t43705;
    let t44412 = t11923 * t225;
    let t45345 = t11931 * t225;
    let t45349 = F::new(1.0) / t11604 / t496;
    let t45350 = t68 * t45349;
    let t45355 = t11601 * t225;
    let t45375 = t11599 * t225;
    (t40955, t41554, t43706, t44412, t45345, t45350, t45355, t45375)
}
