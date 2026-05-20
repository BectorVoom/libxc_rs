//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2045/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2045<F: Float>(t1905: F, t81686: F, t9537: F, t23004: F, t23110: F, t23185: F, t23005: F, t6579: F, t23181: F, t2587: F, t81151: F, t23172: F) -> (F, F, F, F, F, F) {
    let t81688 = t81686 * t9537 * t1905;
    let t81689 = F::cast_from(0.13707783890401886971e-2_f64) * t81688;
    let t81691 = t23185 * t23110 * t23004;
    let t81697 = t6579 * t23005;
    let t81704 = t6579 * t23181;
    let t81715 = t81151 * t2587;
    let t81716 = t81715 * t23172;
    (t81689, t81691, t81697, t81704, t81715, t81716)
}
