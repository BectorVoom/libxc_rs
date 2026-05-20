//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2189/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2189<F: Float>(t1880: F, t23196: F, t25224: F, t23030: F, t25205: F, t23164: F, t7479: F, t82133: F, t6552: F, t82124: F, t23237: F, t25341: F) -> (F, F, F, F, F) {
    let t87893 = t1880 * t25224 * t23196;
    let t87898 = t23030 * t25205;
    let t87901 = t23164 * t82133 * t7479;
    let t87902 = F::cast_from(0.16449340668482264365e-1_f64) * t87901;
    let t87904 = t6552 * t82124 * t7479;
    let t87907 = t6552 * t23237 * t25341;
    (t87893, t87898, t87902, t87904, t87907)
}
