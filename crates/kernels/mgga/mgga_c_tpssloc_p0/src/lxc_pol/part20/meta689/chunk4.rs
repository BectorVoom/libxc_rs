//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2615/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2615<F: Float>(t11692: F, t11697: F, t15703: F, t11702: F, t5019: F, t3516: F, t607: F, t1734: F, t3493: F, t15458: F, t3577: F, t15462: F) -> (F, F, F, F, F, F) {
    let t53135 = t11692 * t11697 * t15703;
    let t53142 = t5019 * t11702;
    let t53144 = t3516 * t607;
    let t53149 = t1734 * t3493;
    let t53155 = t3577 * t11697 * t15458;
    let t53158 = t3577 * t11697 * t15462;
    (t53135, t53142, t53144, t53149, t53155, t53158)
}
