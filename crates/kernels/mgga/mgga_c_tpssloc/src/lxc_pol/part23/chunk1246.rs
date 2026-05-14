//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1246/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1246<F: Float>(t21589: F, t4483: F, t77119: F, t77122: F, t77124: F, t77127: F, t77130: F, t77133: F, t77135: F, t77138: F, t77143: F, t77145: F, t77148: F, t17934: F, t5808: F, t10523: F, t76637: F, t951: F, t959: F) -> (F, F, F, F) {
    let t77150 = 0.23392894490538584828e1 * t4483 * t21589;
    let t77151 = t77119 - t77122 - t77124 + t77127 - t77130 - t77133 + t77135 + t77138 + t77143 - t77145 - t77148 - t77150;
    let t77153 = 0.35089341735807877242e1 * t17934 * t5808;
    let t77157 = 0.14035736694323150897e2 * t959 * t10523 * t76637 * t951;
    (t77150, t77151, t77153, t77157)
}
