//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1194/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1194<F: Float>(t22779: F, t32714: F, t5230: F, t8465: F, t8467: F, t1814: F, t31175: F, t26288: F, t5308: F, t6950: F, t1985: F, t26202: F, t31137: F, t1799: F, t2006: F, t1307: F, t26331: F, t26446: F) -> (F, F, F, F, F, F, F) {
    let t120410 = t22779 * t32714;
    let t120413 = t5230 * t8465 * t8467;
    let t120416 = t1814 * t31175 * t8467;
    let t120419 = t26288 * t6950 * t5308;
    let t120436 = 0.16449340668482264365e-1 * t1985 * t31137 * t26202;
    let t120437 = t2006 * t1799;
    let t120441 = 0.9869604401089358619e-1 * t26331 * t26446 * t120437 * t1307;
    (t120410, t120413, t120416, t120419, t120436, t120437, t120441)
}
