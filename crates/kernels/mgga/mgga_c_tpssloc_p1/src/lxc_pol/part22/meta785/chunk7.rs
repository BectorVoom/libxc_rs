//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2710/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2710<F: Float>(t112: F, t75554: F, t5449: F, t671: F, t20305: F, t626: F, t20308: F, t1453: F, t5488: F, t20343: F, t1444: F, t5396: F) -> (F, F, F, F, F, F, F) {
    let t75555 = t75554 * t112;
    let t75560 = t5449 * t671;
    let t75592 = t626 * t20305;
    let t75601 = t626 * t20308;
    let t75603 = t1453 * t5488;
    let t75613 = t626 * t20343;
    let t75631 = t1444 * t5396;
    (t75555, t75560, t75592, t75601, t75603, t75613, t75631)
}
