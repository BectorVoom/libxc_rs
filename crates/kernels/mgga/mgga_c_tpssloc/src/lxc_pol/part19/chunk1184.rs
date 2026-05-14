//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1184/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1184<F: Float>(t909: F, t9709: F, t10310: F, t699: F, t10304: F, t136: F, t41688: F, t2403: F, t2833: F, t2827: F, t10322: F, t10306: F, t41678: F, t41682: F, t41684: F, t41690: F, t41699: F, t41703: F, t41711: F) -> (F, F, F, F, F, F, F, F) {
    let t41863 = t9709 * t909;
    let t41865 = t699 * t10310;
    let t41868 = t136 * t10304 * t41688;
    let t41870 = t2403 * t2833;
    let t41872 = t2403 * t2827;
    let t41874 = t699 * t10322;
    let t41876 = t699 * t10306;
    let t41878 = -0.16102666666666666667e1 * t41678 + 0.24154e1 * t41682 + 0.12524296296296296297e1 * t41684 + 0.40256666666666666666e1 * t41690 - 0.72462e1 * t41699 - 0.60384999999999999999e0 * t41703 + 0.72462e1 * t41711 + 0.98115555555555555556e0 * t41863 - 0.44152e0 * t41865 + 0.44152e0 * t41868 - 0.5519e0 * t41870 - 0.18396666666666666667e0 * t41872 + 0.22076e0 * t41874 + 0.98115555555555555555e-1 * t41876;
    (t41863, t41865, t41868, t41870, t41872, t41874, t41876, t41878)
}
