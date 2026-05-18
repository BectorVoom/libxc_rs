//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1237/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1237<F: Float>(t101226: F, t105731: F, t105754: F, t105758: F, t105762: F, t105769: F, t108451: F, t1484: F, t1530: F, t1877: F, t193: F, t202: F, t2056: F, t2057: F, t20756: F, t20778: F, t20800: F, t20947: F, t21066: F, t24344: F, t2522: F, t26744: F, t28248: F, t29106: F, t4314: F, t5527: F, t5544: F, t5660: F, t5664: F, t7114: F, t7845: F, t84766: F, t870: F, t93000: F) -> F {
    let t108522 = -F::new(6.0) * t1877 * t84766 * t20778 + F::new(6.0) * t1877 * t24344 * t105769 - F::new(9.0) * t2522 * t7114 * t105758 - F::new(9.0) * t2522 * t7114 * t105754 + F::new(18.0) * t4314 * t2057 * t20947 + F::new(9.0) * t2522 * t29106 * t1484 - t1877 * t7114 * t21066 - F::new(3.0) * t1877 * t26744 * t5660 + F::new(6.0) * t193 * t20756 * t2056 * t870 + F::new(18.0) * t4314 * t7845 * t5527 + F::new(9.0) * t2522 * t7845 * t5544 - F::new(18.0) * t4314 * t7114 * t105762 - F::new(18.0) * t2522 * t26744 * t28248 + F::new(18.0) * t2522 * t24344 * t105731 + F::new(6.0) * t1877 * t93000 * t5664 - F::new(3.0) * t1877 * t101226 * t1530 + t193 * t202 * t108451 * t870 + F::new(3.0) * t2522 * t2057 * t20800;
    t108522
}
