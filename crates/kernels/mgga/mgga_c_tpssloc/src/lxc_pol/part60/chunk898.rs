//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 898/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk898<F: Float>(t127796: F, t127833: F, t127858: F, t127883: F, t127926: F, t127947: F, t128042: F, t128072: F, t870: F, t1530: F, t33476: F, t1914: F, t5544: F, t101226: F, t115027: F, t126177: F, t1408: F, t1877: F, t22960: F, t24191: F, t25: F, t2522: F, t25373: F, t26744: F, t28252: F, t28456: F, t28459: F, t28462: F, t31434: F, t33466: F, t33486: F, t5397: F, t7114: F, t7475: F, t8566: F, t8569: F) -> (F, F, F, F, F) {
    let t128075 = t127796 + t127833 + t127858 + t127883 + t127926 + t127947 + t128042 + t128072;
    let t128076 = t128075 * t870;
    let t128080 = t33476 * t1530;
    let t128086 = t1914 * t5544;
    let t128093 = t1877 * t115027 * t28456 + 3.0 * t2522 * t8566 * t28252 + t1877 * t33466 * t1408 - t1877 * t26744 * t33486 - t1877 * t7114 * t5397 * t1914 / 2.0 + 3.0 * t2522 * t33466 * t7475 - t1877 * t31434 * t28462 / 2.0 - 3.0 * t24191 * t126177 + t1877 * t8566 * t5397 / 2.0 + t1877 * t128076 * t25 / 2.0 + 6.0 * t24191 * t25373 * t128080 - t1877 * t31434 * t28459 - 3.0 / 2.0 * t24191 * t22960 * t128086 - t1877 * t101226 * t8569 / 2.0;
    (t128075, t128076, t128080, t128086, t128093)
}
