//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 776/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk776<F: Float>(t1401: F, t1458: F, t2039: F, t3941: F, t5371: F, t577: F, t7230: F, t7801: F, t7945: F, t7956: F, t1409: F, t1419: F, t56: F, t6503: F, t7251: F, t67: F) -> (F, F, F) {
    let t7961 = 0.45e1 * t7945 * t577 + 0.135e2 * t7230 * t1458 + 0.135e2 * t5371 * t2039 + 27.0 * t3941 * t7956 + 0.135e2 * t1401 * t7801;
    let t7973 = -8.0 / 3.0 * t1419 * t56 - 5.0 / 6.0 * t7251 * t1409 + t6503;
    let t7974 = t7973 * t67;
    (t7961, t7973, t7974)
}
