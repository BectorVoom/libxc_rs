//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2082/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2082<F: Float>(t5303: F, t80820: F, t22783: F, t5310: F, t1827: F, t80914: F, t1811: F, t80775: F, t7709: F, t80766: F, t22797: F, t5227: F) -> (F, F, F, F, F, F) {
    let t91364 = t80820 * t5303;
    let t91365 = F::new(7.0) / F::new(288.0) * t91364;
    let t91386 = t22783 * t5310;
    let t91387 = F::new(35.0) / F::new(288.0) * t91386;
    let t91394 = t80914 * t1827;
    let t91398 = t80775 * t1811;
    let t91400 = t80766 * t7709;
    let t91402 = t22797 * t5227;
    (t91365, t91387, t91394, t91398, t91400, t91402)
}
