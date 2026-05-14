//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1112/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1112<F: Float>(t15636: F, t4582: F, t13969: F, t4979: F, t3506: F, t4973: F, t1227: F, t11705: F, t11719: F, t11728: F, t11734: F, t11746: F, t15610: F, t15612: F, t15617: F, t15622: F, t15627: F, t15631: F, t3490: F, t3496: F, t3515: F, t4974: F, t4984: F, t5019: F) -> (F,) {
    let t15637 = t4582 * t15636;
    let t15640 = t13969 * t4979;
    let t15642 = t3506 * t15640 / 1152.0;
    let t15643 = t13969 * t4973;
    let t15645 = t1227 * t15643 / 1728.0;
    let t15648 = -t11705 / 3456.0 - t5019 * t3496 / 576.0 + t11746 / 2304.0 - t15610 - t1227 * t15612 / 2304.0 - t1227 * t15617 / 768.0 + t3506 * t15622 / 1536.0 + t11719 * t15627 / 512.0 - t11728 * t15631 / 512.0 - t11734 * t4984 / 1536.0 - t3515 * t15637 / 1536.0 + t15642 - t15645 - t3490 * t4974 / 1152.0;
    (t15648,)
}
