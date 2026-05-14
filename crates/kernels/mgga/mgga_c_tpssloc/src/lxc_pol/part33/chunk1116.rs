//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1116/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1116<F: Float>(t81688: F, t2587: F, t81151: F, t23172: F, t133: F, t1891: F, t6601: F, t80953: F, t22816: F, t23104: F, t80967: F, t6612: F, t812: F, t836: F, t2690: F, t6619: F) -> (F, F, F, F, F, F, F) {
    let t81689 = 0.13707783890401886971e-2 * t81688;
    let t81715 = t81151 * t2587;
    let t81716 = t81715 * t23172;
    let t81717 = 0.98696044010893586188e-1 * t81716;
    let t81735 = t80953 * t1891 * t133 * t6601;
    let t81736 = 0.69792532988666768264e-2 * t81735;
    let t81742 = t80967 * t1891 * t22816 * t23104;
    let t81743 = 0.43737152435318756759e-3 * t81742;
    let t81749 = t812 * t6612 * t836;
    let t81763 = t812 * t6619 * t2690;
    (t81689, t81715, t81717, t81736, t81743, t81749, t81763)
}
