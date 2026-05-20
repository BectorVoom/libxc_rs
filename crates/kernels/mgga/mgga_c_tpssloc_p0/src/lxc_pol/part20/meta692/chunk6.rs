//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2641/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2641<F: Float>(t15908: F, t9467: F, t9882: F, t118: F, t2375: F, t5151: F, t16169: F, t2663: F, t1388: F, t3734: F, t15892: F, t2371: F) -> (F, F, F, F, F, F) {
    let t53777 = t15908 * t9467;
    let t53778 = F::cast_from(0.21687162600603479684e-1_f64) * t53777;
    let t53779 = t15908 * t9882;
    let t53780 = F::cast_from(0.32530743900905219526e-1_f64) * t53779;
    let t53782 = t5151 * t118 * t2375;
    let t53783 = F::cast_from(0.32530743900905219526e-1_f64) * t53782;
    let t53787 = t16169 * t2663;
    let t53788 = F::cast_from(0.73245789224026180216e-3_f64) * t53787;
    let t53789 = t1388 * t3734;
    let t53796 = t15892 * t2371;
    (t53778, t53780, t53783, t53788, t53789, t53796)
}
