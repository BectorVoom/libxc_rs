//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1108/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1108<F: Float>(t12903: F, t12904: F, t12906: F, t12925: F, t219: F, t1226: F, t73: F, t1625: F, t3346: F, t3245: F, t1246: F, t4397: F, t1206: F, t3234: F, t4452: F, t1228: F, t12810: F) -> (F, F, F, F, F, F) {
    let t12928 = (t12903 + t12904 + t12906 + t12925) * t219;
    let t12938 = t1226 * t73;
    let t12943 = t3346 * t1625;
    let t12944 = t12943 * t3245;
    let t12947 = t1246 * t4397;
    let t12948 = t12947 * t1206;
    let t12951 = t4452 * t3234;
    let t12954 = t1228 * t12810;
    (t12928, t12938, t12944, t12948, t12951, t12954)
}
