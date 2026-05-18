//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1220/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1220<F: Float>(t10121: F, t10140: F, t13487: F, t1877: F, t193: F, t202: F, t2056: F, t2057: F, t2379: F, t24335: F, t24339: F, t24344: F, t2522: F, t2553: F, t2745: F, t2749: F, t4314: F, t46240: F, t46252: F, t46298: F, t46320: F, t46362: F, t7110: F, t7114: F, t776: F, t84766: F, t84791: F, t84800: F, t85166: F, t868: F, t870: F, t9458: F, t9516: F, t9616: F) -> F {
    let t85243 = -F::new(18.0) * t2522 * t24339 * t13487 + F::new(3.0) * t2522 * t2057 * t9516 + t193 * t202 * t85166 * t870 - F::new(9.0) * t2522 * t7114 * t46252 - F::new(9.0) * t2522 * t7114 * t46240 + F::new(18.0) * t2522 * t24344 * t46320 - F::new(3.0) * t1877 * t24339 * t2745 + F::new(9.0) * t2522 * t7110 * t2553 - F::new(18.0) * t4314 * t7114 * t46298 + F::new(18.0) * t4314 * t2057 * t9616 + F::new(6.0) * t193 * t9458 * t2056 * t870 + F::new(18.0) * t4314 * t7110 * t2379 + F::new(6.0) * t1877 * t24344 * t46362 + F::new(9.0) * t2522 * t24335 * t776 - F::new(6.0) * t1877 * t84766 * t10140 + F::new(6.0) * t1877 * t84800 * t2749 - t1877 * t7114 * t10121 - F::new(3.0) * t1877 * t84791 * t868;
    t85243
}
