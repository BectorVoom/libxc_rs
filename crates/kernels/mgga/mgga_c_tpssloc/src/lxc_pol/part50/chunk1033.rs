//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1033/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1033<F: Float>(t23384: F, t30879: F, t1920: F, t2966: F, t8400: F, t30874: F, t6680: F, t362: F, t6768: F, t82632: F, t8381: F, t82573: F, t30858: F, t30800: F, t968: F, t6703: F) -> (F, F, F, F, F, F, F, F, F) {
    let t113528 = t23384 * t30879;
    let t113562 = 0.36554090374405031922e-2 * t1920 * t2966 * t8400;
    let t113576 = t6680 * t30874;
    let t113578 = t362 * t6768;
    let t113600 = 0.36554090374405031922e-2 * t82632 * t8381;
    let t113601 = t82573 * t8381;
    let t113608 = t23384 * t30858;
    let t113611 = t1920 * t968 * t30800;
    let t113619 = t6703 * t6768;
    (t113528, t113562, t113576, t113578, t113600, t113601, t113608, t113611, t113619)
}
