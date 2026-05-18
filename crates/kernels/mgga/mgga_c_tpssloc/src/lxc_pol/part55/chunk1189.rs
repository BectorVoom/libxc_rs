//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1189/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1189<F: Float>(t118709: F, t1888: F, t232: F, t6646: F, t7510: F, t828: F, t118690: F, t25038: F, t25248: F, t776: F, t30676: F, t4119: F, t6552: F, t6637: F) -> (F, F, F, F) {
    let t118710 = F::new(0.82246703342411321825e-2) * t118709;
    let t118715 = F::new(0.16449340668482264365e-1) * t1888 * t6646 * t7510 * t828 * t232;
    let t118719 = F::new(0.9869604401089358619e-1) * t25038 * t25248 * t118690 * t776;
    let t118725 = F::new(0.3289868133696452873e-1) * t6552 * t6637 * t30676 * t4119;
    (t118710, t118715, t118719, t118725)
}
