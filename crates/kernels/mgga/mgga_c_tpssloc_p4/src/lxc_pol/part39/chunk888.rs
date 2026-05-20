//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 888/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk888<F: Float>(t334: F, t371: F, t533: F, t556: F, t1184: F, t460: F, t1458: F, t89: F, t88: F) -> (F, F, F, F, F) {
    let t6793 = t371 * t334;
    let t6924 = F::new(1.0) / t556 / t533;
    let t7319 = t1184 * t460;
    let t7458 = t89 * t1458;
    let t7676 = t88 * t1458;
    (t6793, t6924, t7319, t7458, t7676)
}
