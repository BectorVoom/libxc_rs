//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 849/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk849<F: Float>(t1880: F, t30657: F, t23204: F, t8335: F, t6562: F, t1902: F, t214: F) -> (F, F, F, F) {
    let t30659 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t30657;
    let t30660 = t23204 * t8335;
    let t30662 = F::cast_from(0.82246703342411321825e-2_f64) * t6562 * t30660;
    let t30663 = t214 * t1902;
    (t30659, t30660, t30662, t30663)
}
