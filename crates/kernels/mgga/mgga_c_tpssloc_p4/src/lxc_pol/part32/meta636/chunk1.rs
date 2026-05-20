//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2051/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2051<F: Float>(t23110: F, t23185: F, t25237: F, t23168: F, t25307: F, t25287: F, t81651: F, t22893: F, t23164: F, t25320: F, t7521: F, t81632: F) -> (F, F, F, F, F) {
    let t87601 = t23185 * t23110 * t25237;
    let t87602 = F::cast_from(0.82246703342411321824e-2_f64) * t87601;
    let t87603 = t23168 * t25307;
    let t87604 = F::cast_from(0.76763589786250567036e-1_f64) * t87603;
    let t87612 = t81651 * t23110 * t25287;
    let t87613 = F::cast_from(0.16449340668482264365e-1_f64) * t87612;
    let t87618 = t23164 * t22893 * t25320;
    let t87619 = F::cast_from(0.16449340668482264365e-1_f64) * t87618;
    let t87635 = t81632 * t7521;
    (t87602, t87604, t87613, t87619, t87635)
}
