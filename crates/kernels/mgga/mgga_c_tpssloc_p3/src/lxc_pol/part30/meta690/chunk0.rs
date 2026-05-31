//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2199/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2199<F: Float>(t19534: F, t89: F, t1874: F, t28030: F, t6525: F, t28821: F, t6880: F, t28239: F, t6876: F, t1983: F, t26503: F, t5161: F) -> (F, F, F, F, F) {
    let t97933 = t89 * t19534;
    let t97935 = F::cast_from(2.0_f64) * t97933 * t1874;
    let t97937 = F::cast_from(2.0_f64) * t28030 * t6525;
    let t97941 = F::cast_from(3.0_f64) * t28821 * t6880;
    let t97942 = t6876 * t28239;
    let t97947 = F::cast_from(2.0_f64) * t1983 * t26503 * t5161;
    (t97935, t97937, t97941, t97942, t97947)
}
