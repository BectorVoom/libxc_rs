//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2084/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2084<F: Float>(t26392: F, t80670: F, t22705: F, t26422: F, t81228: F, t22704: F, t26466: F, t26461: F, t26433: F, t6883: F, t22716: F, t7741: F) -> (F, F, F, F, F, F) {
    let t90837 = t80670 * t26392;
    let t90844 = t81228 * t22705 * t26422;
    let t90845 = F::cast_from(0.16449340668482264365e-1_f64) * t90844;
    let t90859 = t22704 * t22705 * t26466;
    let t90860 = F::cast_from(0.82246703342411321824e-2_f64) * t90859;
    let t90864 = t22704 * t22705 * t26461;
    let t90865 = F::cast_from(0.82246703342411321824e-2_f64) * t90864;
    let t90866 = t6883 * t26433;
    let t90867 = F::cast_from(0.38381794893125283518e-1_f64) * t90866;
    let t90868 = t22716 * t7741;
    (t90837, t90845, t90860, t90865, t90867, t90868)
}
