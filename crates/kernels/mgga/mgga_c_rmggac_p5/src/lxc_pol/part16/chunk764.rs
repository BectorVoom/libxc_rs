//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 764/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk764<F: Float>(t1343: F, t2040: F, t638: F, t71: F, t830: F, t2046: F, t2051: F, t271: F, t4773: F, t7292: F, t7385: F, t2067: F, t25640: F) -> (F, F, F, F) {
    let t35781 = t638 * t830 * t1343 * t71 * t2040;
    let t35786 = t2046 * t4773 * t271 * t71 * t2051;
    let t35798 = t638 * t7292 * t7385;
    let t35810 = t25640 * t2067;
    (t35781, t35786, t35798, t35810)
}
