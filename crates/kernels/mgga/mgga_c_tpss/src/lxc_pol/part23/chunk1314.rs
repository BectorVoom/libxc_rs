//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1314/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1314<F: Float>(t1688: F, t41905: F, t42719: F, t13133: F, t5531: F, t42336: F, t19572: F, t5706: F, t19305: F, t5532: F, t19308: F, t18409: F, t6103: F, t19574: F, t18552: F, t6243: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t65993 = 2.0 * t41905 * t1688;
    let t65995 = 4.0 * t42719 * t1688;
    let t65997 = 4.0 * t13133 * t5531;
    let t65999 = 2.0 * t42336 * t1688;
    let t66005 = 2.0 * t5706 * t19572;
    let t66009 = 4.0 * t19305 * t5532;
    let t66011 = 4.0 * t19308 * t5532;
    let t66013 = 4.0 * t6103 * t18409;
    let t66015 = 2.0 * t5706 * t19574;
    let t66017 = 3.0 * t6243 * t18552;
    (t65993, t65995, t65997, t65999, t66005, t66009, t66011, t66013, t66015, t66017)
}
