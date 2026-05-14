//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1167/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1167<F: Float>(t22734: F, t81159: F, t1352: F, t26331: F, t3734: F, t562: F, t6976: F, t22633: F, t81052: F, t1992: F, t22897: F, t3792: F, t81094: F, t40475: F, t550: F, t81028: F) -> (F, F, F, F, F, F) {
    let t81160 = t81159 * t22734;
    let t81165 = t26331 * t6976 * t562 * t3734 * t1352;
    let t81169 = t22633 * t6976 * t81052 * t1352;
    let t81173 = t1992 * t22897 * t81094 * t3792;
    let t81177 = t1992 * t6976 * t40475 * t550;
    let t81181 = t1992 * t22897 * t81028 * t3792;
    (t81160, t81165, t81169, t81173, t81177, t81181)
}
