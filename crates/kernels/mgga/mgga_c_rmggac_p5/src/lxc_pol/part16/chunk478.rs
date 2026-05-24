//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 478/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk478<F: Float>(t5982: F, t5995: F, t6005: F, t6015: F, t1847: F, t453: F, t1839: F, t446: F, t1392: F, t589: F, t1835: F, t1794: F, t4396: F) -> (F, F, F, F, F, F) {
    let t6017 = t5982 + t5995 + t6005 + t6015;
    let t6020 = t1847 * t453;
    let t6031 = t1839 * t446;
    let t6034 = t589 * t1392;
    let t6039 = t1835 * t446;
    let t6042 = t4396 * t1794;
    (t6017, t6020, t6031, t6034, t6039, t6042)
}
