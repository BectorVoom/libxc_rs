//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 623/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk623<F: Float>(t15110: F, t15112: F, t15114: F, t15120: F, t14478: F, t14481: F, t14484: F, t14487: F, t14490: F, t14493: F, t15101: F, t15103: F, t15116: F, t15118: F, t15122: F, t15585: F, t15586: F, t15589: F) -> F {
    let t15590 = F::cast_from(0.5177134851037310236e-2_f64) * t15110;
    let t15591 = F::cast_from(0.66380770525302906696e-3_f64) * t15112;
    let t15592 = F::cast_from(0.99571155787954360044e-3_f64) * t15114;
    let t15595 = F::cast_from(0.14464861606874801909e-3_f64) * t15120;
    let t15597 = t15585 - t15586 - t14478 - F::cast_from(0.68186654135613354322e-2_f64) * t15101 + F::cast_from(0.13637330827122670864e-1_f64) * t15103 + t14481 + t15589 - t15590 - t14484 + t15591 - t15592 - t14487 - F::cast_from(0.45360193192290319574e-3_f64) * t15116 + F::cast_from(0.63504270469206447404e-3_f64) * t15118 + t14490 + t15595 - F::cast_from(0.19286482142499735878e-3_f64) * t15122 - t14493;
    t15597
}
