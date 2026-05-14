//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1049/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1049<F: Float>(t22531: F, t23967: F, t22546: F, t22549: F, t23963: F, t23970: F, t605: F, t83722: F, t83745: F, t83778: F, t83820: F, t84174: F, t84180: F, t84183: F, t84186: F, t84190: F, t84196: F, t84198: F) -> (F,) {
    let t84200 = t23967 * t22531;
    let t84202 = -160.0 / 3.0 * t84174 + 20.0 * t83722 * t23970 + 10.0 * t83778 * t23970 + 20.0 * t22549 * t84180 + 10.0 * t22549 * t84183 - 2.0 * t605 * t84186 * t83820 + 30.0 * t84190 * t22546 + 30.0 * t23963 * t83745 + 80.0 / 3.0 * t84196 + 80.0 / 3.0 * t84198 + 40.0 / 3.0 * t84200;
    (t84202,)
}
