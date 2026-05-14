//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1174/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1174<F: Float>(t118690: F, t22986: F, t6646: F, t829: F, t112968: F, t25261: F, t2647: F, t112974: F, t32826: F, t6562: F, t794: F, t1888: F, t232: F, t7510: F, t828: F, t25038: F, t25248: F, t776: F) -> (F, F, F, F, F, F, F) {
    let t118694 = 0.3289868133696452873e-1 * t22986 * t6646 * t118690 * t829;
    let t118695 = 0.76763589786250567036e-1 * t112968;
    let t118699 = 0.3289868133696452873e-1 * t22986 * t6646 * t25261 * t2647;
    let t118700 = 0.38381794893125283518e-1 * t112974;
    let t118709 = t6562 * t794 * t32826;
    let t118710 = 0.82246703342411321825e-2 * t118709;
    let t118715 = 0.16449340668482264365e-1 * t1888 * t6646 * t7510 * t828 * t232;
    let t118719 = 0.9869604401089358619e-1 * t25038 * t25248 * t118690 * t776;
    (t118694, t118695, t118699, t118700, t118710, t118715, t118719)
}
