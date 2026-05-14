//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1129/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1129<F: Float>(t12097: F, t12100: F, t12111: F, t12120: F, t15968: F, t184: F, t17: F, t2663: F, t5157: F, t12103: F, t12105: F, t12109: F, t12114: F, t12116: F, t12118: F, t12123: F, t12477: F, t15970: F, t15972: F, t1799: F, t3719: F, t3918: F, t5122: F, t9797: F, t9820: F, t9824: F) -> (F, F, F, F, F, F, F) {
    let t15973 = 0.4883052614935078681e-3 * t12097;
    let t15974 = 0.18311447306006545054e-3 * t12100;
    let t15975 = 0.21687162600603479684e-1 * t12111;
    let t15976 = 4.0 * t12120;
    let t15977 = t15968 * t184;
    let t15978 = t17 * t15977;
    let t15979 = t5157 * t2663;
    let t15980 = 0.24415263074675393405e-3 * t15979;
    let t15981 = -3.0 * t12477 * t1799 * t3918 + 3.0 * t3719 * t3918 * t5122 + t12103 - t12105 - t12109 - t12114 + t12116 + t12118 + t12123 + t15970 + t15972 + t15973 - t15974 + t15975 + t15976 + t15978 + t15980 + t9797 - t9820 - t9824;
    (t15973, t15974, t15975, t15976, t15978, t15980, t15981)
}
