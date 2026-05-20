//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1149/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1149<F: Float>(t30874: F, t6680: F, t362: F, t6768: F, t82632: F, t8381: F, t82573: F, t23384: F, t30858: F, t1920: F, t30800: F, t968: F) -> (F, F, F, F, F, F) {
    let t113576 = t6680 * t30874;
    let t113578 = t362 * t6768;
    let t113600 = F::cast_from(0.36554090374405031922e-2_f64) * t82632 * t8381;
    let t113601 = t82573 * t8381;
    let t113608 = t23384 * t30858;
    let t113611 = t1920 * t968 * t30800;
    (t113576, t113578, t113600, t113601, t113608, t113611)
}
