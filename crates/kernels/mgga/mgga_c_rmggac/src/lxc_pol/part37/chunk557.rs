//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 557/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk557<F: Float>(t14444: F, t558: F, t5266: F, t15096: F, t15099: F, t15107: F, t15110: F, t15112: F, t15114: F, t15120: F, t15284: F, t15288: F, t15292: F, t3225: F, t8368: F, t22: F, t2447: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15582 = t14444 * t558;
    let t15584 = 0.11974241701863808564e0 * t5266 * t15582;
    let t15585 = 0.49892673757765869017e-2 * t15096;
    let t15586 = 0.14967802127329760705e-1 * t15099;
    let t15589 = 0.31062809106223861416e-2 * t15107;
    let t15590 = 0.5177134851037310236e-2 * t15110;
    let t15591 = 0.66380770525302906696e-3 * t15112;
    let t15592 = 0.99571155787954360044e-3 * t15114;
    let t15595 = 0.14464861606874801909e-3 * t15120;
    let t15609 = 0.68186654135613354325e-2 * t15284;
    let t15610 = 0.68186654135613354325e-2 * t15288;
    let t15611 = 0.20455996240684006296e-1 * t15292;
    let t15614 = t8368 * t3225;
    let t15615 = 0.34093327067806677161e-2 * t15614;
    let t15616 = t2447 * t22;
    (t15584, t15585, t15586, t15589, t15590, t15591, t15592, t15595, t15609, t15610, t15611, t15615, t15616)
}
