//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 556/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk556<F: Float>(t15579: F, t5148: F, t14444: F, t558: F, t5266: F, t15096: F, t15099: F, t15107: F, t15110: F, t15112: F, t15114: F, t15120: F, t14478: F, t14481: F, t14484: F, t14487: F, t14490: F, t14493: F, t15101: F, t15103: F, t15116: F, t15118: F, t15122: F) -> (F, F, F) {
    let t15581 = 0.11974241701863808564e0 * t5148 * t15579;
    let t15582 = t14444 * t558;
    let t15584 = 0.11974241701863808564e0 * t5266 * t15582;
    let t15585 = 0.49892673757765869017e-2 * t15096;
    let t15586 = 0.14967802127329760705e-1 * t15099;
    let t15589 = 0.31062809106223861416e-2 * t15107;
    let t15590 = 0.5177134851037310236e-2 * t15110;
    let t15591 = 0.66380770525302906696e-3 * t15112;
    let t15592 = 0.99571155787954360044e-3 * t15114;
    let t15595 = 0.14464861606874801909e-3 * t15120;
    let t15597 = t15585 - t15586 - t14478 - 0.68186654135613354322e-2 * t15101 + 0.13637330827122670864e-1 * t15103 + t14481 + t15589 - t15590 - t14484 + t15591 - t15592 - t14487 - 0.45360193192290319574e-3 * t15116 + 0.63504270469206447404e-3 * t15118 + t14490 + t15595 - 0.19286482142499735878e-3 * t15122 - t14493;
    (t15581, t15584, t15597)
}
