//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1236/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1236<F: Float>(t25373: F, t33483: F, t1408: F, t1914: F, t1484: F, t1530: F, t1877: F, t193: F, t202: F, t24344: F, t2522: F, t26744: F, t31434: F, t33465: F, t33476: F, t7114: F, t7540: F, t8566: F, t870: F) -> (F, F, F) {
    let t33484 = t25373 * t33483;
    let t33486 = t1408 * t1914;
    let t33512 = t193 * t202 * t33465 * t870 + F::new(3.0) * t1484 * t2522 * t8566 - t1530 * t1877 * t31434 - t1877 * t1914 * t26744 + F::new(2.0) * t1877 * t24344 * t33483 - t1877 * t7114 * t7540 - F::new(3.0) * t2522 * t33476 * t7114;
    (t33484, t33486, t33512)
}
