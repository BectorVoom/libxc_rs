//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 755/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk755<F: Float>(t2019: F, t640: F, t70901: F, t7764: F, t70610: F, t70612: F, t14372: F, t15262: F, t16156: F, t15254: F, t14229: F, t8576: F) -> (F, F, F, F, F, F, F) {
    let t72177 = t2019 * t7764 * t640 * t70901;
    let t72178 = F::new(0.30487649791575028314e-3) * t72177;
    let t72192 = F::new(0.6505345598561924296e-5) * t70610;
    let t72193 = F::new(0.30487649791575028312e-3) * t70612;
    let t72207 = F::new(0.81756761766873046872e-5) * t14372;
    let t73688 = t16156 * t15262;
    let t73690 = t16156 * t15254;
    let t73691 = F::new(0.19863479950205658386e-4) * t73690;
    let t73692 = t8576 * t14229;
    (t72178, t72192, t72193, t72207, t73688, t73691, t73692)
}
