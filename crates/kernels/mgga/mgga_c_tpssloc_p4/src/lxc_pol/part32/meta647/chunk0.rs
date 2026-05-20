//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2069/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2069<F: Float>(t90864: F, t26433: F, t6883: F, t22716: F, t7741: F, t22704: F, t5336: F, t80798: F, t22724: F, t26436: F, t26423: F, t81159: F) -> (F, F, F, F, F, F) {
    let t90865 = F::cast_from(0.82246703342411321824e-2_f64) * t90864;
    let t90866 = t6883 * t26433;
    let t90867 = F::cast_from(0.38381794893125283518e-1_f64) * t90866;
    let t90868 = t22716 * t7741;
    let t90898 = t22704 * t80798 * t5336;
    let t90899 = F::cast_from(0.16449340668482264365e-1_f64) * t90898;
    let t90900 = t22724 * t26436;
    let t90912 = t81159 * t26423;
    (t90865, t90867, t90868, t90899, t90900, t90912)
}
