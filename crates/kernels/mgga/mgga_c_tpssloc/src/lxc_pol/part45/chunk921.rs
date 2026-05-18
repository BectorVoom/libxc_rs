//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 921/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk921<F: Float>(t31059: F, t214: F, t6624: F, t1880: F, t6572: F, t23218: F, t30663: F, t30657: F, t6547: F, t22986: F, t23270: F, t30633: F, t87036: F) -> (F, F, F, F, F, F) {
    let t112622 = F::new(4.0) * t31059;
    let t112660 = t214 * t6624;
    let t112663 = F::new(0.3289868133696452873e-1) * t1880 * t112660 * t6572;
    let t112666 = F::new(0.16449340668482264365e-1) * t1880 * t30663 * t23218;
    let t112667 = t6547 * t30657;
    let t112668 = F::new(0.76763589786250567036e-1) * t112667;
    let t112672 = F::new(0.13159472534785811492e0) * t22986 * t23270 * t30633 * t87036;
    (t112622, t112660, t112663, t112666, t112668, t112672)
}
