//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1228/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1228<F: Float>(t1905: F, t81686: F, t9537: F, t2587: F, t81151: F, t23172: F, t133: F, t1891: F, t6601: F, t80953: F, t22816: F, t23104: F, t80967: F) -> (F, F, F, F, F) {
    let t81688 = t81686 * t9537 * t1905;
    let t81689 = F::new(0.13707783890401886971e-2) * t81688;
    let t81715 = t81151 * t2587;
    let t81716 = t81715 * t23172;
    let t81717 = F::new(0.98696044010893586188e-1) * t81716;
    let t81735 = t80953 * t1891 * t133 * t6601;
    let t81736 = F::new(0.69792532988666768264e-2) * t81735;
    let t81742 = t80967 * t1891 * t22816 * t23104;
    (t81689, t81715, t81717, t81736, t81742)
}
