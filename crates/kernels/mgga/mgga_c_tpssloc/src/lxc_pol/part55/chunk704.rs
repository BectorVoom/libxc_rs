//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 704/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk704<F: Float>(t1499: F, t1909: F, t226: F, t6636: F, t6645: F, t7522: F, t7526: F, t7530: F, t7533: F, t7535: F, t812: F) -> (F,) {
    let t7537 = -t6636 - 0.16449340668482264365e-1 * t7522 - t6645 - 0.82246703342411321825e-2 * t7526 + 0.82246703342411321825e-2 * t7530 + t1499 * t1909 - t812 * t7533 + t226 * t7535;
    (t7537,)
}
