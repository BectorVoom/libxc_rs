//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1507/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1507<F: Float>(t39563: F, t39570: F, t39582: F, t39585: F, t39590: F, t39593: F, t39595: F, t79925: F, t79927: F, t79928: F, t79929: F, t79930: F, t79934: F) -> F {
    let t80109 = t39563 + t79925 + t39570 + t79927 + t79928 + t79929 + t79930 - t39582 - t39585 + t39590 - t39593 + t39595 + t79934;
    t80109
}
