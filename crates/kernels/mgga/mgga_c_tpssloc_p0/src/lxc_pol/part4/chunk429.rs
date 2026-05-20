//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 429/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk429<F: Float>(t1409: F, t31: F, t65: F, t43: F, t46: F, t48: F, rho1: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t1410 = t31 * t1409;
    let t1411 = t1410 * t65;
    let t1414 = t43 * t1409;
    let t1417 = t46 * rho1;
    let t1419 = F::new(1.0) / t48 / t1417;
    let t1420 = sigma2 * t1419;
    (t1410, t1411, t1414, t1417, t1419, t1420)
}
