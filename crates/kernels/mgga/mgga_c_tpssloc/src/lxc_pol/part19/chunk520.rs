//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 520/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk520<F: Float>(t2742: F, t858: F, t259: F, t2592: F, t2594: F, t2597: F, t2711: F, t2713: F, t2720: F, t855: F, t866: F) -> (F, F) {
    let t2743 = t858 * t2742;
    let t2745 = t259 * t2592 + 2.0 * t259 * t2594 + t259 * t2711 - 2.0 * t2597 * t866 - 2.0 * t2713 * t866 + 2.0 * t2720 * t855 - t2743 * t855;
    (t2743, t2745)
}
