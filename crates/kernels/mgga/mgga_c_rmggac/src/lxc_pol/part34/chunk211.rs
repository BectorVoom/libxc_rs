//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 211/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk211<F: Float>(t1433: F, t1438: F, t453: F, t592: F, t1152: F, t1157: F, t1392: F, t1430: F, t198: F, t446: F, t454: F, t589: F) -> (F, F) {
    let t1439 = t1433 + t1438;
    let t1442 = t592 * t453;
    let t1451 = -F::new(0.32163648644302209643e2) * t1439 * t198 + F::new(0.96490945932906628929e2) * t1442 * t446 + F::new(0.96490945932906628929e2) * t1152 * t589 - F::new(0.38596378373162651572e3) * t1157 * t1430 + F::new(0.96490945932906628929e2) * t454 * t1392;
    (t1439, t1451)
}
