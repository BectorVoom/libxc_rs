//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1006/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1006<F: Float>(t26223: F, t26364: F, t26485: F, t26500: F, t533: F, t1390: F, t1983: F, t1393: F, t1442: F, t1459: F, t1774: F, t1849: F, t1869: F, t22461: F, t26103: F, t26157: F, t26166: F, t26170: F, t26178: F, t26181: F, t26183: F, t4037: F, t5107: F, t6515: F, t6517: F, t6862: F, t6872: F, t7681: F) -> (F, F, F, F) {
    let t26502 = t26223 + t26364 + t26485 + t26500;
    let t26503 = t533 * t26502;
    let t26504 = t26503 * t1390;
    let t26505 = t1983 * t26504;
    let t26507 = t1393 * t7681 - t1442 * t6862 - 2.0 * t1459 * t22461 - 2.0 * t1459 * t26103 - t1774 * t6515 + t1849 * t6872 - t1869 * t5107 - 2.0 * t4037 * t6517 + t26157 + t26166 + t26170 - t26178 - t26181 - t26183 + t26505;
    (t26502, t26504, t26505, t26507)
}
