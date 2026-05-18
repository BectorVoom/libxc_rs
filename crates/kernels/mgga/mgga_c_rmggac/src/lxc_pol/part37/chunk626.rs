//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 626/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk626<F: Float>(t14980: F, t570: F, t14478: F, t14481: F, t14484: F, t14487: F, t14493: F, t14933: F, t15101: F, t15103: F, t15116: F, t15118: F, t15122: F, t15585: F, t15586: F, t15589: F, t15590: F, t15591: F, t15592: F, t15595: F) -> (F, F) {
    let t15872 = t14980 * t570;
    let t15881 = t15585 - t15586 - t14478 - F::new(0.68186654135613354324e-2) * t15101 + F::new(0.13637330827122670865e-1) * t15103 + t14481 + t15589 - t15590 - t14484 + t15591 - t15592 - t14487 - F::new(0.45360193192290319575e-3) * t15116 + F::new(0.63504270469206447405e-3) * t15118 + t14933 + t15595 - F::new(0.19286482142499735879e-3) * t15122 - t14493;
    (t15872, t15881)
}
