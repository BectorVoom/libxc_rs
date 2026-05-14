//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1358/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1358<F: Float>(t19572: F, t5706: F, t19305: F, t5532: F, t19308: F, t18409: F, t6103: F, t19574: F, t18552: F, t6243: F, t1600: F, t18414: F, t4541: F, t544: F, t5702: F, t65921: F, t65923: F, t65927: F, t65929: F, t65931: F, t65933: F, t65935: F, t65937: F, t65942: F, t65945: F, t65975: F, t66001: F) -> (F,) {
    let t66005 = 2.0 * t5706 * t19572;
    let t66009 = 4.0 * t19305 * t5532;
    let t66011 = 4.0 * t19308 * t5532;
    let t66013 = 4.0 * t6103 * t18409;
    let t66015 = 2.0 * t5706 * t19574;
    let t66017 = 3.0 * t6243 * t18552;
    let t66018 = -t65921 - t65923 - t65927 - t65929 - t65931 - t65933 - t65935 + t65937 + 2.0 * t5702 * t4541 + t65942 + t65945 + (t65975 + t66001) * t544 + t66005 - 2.0 * t18414 * t1600 - t66009 - t66011 - t66013 - t66015 + t66017;
    (t66018,)
}
