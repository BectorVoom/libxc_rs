//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 558/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk558<F: Float>(t15605: F, t82: F, t72: F, t15284: F, t15288: F, t15292: F, t3207: F, t534: F, t3225: F, t8368: F, t22: F, t2447: F, t656: F, t2145: F, t15297: F, t2265: F, t2415: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15606 = t82 * t15605;
    let t15607 = t72 * t15606;
    let t15609 = 0.68186654135613354325e-2 * t15284;
    let t15610 = 0.68186654135613354325e-2 * t15288;
    let t15611 = 0.20455996240684006296e-1 * t15292;
    let t15612 = t534 * t3207;
    let t15613 = t72 * t15612;
    let t15614 = t8368 * t3225;
    let t15615 = 0.34093327067806677161e-2 * t15614;
    let t15616 = t2447 * t22;
    let t15617 = t15616 * t656;
    let t15618 = t2145 * t15617;
    let t15619 = 0.34093327067806677161e-2 * t15618;
    let t15620 = 0.1276937996798935182e-4 * t15297;
    let t15621 = t2415 * t2265;
    (t15606, t15607, t15609, t15610, t15611, t15612, t15613, t15615, t15616, t15617, t15619, t15620, t15621)
}
