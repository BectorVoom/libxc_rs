//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1178/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1178<F: Float>(t118661: F, t32863: F, t6579: F, t112726: F, t118639: F, t118640: F, t118650: F, t118654: F, t13042: F, t13463: F, t1912: F, t23281: F, t25170: F, t2713: F, t32800: F, t32804: F, t7517: F, t8363: F, t87758: F, t87810: F, t87837: F) -> F {
    let t118662 = F::new(0.16449340668482264365e-1) * t118661;
    let t118663 = t6579 * t32863;
    let t118664 = F::new(0.76763589786250567037e-1) * t118663;
    let t118667 = F::new(0.38381794893125283518e-1) * t112726;
    let t118668 = -F::new(12.0) * t118640 * t25170 - t13042 * t8363 - t13463 * t8363 - F::new(2.0) * t1912 * t87758 - F::new(2.0) * t1912 * t87810 - F::new(2.0) * t1912 * t87837 + F::new(4.0) * t23281 * t7517 + F::new(4.0) * t2713 * t32800 + F::new(2.0) * t2713 * t32804 + t118639 + t118650 + t118654 - t118662 - t118664 + t118667;
    t118668
}
