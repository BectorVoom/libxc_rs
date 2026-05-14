//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 677/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk677<F: Float>(t507: F, t541: F, t1196: F, t1270: F, t198: F, t2292: F, t2302: F, t3213: F, t3216: F, t3234: F, t3245: F, t3281: F, t3299: F, t3302: F, t3304: F, t3307: F, t3310: F, t3312: F, t3387: F, t509: F) -> (F,) {
    let t3391 = t507 * t541;
    let t3395 = t1270 * t198 * t3387 * t509 + 3.0 * t1196 * t198 * t3234 + 6.0 * t198 * t3245 * t3391 - t2292 + t2302 + t3213 - t3216 + t3281 + t3299 + t3302 + t3304 + t3307 + t3310 + t3312;
    (t3395,)
}
