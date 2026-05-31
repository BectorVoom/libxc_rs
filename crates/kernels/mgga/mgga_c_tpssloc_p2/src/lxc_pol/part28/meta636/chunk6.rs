//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2026/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2026<F: Float>(t93642: F, t93661: F, t93681: F, t93699: F, t93719: F, t93738: F, t93756: F, t93773: F, t16060: F, t27086: F, t3777: F, t544: F, t553: F, t7209: F, t81127: F, t81140: F, t81149: F, t81160: F, t81184: F, t84595: F, t84597: F, t91002: F, t91008: F, t91014: F, t91025: F, t91036: F, t93615: F, t93618: F) -> (F, F) {
    let t93776 = t93642 + t93661 + t93681 + t93699 + t93719 + t93738 + t93756 + t93773;
    let t93784 = F::cast_from(0.76763589786250567036e-1_f64) * t81127 - t93615 - F::cast_from(0.46058153871750340222e0_f64) * t91002 - F::cast_from(0.13159472534785811492e0_f64) * t91008 + t93618 - F::cast_from(2.0_f64) * t3777 * t27086 - F::cast_from(0.49348022005446793095e-1_f64) * t81140 - t84595 - F::cast_from(0.3289868133696452873e-1_f64) * t91014 - F::cast_from(2.0_f64) * t16060 * t7209 + t544 * t553 * t93776 - F::cast_from(0.3289868133696452873e-1_f64) * t81149 + t84597 - F::cast_from(0.15352717957250113407e0_f64) * t81160 - F::cast_from(0.76763589786250567036e-1_f64) * t81184 - F::cast_from(0.3289868133696452873e-1_f64) * t91025 + F::cast_from(0.6579736267392905746e-1_f64) * t91036;
    (t93776, t93784)
}
