//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1254/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1254<F: Float>(t1378: F, t8279: F, t2364: F, t2161: F, t65: F, t8491: F, t1639: F, t3259: F, t3326: F, t10089: F, t1625: F, t3387: F, t13111: F, t3205: F, t10444: F, t116: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35764 = t1378 * t8279;
    let t36075 = t1378 * t2364;
    let t36098 = t1378 * t2161;
    let t36839 = t65 * t8491;
    let t41371 = t1639 * t3259;
    let t41437 = t1639 * t3326;
    let t41590 = t1639 * t10089;
    let t41839 = t1625 * t3387;
    let t41867 = t13111 * t3205;
    let t41905 = t10444 * t116;
    (t35764, t36075, t36098, t36839, t41371, t41437, t41590, t41839, t41867, t41905)
}
