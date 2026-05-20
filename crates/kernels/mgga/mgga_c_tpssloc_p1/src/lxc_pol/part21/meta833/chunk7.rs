//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2948/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2948<F: Float>(t13779: F, t17171: F, t2986: F, t13784: F, t17157: F, t10190: F, t17817: F, t17769: F, t2960: F, t10224: F, t5824: F, t973: F) -> (F, F, F, F, F) {
    let t61391 = t2986 * t13779 * t17171;
    let t61394 = t2986 * t13784 * t17157;
    let t61397 = t2986 * t10190 * t17817;
    let t61405 = t2960 * t17769;
    let t61408 = t973 * t10224 * t5824;
    (t61391, t61394, t61397, t61405, t61408)
}
