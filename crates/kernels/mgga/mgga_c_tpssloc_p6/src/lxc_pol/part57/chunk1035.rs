//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1035/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1035<F: Float>(t102917: F, t114225: F, t114264: F, t122331: F, t127354: F, t127355: F, t127422: F, t127423: F, t127427: F, t128797: F, t128805: F, t128809: F, t128816: F, t20029: F, t2016: F, t26224: F, t26477: F, t26989: F, t27009: F, t28110: F, t7750: F, t7937: F, t8637: F) -> F {
    let t128818 = t114225 - F::cast_from(2.0_f64) * t20029 * t8637 - F::cast_from(2.0_f64) * t27009 * t7750 + t127354 + t127355 + F::cast_from(0.16449340668482264365e-1_f64) * t122331 - F::cast_from(0.16449340668482264365e-1_f64) * t128797 - F::cast_from(2.0_f64) * t26477 * t7937 - F::cast_from(2.0_f64) * t102917 * t2016 - F::cast_from(0.6579736267392905746e-1_f64) * t128805 + t114264 - t127422 + F::cast_from(0.3289868133696452873e-1_f64) * t128809 + t127423 - F::cast_from(6.0_f64) * t26224 * t26989 * t28110 - F::cast_from(0.49348022005446793095e-1_f64) * t128816 + t127427;
    t128818
}
