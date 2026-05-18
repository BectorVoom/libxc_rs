//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 764/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk764<F: Float>(t15388: F, t68538: F, t3154: F, t38638: F, t15266: F, t16156: F, t21719: F, t35155: F, t9183: F, t236: F, t446: F, t615: F) -> (F, F, F, F, F) {
    let t73814 = t68538 * t15388;
    let t73816 = t38638 * t3154;
    let t73817 = F::new(0.19863479950205658386e-4) * t73816;
    let t73819 = t16156 * t15266;
    let t73822 = t21719 * t35155 * t9183;
    let t73825 = t236 * t615 * t446;
    (t73814, t73817, t73819, t73822, t73825)
}
