//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2439/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2439<F: Float>(t3147: F, t698: F, t973: F, t10632: F, t2924: F, t10510: F, t3114: F, t10508: F, t248: F, t3039: F, t3041: F, t3020: F, t3030: F) -> (F, F, F, F, F) {
    let t42613 = t973 * t698 * t3147;
    let t42671 = t10632 * t2924;
    let t42721 = t3114 * t10510;
    let t42735 = t3039 * t248 * t10508 * t3041;
    let t42741 = t3020 * t3030;
    (t42613, t42671, t42721, t42735, t42741)
}
