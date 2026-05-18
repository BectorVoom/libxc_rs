//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1082/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1082<F: Float>(t6420: F, t7208: F, t6415: F, t1825: F, t27097: F, t1336: F, t24108: F, t24110: F, t26427: F, t26429: F, t26437: F, t28161: F, t28165: F, t28169: F, t28183: F, t5234: F, t7932: F) -> (F, F, F, F) {
    let t29343 = t7208 * t6420;
    let t29345 = t7208 * t6415;
    let t29349 = t27097 * t1825;
    let t29359 = -t1336 * t29343 - t1336 * t29345 - F::new(2.0) * t5234 * t7932 - F::new(2.0) * t1336 * t29349 + F::new(0.16449340668482264365e-1) * t26427 - F::new(0.76763589786250567036e-1) * t26429 - F::new(0.16449340668482264365e-1) * t26437 + F::new(0.16449340668482264365e-1) * t28161 + t24108 + t24110 - F::new(0.3289868133696452873e-1) * t28165 - F::new(0.16449340668482264365e-1) * t28169 - F::new(0.16449340668482264365e-1) * t28183;
    (t29343, t29345, t29349, t29359)
}
