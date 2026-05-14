//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 862/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk862<F: Float>(t114601: F, t1888: F, t23270: F, t865: F, t31334: F, t6579: F, t22986: F, t2553: F, t31337: F, t23185: F, t31333: F, t82074: F, t31316: F, t6547: F, t112663: F, t112666: F, t112668: F, t112672: F, t112674: F, t112676: F, t112679: F, t112681: F, t112685: F, t114592: F, t114596: F, t114599: F) -> (F,) {
    let t114604 = t1888 * t23270 * t114601 * t865;
    let t114606 = t6579 * t31334;
    let t114610 = t22986 * t23270 * t31337 * t2553;
    let t114613 = t23185 * t82074 * t31333;
    let t114615 = t6547 * t31316;
    let t114617 = -t112663 - 0.16449340668482264365e-1 * t114592 - 0.49348022005446793095e-1 * t114596 + 0.16449340668482264365e-1 * t114599 + 0.3289868133696452873e-1 * t114604 - t112666 + t112668 - 0.76763589786250567036e-1 * t114606 - t112672 + 0.16449340668482264365e-1 * t114610 - 0.16449340668482264365e-1 * t114613 + t112674 - t112676 - 0.38381794893125283518e-1 * t114615 + t112679 - t112681 - t112685;
    (t114617,)
}
