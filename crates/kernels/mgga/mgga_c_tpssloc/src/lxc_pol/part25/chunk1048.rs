//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1048/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1048<F: Float>(t2319: F, t7039: F, t22550: F, t7031: F, t22549: F, t2031: F, t83728: F, t83737: F, t607: F, t63: F, t39054: F, t7025: F, t23966: F, t9231: F, t6492: F, t22527: F, t23967: F) -> (F, F, F, F, F, F, F, F) {
    let t84149 = t7039 * t2319;
    let t84173 = t7031 * t22550;
    let t84174 = t22549 * t84173;
    let t84180 = t2031 * t83728;
    let t84183 = t2031 * t83737;
    let t84186 = t607 * t63;
    let t84190 = t39054 * t7025;
    let t84195 = t9231 * t23966;
    let t84196 = t84195 * t6492;
    let t84198 = t23967 * t22527;
    (t84149, t84174, t84180, t84183, t84186, t84190, t84196, t84198)
}
