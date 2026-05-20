//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2229/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2229<F: Float>(t4509: F, t5842: F, t17686: F, t42841: F, t17783: F, t2960: F, t13779: F, t17167: F, t2986: F, t17171: F, t13784: F, t17157: F) -> (F, F, F, F, F, F) {
    let t61365 = t4509 * t5842;
    let t61375 = t42841 * t17686;
    let t61383 = t2960 * t17783;
    let t61387 = t2986 * t13779 * t17167;
    let t61391 = t2986 * t13779 * t17171;
    let t61394 = t2986 * t13784 * t17157;
    (t61365, t61375, t61383, t61387, t61391, t61394)
}
