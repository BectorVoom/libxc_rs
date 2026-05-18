//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 980/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk980<F: Float>(t114285: F, t22633: F, t28116: F, t120269: F, t120276: F, t120296: F, t6431: F, t8466: F, t1831: F, t32717: F, t6427: F, t31170: F, t6396: F) -> (F, F, F, F, F, F, F, F) {
    let t127220 = F::new(0.6579736267392905746e-1) * t22633 * t114285 * t28116;
    let t127229 = F::new(0.76763589786250567036e-1) * t120269;
    let t127242 = F::new(0.15352717957250113407e0) * t120276;
    let t127249 = F::new(0.16449340668482264365e-1) * t120296;
    let t127252 = t8466 * t6431;
    let t127254 = t32717 * t1831;
    let t127256 = t8466 * t6427;
    let t127258 = t31170 * t6396;
    (t127220, t127229, t127242, t127249, t127252, t127254, t127256, t127258)
}
