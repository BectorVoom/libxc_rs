//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1361/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1361<F: Float>(t105726: F, t105731: F, t105754: F, t105758: F, t105762: F, t105769: F, t1484: F, t1530: F, t1877: F, t1914: F, t1915: F, t193: F, t202: F, t20756: F, t20778: F, t20800: F, t20947: F, t21066: F, t23295: F, t2522: F, t25358: F, t28248: F, t28448: F, t4314: F, t5527: F, t5544: F, t5660: F, t5664: F, t6670: F, t7541: F, t82312: F, t870: F, t87975: F, t98054: F) -> F {
    let t106606 = -F::new(18.0) * t2522 * t25358 * t28248 - t1877 * t6670 * t21066 - F::new(18.0) * t4314 * t6670 * t105762 + F::new(18.0) * t4314 * t1915 * t20947 + F::new(18.0) * t2522 * t23295 * t105731 - F::new(6.0) * t1877 * t82312 * t20778 + F::new(6.0) * t1877 * t87975 * t5664 + F::new(6.0) * t1877 * t23295 * t105769 + F::new(6.0) * t193 * t20756 * t1914 * t870 + F::new(18.0) * t4314 * t7541 * t5527 + F::new(9.0) * t2522 * t28448 * t1484 - F::new(9.0) * t2522 * t6670 * t105758 - F::new(9.0) * t2522 * t6670 * t105754 + F::new(3.0) * t2522 * t1915 * t20800 - F::new(3.0) * t1877 * t25358 * t5660 - F::new(3.0) * t1877 * t98054 * t1530 + F::new(9.0) * t2522 * t7541 * t5544 + t193 * t202 * t105726 * t870;
    t106606
}
