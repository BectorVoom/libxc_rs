//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2212/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2212<F: Float>(t22704: F, t22705: F, t26461: F, t26433: F, t6883: F, t22716: F, t7741: F, t1834: F, t3791: F, t1992: F, t550: F, t6976: F) -> (F, F, F, F, F) {
    let t90864 = t22704 * t22705 * t26461;
    let t90865 = F::cast_from(0.82246703342411321824e-2_f64) * t90864;
    let t90866 = t6883 * t26433;
    let t90867 = F::cast_from(0.38381794893125283518e-1_f64) * t90866;
    let t90868 = t22716 * t7741;
    let t90870 = t1834 * t3791;
    let t90873 = t1992 * t6976 * t90870 * t550;
    (t90865, t90867, t90868, t90870, t90873)
}
