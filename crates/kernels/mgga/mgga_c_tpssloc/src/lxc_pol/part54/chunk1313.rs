//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1313/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1313<F: Float>(t23185: F, t32862: F, t82074: F, t32863: F, t6579: F, t112726: F, t112660: F, t6552: F, t7479: F, t112961: F, t32823: F, t1888: F, t22996: F, t25281: F) -> (F, F, F, F, F, F, F) {
    let t118661 = t23185 * t82074 * t32862;
    let t118662 = F::new(0.16449340668482264365e-1) * t118661;
    let t118663 = t6579 * t32863;
    let t118664 = F::new(0.76763589786250567037e-1) * t118663;
    let t118667 = F::new(0.38381794893125283518e-1) * t112726;
    let t118672 = F::new(0.3289868133696452873e-1) * t6552 * t112660 * t7479;
    let t118677 = F::new(0.16449340668482264365e-1) * t112961;
    let t118678 = t6579 * t32823;
    let t118679 = F::new(0.38381794893125283518e-1) * t118678;
    let t118682 = F::new(0.3289868133696452873e-1) * t1888 * t22996 * t25281;
    (t118662, t118664, t118667, t118672, t118677, t118679, t118682)
}
