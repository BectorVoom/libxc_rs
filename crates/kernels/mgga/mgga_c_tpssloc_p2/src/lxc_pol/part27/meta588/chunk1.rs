//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2046/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2046<F: Float>(t81716: F, t23150: F, t814: F, t133: F, t1891: F, t6601: F, t80953: F, t22816: F, t23104: F, t80967: F, t6612: F, t812: F, t836: F) -> (F, F, F, F, F) {
    let t81717 = F::cast_from(0.98696044010893586188e-1_f64) * t81716;
    let t81718 = t814 * t23150;
    let t81735 = t80953 * t1891 * t133 * t6601;
    let t81736 = F::cast_from(0.69792532988666768264e-2_f64) * t81735;
    let t81742 = t80967 * t1891 * t22816 * t23104;
    let t81743 = F::cast_from(0.43737152435318756759e-3_f64) * t81742;
    let t81749 = t812 * t6612 * t836;
    (t81717, t81718, t81736, t81743, t81749)
}
