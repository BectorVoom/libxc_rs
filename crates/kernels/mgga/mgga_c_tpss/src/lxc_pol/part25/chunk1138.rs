//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1138/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1138<F: Float>(t226: F, t6337: F, t782: F, t5577: F, t1708: F, t20446: F, t228: F, t1707: F, t17993: F, t18006: F, t19767: F, t20471: F, t20475: F, t20479: F, t20483: F, t20488: F, t20492: F, t20494: F, t20498: F, t5568: F, t5571: F, t6348: F, t6351: F) -> (F, F, F) {
    let t20502 = t6337 * t782 * t226;
    let t20503 = t5577 * t20502;
    let t20506 = t1708 * t228 * t20446;
    let t20508 = -t1707 * t20506 + t17993 * t6348 - 2.0 * t18006 * t20479 - 2.0 * t19767 * t20483 + t19767 * t20494 + 2.0 * t20471 * t5571 + 2.0 * t20475 * t5571 + t20488 * t5571 + t20492 * t5571 + 2.0 * t20498 * t5571 + t20503 * t5571 - t5568 * t6351;
    (t20503, t20506, t20508)
}
