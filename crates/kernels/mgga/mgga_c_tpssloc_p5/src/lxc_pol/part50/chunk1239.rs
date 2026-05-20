//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1239/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1239<F: Float>(t33137: F, t6876: F, t22574: F, t25988: F, t36533: F, t25985: F, t8450: F, t36363: F, t31044: F, t7685: F, t26168: F, t24991: F) -> (F, F, F, F, F, F, F) {
    let t120075 = F::new(2.0) * t6876 * t33137;
    let t120078 = F::new(6.0) * t22574 * t36533 * t25988;
    let t120079 = t8450 * t25985;
    let t120083 = F::new(3.0) * t22574 * t36363 * t25988;
    let t120085 = F::new(2.0) * t7685 * t31044;
    let t120086 = t8450 * t26168;
    let t120088 = t8450 * t24991;
    (t120075, t120078, t120079, t120083, t120085, t120086, t120088)
}
