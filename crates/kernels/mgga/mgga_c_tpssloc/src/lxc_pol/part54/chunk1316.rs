//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1316/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1316<F: Float>(t22893: F, t23164: F, t32818: F, t112983: F, t1888: F, t25262: F, t6646: F, t112991: F, t112997: F, t32827: F, t6547: F, t1880: F, t1894: F, t214: F, t25160: F) -> (F, F, F, F, F, F, F) {
    let t118727 = t23164 * t22893 * t32818;
    let t118728 = F::cast_from(0.16449340668482264365e-1_f64) * t118727;
    let t118730 = F::cast_from(0.82246703342411321825e-2_f64) * t112983;
    let t118735 = F::cast_from(0.16449340668482264365e-1_f64) * t1888 * t6646 * t25262;
    let t118736 = F::cast_from(0.38381794893125283518e-1_f64) * t112991;
    let t118737 = F::cast_from(0.82246703342411321825e-2_f64) * t112997;
    let t118738 = t6547 * t32827;
    let t118739 = F::cast_from(0.38381794893125283518e-1_f64) * t118738;
    let t118743 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t214 * t1894 * t25160;
    (t118728, t118730, t118735, t118736, t118737, t118739, t118743)
}
