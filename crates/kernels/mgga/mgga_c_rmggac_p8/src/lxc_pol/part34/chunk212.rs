//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 212/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk212<F: Float>(t1451: F, t201: F, t228: F, t457: F, t597: F, t461: F, t615: F, t495: F, t1180: F, t31: F, t217: F, t673: F) -> (F, F, F, F, F, F, F, F) {
    let t1452 = t1451 * t201;
    let t1453 = t1452 * t228;
    let t1454 = t597 * t457;
    let t1455 = t201 * t228;
    let t1459 = t461 * t615;
    let t1462 = t615 * t495;
    let t1465 = t1180 * t31;
    let t1466 = t673 * t217;
    (t1452, t1453, t1454, t1455, t1459, t1462, t1465, t1466)
}
