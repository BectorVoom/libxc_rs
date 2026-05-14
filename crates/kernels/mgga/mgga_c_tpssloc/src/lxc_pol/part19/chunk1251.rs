//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1251/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1251<F: Float>(t11018: F, t225: F, t3206: F, t11016: F, t10160: F, t10170: F, t10182: F, t10358: F, t1049: F, t1052: F, t1066: F, t11007: F, t11010: F, t11085: F, t3020: F, t3026: F, t3166: F, t3169: F, t3174: F, t3176: F, t3207: F, t349: F, t388: F, t43419: F, t990: F) -> (F,) {
    let t43431 = t11018 * t225;
    let t43436 = t3206 * t3206;
    let t43440 = t11016 * t225;
    let t43447 = 4.0 * t10358 * t1049 * t388 + 6.0 * t1052 * t3174 * t43436 + 4.0 * t11007 * t388 * t990 + 6.0 * t3020 * t3166 * t388 + t349 * t388 * t43419 + 24.0 * t10160 * t3176 + 12.0 * t10170 * t3176 + 24.0 * t10182 * t3026 + 24.0 * t10182 * t3169 - 12.0 * t1066 * t43431 - 4.0 * t1066 * t43440 - 6.0 * t11010 * t3207 - 4.0 * t11085 * t3169;
    (t43447,)
}
