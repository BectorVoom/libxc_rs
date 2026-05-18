//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 912/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk912<F: Float>(t1351: F, t1824: F, t3792: F, t225: F, t5319: F, t5217: F, t112: F, t5363: F, t111: F, t1851: F, t1484: F, t868: F) -> (F, F, F, F, F, F, F) {
    let t16306 = t1824 * t1351;
    let t16311 = t1824 * t3792;
    let t16439 = t5319 * t225;
    let t16460 = t5217 * t225;
    let t16521 = t5363 * t112;
    let t16524 = t1851 * t111;
    let t16596 = t1484 * t868;
    (t16306, t16311, t16439, t16460, t16521, t16524, t16596)
}
