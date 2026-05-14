//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 563/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk563<F: Float>(t1336: F, t1814: F, t2013: F, t544: F, t6967: F, t6975: F, t7734: F, t7738: F, t7742: F, t7745: F, t7747: F) -> (F,) {
    let t7749 = -t6967 - 0.16449340668482264365e-1 * t7734 - t6975 - 0.82246703342411321825e-2 * t7738 + 0.82246703342411321825e-2 * t7742 + t1814 * t2013 - t1336 * t7745 + t544 * t7747;
    (t7749,)
}
