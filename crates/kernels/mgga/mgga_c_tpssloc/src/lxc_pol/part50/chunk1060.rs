//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1060/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1060<F: Float>(t112983: F, t1888: F, t25262: F, t6646: F, t112991: F, t112997: F, t32827: F, t6547: F, t1880: F, t1894: F, t214: F, t25160: F, t23168: F, t32819: F, t234: F, t7510: F) -> (F, F, F, F, F, F, F, F) {
    let t118730 = 0.82246703342411321825e-2 * t112983;
    let t118735 = 0.16449340668482264365e-1 * t1888 * t6646 * t25262;
    let t118736 = 0.38381794893125283518e-1 * t112991;
    let t118737 = 0.82246703342411321825e-2 * t112997;
    let t118738 = t6547 * t32827;
    let t118739 = 0.38381794893125283518e-1 * t118738;
    let t118743 = 0.16449340668482264365e-1 * t1880 * t214 * t1894 * t25160;
    let t118744 = t23168 * t32819;
    let t118745 = 0.76763589786250567037e-1 * t118744;
    let t118747 = t234 * t7510;
    (t118730, t118735, t118736, t118737, t118739, t118743, t118745, t118747)
}
