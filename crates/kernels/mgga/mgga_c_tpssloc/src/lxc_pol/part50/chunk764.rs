//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 764/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk764<F: Float>(t1510: F, t6657: F, t235: F, t7510: F, t1499: F, t1909: F, t226: F, t6636: F, t6645: F, t7522: F, t7526: F, t7530: F, t812: F) -> (F, F, F) {
    let t7533 = t6657 * t1510;
    let t7535 = t235 * t7510;
    let t7537 = -t6636 - F::cast_from(0.16449340668482264365e-1_f64) * t7522 - t6645 - F::cast_from(0.82246703342411321825e-2_f64) * t7526 + F::cast_from(0.82246703342411321825e-2_f64) * t7530 + t1499 * t1909 - t812 * t7533 + t226 * t7535;
    (t7533, t7535, t7537)
}
