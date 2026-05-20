//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 895/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk895<F: Float>(t25: F, t1268: F, t2312: F, t2314: F, t2319: F, t2363: F, t671: F, t88: F, t526: F, t606: F, t2249: F, t514: F, t528: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t3660 = F::new(2.0) * t1268 * t2363 + F::new(4.0) * t2314 * t671 + F::new(2.0) * t2319 * t88 + t2312;
    let t3664 = F::new(1.0) / t526;
    let t3665 = t606 * t606;
    let t3671 = piecewise3::<F>(t26, F::new(0.0), F::new(4.0) / F::new(9.0) * t3664 * t3665 + F::new(4.0) / F::new(3.0) * t514 * t2249);
    let t3672 = F::new(1.0) / t528;
    (t3660, t3664, t3665, t3671, t3672)
}
