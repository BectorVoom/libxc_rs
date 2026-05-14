//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 199/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk199<F: Float>(t453: F, t592: F, t1152: F, t1157: F, t1392: F, t1430: F, t1439: F, t198: F, t446: F, t454: F, t589: F, t201: F, t228: F, t457: F, t597: F, t461: F, t615: F) -> (F, F, F, F, F, F) {
    let t1442 = t592 * t453;
    let t1451 = -0.32163648644302209643e2 * t1439 * t198 + 0.96490945932906628929e2 * t1442 * t446 + 0.96490945932906628929e2 * t1152 * t589 - 0.38596378373162651572e3 * t1157 * t1430 + 0.96490945932906628929e2 * t454 * t1392;
    let t1452 = t1451 * t201;
    let t1453 = t1452 * t228;
    let t1454 = t597 * t457;
    let t1455 = t201 * t228;
    let t1459 = t461 * t615;
    (t1451, t1452, t1453, t1454, t1455, t1459)
}
