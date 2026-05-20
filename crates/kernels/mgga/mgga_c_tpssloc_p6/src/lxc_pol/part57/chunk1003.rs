//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1003/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1003<F: Float>(t101226: F, t115027: F, t121782: F, t126176: F, t126197: F, t128075: F, t128086: F, t128097: F, t128101: F, t128110: F, t1484: F, t1530: F, t1877: F, t1914: F, t193: F, t202: F, t23295: F, t24191: F, t24344: F, t2522: F, t26744: F, t28248: F, t28447: F, t31434: F, t33466: F, t33476: F, t33483: F, t4314: F, t5527: F, t5544: F, t5660: F, t5664: F, t7114: F, t7540: F, t84766: F, t8566: F, t870: F, t93000: F) -> F {
    let t128193 = t193 * t202 * t128075 * t870 + F::new(6.0) * t2522 * t33466 * t1484 + F::new(3.0) * t2522 * t8566 * t5544 - F::new(2.0) * t1877 * t121782 * t1530 - F::new(6.0) * t2522 * t7114 * t126176 + F::new(4.0) * t1877 * t24344 * t126197 - F::new(3.0) * t2522 * t7114 * t128086 - F::new(2.0) * t1877 * t26744 * t7540 - F::new(6.0) * t1877 * t84766 * t128110 + F::new(12.0) * t24191 * t23295 * t28248 + F::new(6.0) * t4314 * t8566 * t5527 - t1877 * t101226 * t1914 - F::new(6.0) * t2522 * t26744 * t33476 + F::new(2.0) * t1877 * t24344 * t128101 + F::new(4.0) * t1877 * t93000 * t33483 - F::new(6.0) * t2522 * t31434 * t28248 - F::new(6.0) * t4314 * t7114 * t128097 - t1877 * t31434 * t5660 + F::new(2.0) * t1877 * t115027 * t5664 - t1877 * t7114 * t28447;
    t128193
}
