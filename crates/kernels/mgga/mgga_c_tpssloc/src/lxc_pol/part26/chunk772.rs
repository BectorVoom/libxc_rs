//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 772/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk772<F: Float>(t2701: F, t820: F, t9616: F, t120: F, t2678: F, t4180: F, t829: F, t2631: F, t2632: F, t776: F, t2645: F, t2646: F, t815: F, t836: F, t812: F, t2649: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9618 = t2701 * t820 * t9616;
    let t9621 = t120 * t2678;
    let t9623 = t4180 * t9621 * t829;
    let t9626 = t120 * t2631;
    let t9627 = t2632 * t776;
    let t9629 = t2645 * t9626 * t9627;
    let t9632 = t2632 * t2678;
    let t9634 = t4180 * t2646 * t9632;
    let t9637 = t815 * t836;
    let t9638 = t812 * t9637;
    let t9639 = t9638 * t2649;
    (t9618, t9621, t9623, t9626, t9627, t9629, t9632, t9634, t9639)
}
