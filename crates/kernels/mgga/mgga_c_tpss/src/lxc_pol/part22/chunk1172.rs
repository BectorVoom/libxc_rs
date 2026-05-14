//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1172/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1172<F: Float>(t2435: F, t251: F, t8346: F, t1364: F, t2428: F, t198: F, t2116: F, t1378: F, t8279: F, t2364: F, t2161: F, t1639: F, t3259: F, t3326: F, t10089: F, t1625: F, t3387: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31813 = t2435 * t2435;
    let t31814 = 1.0 / t31813;
    let t32386 = 1.0 / t8346 / t251;
    let t35525 = t1364 * t2428;
    let t35530 = t198 * t2116;
    let t35764 = t1378 * t8279;
    let t36075 = t1378 * t2364;
    let t36098 = t1378 * t2161;
    let t41371 = t1639 * t3259;
    let t41437 = t1639 * t3326;
    let t41590 = t1639 * t10089;
    let t41839 = t1625 * t3387;
    (t31814, t32386, t35525, t35530, t35764, t36075, t36098, t41371, t41437, t41590, t41839)
}
