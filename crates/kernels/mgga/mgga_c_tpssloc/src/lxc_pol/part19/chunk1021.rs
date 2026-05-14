//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1021/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1021<F: Float>(t3014: F, t343: F, t12461: F, t3698: F, t3475: F, t460: F, t20: F, t60: F, t9108: F, t94: F, t102: F, t9174: F, t12512: F, t580: F, t1404: F, t3931: F) -> (F, F, F, F, F, F, F, F) {
    let t23547 = t3014 * t343;
    let t23857 = t12461 * t3698;
    let t24705 = t3475 * t460;
    let t32253 = 1.0 / t60 / t20;
    let t35577 = t94 * t9108;
    let t35761 = t102 * t9174;
    let t39022 = t12512 * t580;
    let t39024 = t3931 * t1404;
    (t23547, t23857, t24705, t32253, t35577, t35761, t39022, t39024)
}
