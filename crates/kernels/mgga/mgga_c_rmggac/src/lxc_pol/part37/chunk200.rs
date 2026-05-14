//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 200/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk200<F: Float>(t495: F, t615: F, t1180: F, t31: F, t217: F, t673: F, t1184: F, t605: F, t476: F) -> (F, F, F, F) {
    let t1462 = t615 * t495;
    let t1465 = t1180 * t31;
    let t1466 = t673 * t217;
    let t1467 = t1465 * t1466;
    let t1468 = t605 * t1184;
    let t1469 = t1468 * t476;
    (t1462, t1466, t1467, t1469)
}
