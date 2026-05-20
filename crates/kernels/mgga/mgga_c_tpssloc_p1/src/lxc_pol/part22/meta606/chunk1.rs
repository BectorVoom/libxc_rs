//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2132/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2132<F: Float>(t3039: F, t4599: F, t49850: F, t10870: F, t4644: F, t10875: F, t48569: F, t10903: F, t14507: F, t14651: F, t3069: F, t4608: F, t698: F, t973: F) -> (F, F, F, F, F, F) {
    let t50258 = t3039 * t49850 * t4599;
    let t50259 = t50258 / F::new(4608.0);
    let t50262 = t4644 * t10870;
    let t50263 = t50262 / F::new(6912.0);
    let t50265 = t48569 * t10875;
    let t50302 = t14507 * t10903;
    let t50324 = t14651 * t3069;
    let t50361 = t973 * t698 * t4608;
    (t50259, t50263, t50265, t50302, t50324, t50361)
}
