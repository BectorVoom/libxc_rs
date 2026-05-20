//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1040/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1040<F: Float>(t22635: F, t26331: F, t31549: F, t6330: F, t115630: F, t122390: F, t122551: F, t127434: F, t127442: F, t127445: F, t127448: F, t127455: F, t127459: F, t127463: F, t128841: F, t128855: F, t128874: F, t128882: F, t1375: F, t1378: F, t20060: F, t29299: F, t29372: F, t33320: F, t5215: F, t5321: F, t6958: F, t8637: F) -> F {
    let t128894 = t26331 * t22635 * t31549 * t6330;
    let t128902 = -t1375 * t1378 * (t128841 + t128855 + t128874 + t128882) + F::new(4.0) * t5215 * t33320 + F::new(2.0) * t6958 * t29372 + F::cast_from(0.82246703342411321824e-2_f64) * t122390 - t127434 + t115630 - t127442 - F::cast_from(0.49348022005446793095e-1_f64) * t128894 - t127445 + F::new(4.0) * t5321 * t33320 - t127448 - t20060 * t8637 + t127455 - F::cast_from(0.82246703342411321824e-2_f64) * t122551 - F::new(6.0) * t6958 * t29299 + t127459 - t127463;
    t128902
}
