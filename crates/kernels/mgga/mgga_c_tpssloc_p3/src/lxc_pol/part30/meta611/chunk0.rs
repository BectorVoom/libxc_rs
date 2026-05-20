//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2006/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2006<F: Float>(t23511: F, t6733: F, t1049: F, t6743: F, t883: F, t221: F, t697: F, t1926: F) -> (F, F, F) {
    let t82620 = t6733 * t23511;
    let t82625 = t6743 * t1049 * t883;
    let t82631 = t221 * t697;
    let t82632 = t1926 * t82631;
    (t82620, t82625, t82632)
}
