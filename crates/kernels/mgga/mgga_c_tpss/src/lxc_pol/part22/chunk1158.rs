//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1158/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1158<F: Float>(t12903: F, t12904: F, t12906: F, t12925: F, t219: F, t1226: F, t73: F, t1625: F, t3346: F, t3245: F, t1246: F, t4397: F) -> (F, F, F, F) {
    let t12928 = (t12903 + t12904 + t12906 + t12925) * t219;
    let t12938 = t1226 * t73;
    let t12943 = t3346 * t1625;
    let t12944 = t12943 * t3245;
    let t12947 = t1246 * t4397;
    (t12928, t12938, t12944, t12947)
}
