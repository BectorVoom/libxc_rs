//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1350/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1350<F: Float>(t31058: F, t4028: F, t12725: F, t8327: F, t20173: F, t33193: F, t3941: F, t4072: F, t8326: F, t16524: F, t31285: F, t16521: F) -> (F, F, F, F, F, F) {
    let t120730 = F::cast_from(2.0_f64) * t4028 * t31058;
    let t120735 = F::cast_from(2.0_f64) * t12725 * t8327;
    let t120800 = F::cast_from(27.0_f64) * t20173 * t33193;
    let t120803 = F::cast_from(27.0_f64) * t3941 * t8326 * t4072;
    let t120807 = F::cast_from(27.0_f64) * t16524 * t31285;
    let t120809 = F::cast_from(0.135e2_f64) * t16521 * t8326;
    (t120730, t120735, t120800, t120803, t120807, t120809)
}
