//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 889/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk889<F: Float>(t31537: F, t7057: F, t22479: F, t89: F, t2040: F, t31540: F, t7050: F, t2314: F, t31747: F, t531: F, t8639: F, t1983: F, t22596: F, t115227: F, t115229: F, t115231: F, t115233: F, t115238: F, t115241: F, t115245: F, t115249: F, t1976: F, t2036: F, t2075: F, t22600: F, t23829: F, t23909: F, t23917: F, t6517: F, t652: F, t672: F, t83935: F) -> (F,) {
    let t115251 = 4.0 * t31537 * t7057;
    let t115252 = t89 * t22479;
    let t115254 = 2.0 * t115252 * t2040;
    let t115256 = 4.0 * t31540 * t7050;
    let t115261 = 4.0 * t2314 * t31747;
    let t115262 = t531 * t8639;
    let t115265 = 6.0 * t1983 * t115262 * t22596;
    let t115267 = -2.0 * t1976 * t23917 * t652 - 4.0 * t115241 * t672 - t2036 * t23829 - 2.0 * t2040 * t83935 - 2.0 * t2075 * t22600 - 2.0 * t23909 * t6517 + t115227 - t115229 - t115231 - t115233 + t115238 + t115245 - t115249 - t115251 - t115254 - t115256 - t115261 + t115265;
    (t115267,)
}
