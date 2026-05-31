//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1056/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1056<F: Float>(t109: F, t22473: F, t26129: F, t4067: F, t6530: F, t22469: F, t22471: F, t26127: F) -> (F, F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t26130 = t22473 * t26129;
    let t26132 = t6530 * t4067;
    let t26135 = piecewise3::<F>(t110, F::cast_from(0.0_f64), t22469 + t22471 / F::cast_from(3.0_f64) + t26127 / F::cast_from(3.0_f64) + t26130 / F::cast_from(4.0_f64) - t26132 / F::cast_from(8.0_f64));
    (t26130, t26132, t26135)
}
