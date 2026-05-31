//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 988/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk988<F: Float>(t13913: F, t973: F, t13552: F, t13550: F, t13644: F, t1036: F, t4622: F, t3117: F, t4571: F, t248: F, t3051: F, t4347: F) -> (F, F, F, F, F, F, F) {
    let t13915 = F::cast_from(0.55555555555555555554e-3_f64) * t973 * t13913;
    let t13921 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t13552;
    let t13922 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t13550;
    let t13923 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13644;
    let t13946 = t4622 * t1036 / F::cast_from(432.0_f64);
    let t13948 = t3117 * t4571 / F::cast_from(3456.0_f64);
    let t13950 = t248 * t3051 * t4347;
    (t13915, t13921, t13922, t13923, t13946, t13948, t13950)
}
