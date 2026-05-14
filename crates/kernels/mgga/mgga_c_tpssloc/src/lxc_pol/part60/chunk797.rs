//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 797/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk797<F: Float>(t26989: F, t7728: F, t1375: F, t26224: F, t31649: F, t31663: F, t33308: F, t33311: F, t33316: F, t33320: F, t5215: F, t5321: F, t6958: F, t7194: F, t7729: F, t7925: F, t8627: F) -> (F, F) {
    let t33323 = t26989 * t7728;
    let t33332 = -0.16449340668482264365e-1 * t33308 - 0.82246703342411321825e-2 * t33311 + t31649 + 2.0 * t5215 * t8627 + 2.0 * t1375 * t33316 + 2.0 * t1375 * t33320 - 6.0 * t26224 * t33323 + 2.0 * t5321 * t8627 + 2.0 * t7194 * t7729 - t31663 + 2.0 * t6958 * t7925;
    (t33323, t33332)
}
