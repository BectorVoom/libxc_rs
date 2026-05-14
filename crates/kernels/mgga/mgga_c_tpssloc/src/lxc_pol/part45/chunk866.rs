//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 866/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk866<F: Float>(t22690: F, t23171: F, t31376: F, t31389: F, t6562: F, t794: F, t23012: F, t8557: F, t112998: F, t113005: F, t113009: F, t114670: F, t114673: F, t114677: F, t114680: F, t114685: F) -> (F,) {
    let t114688 = t23171 * t22690 * t31376;
    let t114689 = 0.82246703342411321824e-2 * t114688;
    let t114691 = t6562 * t794 * t31389;
    let t114693 = t23012 * t8557;
    let t114694 = 0.63969658155208805863e-1 * t114693;
    let t114695 = -t112998 - 0.38381794893125283518e-1 * t114670 + t114673 + 0.16449340668482264365e-1 * t114677 + 0.82246703342411321824e-2 * t114680 - 0.16449340668482264365e-1 * t114685 - t113005 - t113009 - t114689 - 0.82246703342411321824e-2 * t114691 + t114694;
    (t114695,)
}
