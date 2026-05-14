//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 539/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk539<F: Float>(t2798: F, t2799: F, t2764: F, t2766: F, t2773: F, t2778: F, t2782: F) -> (F, F) {
    let t2800 = t2798 * t2799;
    let t2802 = 4.0 / 9.0 * t2764;
    let t2807 = t2802 + 2.0 / 9.0 * t2766 - 2.0 / 9.0 * t2773 + 2.0 / 3.0 * t2778 - t2782 / 3.0;
    (t2800, t2807)
}
