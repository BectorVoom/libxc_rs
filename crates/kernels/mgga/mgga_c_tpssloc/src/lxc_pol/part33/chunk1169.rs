//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1169/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1169<F: Float>(t5914: F, t6703: F, t23384: F, t28492: F, t28500: F, t28648: F, t82431: F, t28667: F, t82736: F, t23665: F, t28626: F, t1539: F, t7582: F, t82655: F, t28622: F, t225: F, t28557: F) -> (F, F, F, F, F, F, F, F, F) {
    let t99895 = t6703 * t5914;
    let t99948 = t23384 * t28492;
    let t99956 = t23384 * t28500;
    let t99960 = t82431 * t28648;
    let t99966 = t82736 * t28667;
    let t99977 = t23665 * t28626;
    let t100008 = t82655 * t1539 * t7582;
    let t100019 = t23665 * t28622;
    let t100126 = t28557 * t225;
    (t99895, t99948, t99956, t99960, t99966, t99977, t100008, t100019, t100126)
}
