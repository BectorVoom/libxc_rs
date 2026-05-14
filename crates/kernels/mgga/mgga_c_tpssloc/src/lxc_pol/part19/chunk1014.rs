//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1014/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1014<F: Float>(t10189: F, t344: F, t134: F, t2978: F, t10213: F, t60: F, t135: F, t340: F, t6733: F, t884: F, t122: F, t247: F) -> (F, F, F, F, F, F, F, F) {
    let t13779 = t10189 * t344;
    let t13783 = t134 * t2978;
    let t13784 = t13783 * t344;
    let t13797 = t60 * t10213;
    let t13798 = t13797 * t344;
    let t13822 = t135 * t340;
    let t13831 = t6733 * t884;
    let t13969 = t247 * t122;
    (t13779, t13783, t13784, t13797, t13798, t13822, t13831, t13969)
}
