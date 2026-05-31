//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1219/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1219<F: Float>(t33137: F, t6876: F, t22574: F, t25988: F, t36533: F, t36363: F, t31044: F, t7685: F, t19577: F, t24995: F, t37589: F, t5308: F) -> (F, F, F, F, F, F) {
    let t120075 = F::cast_from(2.0_f64) * t6876 * t33137;
    let t120078 = F::cast_from(6.0_f64) * t22574 * t36533 * t25988;
    let t120083 = F::cast_from(3.0_f64) * t22574 * t36363 * t25988;
    let t120085 = F::cast_from(2.0_f64) * t7685 * t31044;
    let t120092 = F::cast_from(3.0_f64) * t22574 * t36363 * t19577;
    let t120095 = F::cast_from(6.0_f64) * t24995 * t37589 * t5308;
    (t120075, t120078, t120083, t120085, t120092, t120095)
}
