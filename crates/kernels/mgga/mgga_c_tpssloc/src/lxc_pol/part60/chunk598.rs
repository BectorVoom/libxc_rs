//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 598/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk598<F: Float>(t652: F, t8327: F, t1902: F, t225: F, t258: F, t214: F, t1880: F, t1911: F, t6571: F) -> (F, F, F, F, F) {
    let t8328 = t652 * t8327;
    let t8329 = 2.0 * t8328;
    let t8331 = t1902 * t225 * t258;
    let t8332 = t214 * t8331;
    let t8334 = 0.16449340668482264365e-1 * t1880 * t8332;
    let t8335 = t6571 * t1911;
    (t8329, t8331, t8332, t8334, t8335)
}
