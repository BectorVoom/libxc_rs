//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1360/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1360<F: Float>(t20173: F, t33193: F, t3941: F, t4072: F, t8326: F, t7015: F, t86647: F, t16524: F, t31285: F, t16521: F, t12524: F, t33188: F) -> (F, F, F, F, F, F) {
    let t120800 = F::cast_from(27.0_f64) * t20173 * t33193;
    let t120803 = F::cast_from(27.0_f64) * t3941 * t8326 * t4072;
    let t120804 = t86647 * t7015;
    let t120807 = F::cast_from(27.0_f64) * t16524 * t31285;
    let t120809 = F::cast_from(0.135e2_f64) * t16521 * t8326;
    let t120811 = F::cast_from(54.0_f64) * t12524 * t33188;
    (t120800, t120803, t120804, t120807, t120809, t120811)
}
