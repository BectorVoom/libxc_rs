//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1327/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1327<F: Float>(t1981: F, t20767: F, t38: F, t20718: F, t7690: F, t18338: F, t18347: F, t18360: F, t18366: F, t19192: F, t19220: F, t19223: F, t19226: F, t19388: F, t20719: F, t20769: F, t20777: F, t5489: F, t5492: F, t5966: F, t6080: F, t6472: F, t65325: F) -> (F,) {
    let t68122 = t1981 * t38 * t20767;
    let t68127 = t7690 * t20718;
    let t68146 = 2.0 / 3.0 * t18338 * t6472 + 5.0 / 3.0 * t68122 * t5489 + 2.0 / 3.0 * t5492 * t20769 - 5.0 * t68127 * t18347 + t6080 * t19220 / 3.0 + 2.0 / 3.0 * t6080 * t19223 + t6080 * t19226 / 3.0 + 5.0 / 6.0 * t20719 * t18360 + t18366 * t6472 / 3.0 + 5.0 / 6.0 * t5966 * t65325 + 5.0 / 3.0 * t19192 * t19388 + 2.0 / 3.0 * t5492 * t20777;
    (t68146,)
}
