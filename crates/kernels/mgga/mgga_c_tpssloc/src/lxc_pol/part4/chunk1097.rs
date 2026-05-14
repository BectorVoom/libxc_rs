//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1097/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1097<F: Float>(t19804: F, t562: F, t1372: F, t6361: F, t225: F, t6435: F, t1323: F, t6434: F, t1385: F, t6439: F, t12021: F, t6362: F, t1375: F, t1386: F, t16022: F, t16460: F, t1843: F, t3758: F, t3882: F, t5215: F, t5326: F, t5354: F, t568: F, t6440: F, t6461: F) -> (F,) {
    let t20038 = t19804 * t562;
    let t20040 = t6361 * t1372;
    let t20044 = t6435 * t225;
    let t20048 = t1323 * t6434;
    let t20050 = t6439 * t1385;
    let t20051 = t12021 * t20050;
    let t20060 = t6362 * t225;
    let t20062 = -6.0 * t1375 * t20051 - t1386 * t20044 - t1386 * t20060 - 2.0 * t16022 * t1843 - 2.0 * t16460 * t1843 + t20038 * t568 + t20040 * t568 + t20048 * t568 + 2.0 * t3758 * t6440 - t3758 * t6461 + 2.0 * t3882 * t6440 + 4.0 * t5215 * t5326 - 2.0 * t5215 * t5354;
    (t20062,)
}
