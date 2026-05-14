//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1234/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1234<F: Float>(t1219: F, t6419: F, t1265: F, t12957: F, t13108: F, t1838: F, t18483: F, t18490: F, t18496: F, t18499: F, t18967: F, t18968: F, t19521: F, t19535: F, t20182: F, t20187: F, t20190: F, t20196: F, t20202: F, t3366: F, t3384: F, t4516: F, t520: F, t5739: F, t5740: F, t5745: F, t5918: F, t60649: F, t60653: F, t60811: F, t62508: F, t6424: F, t65691: F, t65696: F, t65711: F, t65715: F, t65719: F, t65722: F, t65867: F, t65871: F) -> (F,) {
    let t66970 = t1219 * t6419;
    let t66998 = -12.0 * t5739 * t18490 * t20182 * t1265 - 2.0 * t18496 * t18967 * t65711 - 4.0 * t18496 * t62508 * t19521 - 4.0 * t18496 * t18967 * t65691 + 2.0 * t5739 * t5740 * t1838 * t13108 - 4.0 * t65871 * t18968 + 4.0 * t18496 * t20190 * t65696 + 6.0 * t60653 * t18967 * t65722 + 4.0 * t5739 * t5740 * t5918 * t4516 - 2.0 * t18496 * t18967 * t65715 - 4.0 * t18496 * t66970 * t18499 - 4.0 * t60649 * t20187 + 2.0 * t65719 * t20202 - 4.0 * t18496 * t62508 * t19535 - 4.0 * t18496 * t18967 * t65867 + 24.0 * t5739 * t60811 * t6424 * t3366 + t5739 * t5745 * t1838 * t12957 * t520 + 2.0 * t18483 * t20196 + 2.0 * t5739 * t5740 * t6419 * t3384;
    (t66998,)
}
