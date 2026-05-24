//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 751/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk751<F: Float>(t2604: F, t7779: F, t674: F, t7433: F, t7715: F, t5542: F, t7541: F, t7244: F, t7469: F, t108: F, t4179: F, t490: F) -> (F, F, F, F, F, F) {
    let t35262 = t2604 * t7779;
    let t35265 = t7433 * t7715 * t674;
    let t35276 = t7541 * t5542;
    let t35277 = t35276 * t674;
    let t35285 = t7244 * t7469;
    let t35311 = t4179 * t108;
    let t35312 = t490 * t35311;
    (t35262, t35265, t35276, t35277, t35285, t35312)
}
