//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 926/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk926<F: Float>(t10667: F, t2389: F, t774: F, t1364: F, t2116: F, t8162: F, t2169: F, t3667: F, t1381: F, t8286: F, t10470: F, t10471: F, t10472: F, t10500: F, t10501: F, t7929: F, t7932: F, t7936: F, t7945: F, t8000: F, t8001: F, t8019: F, t8023: F, t8029: F, t8040: F) -> (F, F, F, F, F, F) {
    let t10669 = t2389 * t774 * t10667;
    let t10672 = t1364 * t2116;
    let t10674 = t8162 * t774 * t10672;
    let t10678 = 7.0 / 2304.0 * t2169 * t3667;
    let t10679 = t8286 * t1381;
    let t10681 = t8000 + t8001 - t10470 - t8019 + t8023 + t10471 - t8029 - t10472 - t8040 + t10500 + t10501 + t7929 - t7932 - t7936 + t7945;
    (t10669, t10672, t10674, t10678, t10679, t10681)
}
