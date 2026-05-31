//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 236/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk236<F: Float>(t1410: F, t65: F, t1409: F, t43: F, t46: F, t48: F, t55: F, t39: F, t51: F, t56: F, t627: F, t33: F, rho1: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t1411 = t1410 * t65;
    let t1414 = t43 * t1409;
    let t1417 = t46 * rho1;
    let t1419 = F::cast_from(1.0_f64) / t48 / t1417;
    let t1420 = sigma2 * t1419;
    let t1423 = t55 * t1409;
    let t1426 = F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t39 * t1414 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1420 * t56 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t51 * t1423 + t627;
    let t1427 = t33 * t1426;
    (t1411, t1417, t1419, t1420, t1423, t1426, t1427)
}
