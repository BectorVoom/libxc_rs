//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1154/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1154<F: Float>(t1165: F, t13133: F, t1338: F, t13554: F, t1799: F, t18898: F, t19305: F, t19656: F, t20288: F, t20289: F, t20294: F, t20319: F, t2056: F, t3493: F, t3537: F, t4347: F, t5801: F, t5815: F, t6234: F, t6323: F, t645: F) -> (F,) {
    let t20322 = 2.0 * t1165 * t20319 + 2.0 * t13133 * t1799 + 2.0 * t1338 * t18898 + 2.0 * t1338 * t20294 + 2.0 * t13554 * t1799 + 2.0 * t1799 * t19305 + 2.0 * t1799 * t19656 + 2.0 * t20289 * t645 + 2.0 * t2056 * t6323 + 2.0 * t3493 * t5815 + 2.0 * t3537 * t5801 + 2.0 * t4347 * t6323 + 2.0 * t5815 * t6234 + t20288;
    (t20322,)
}
