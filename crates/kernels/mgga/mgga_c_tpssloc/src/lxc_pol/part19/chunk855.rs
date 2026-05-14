//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 855/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk855<F: Float>(t1017: F, t3087: F, t1015: F, t1012: F, t2940: F, t2952: F, t2928: F, t320: F) -> (F, F, F, F) {
    let t10515 = t3087 * t1017;
    let t10516 = t1015 * t10515;
    let t10517 = t1012 * t10516;
    let t10521 = 0.51947577317044391276e2 * t2940 * t2952;
    let t10523 = 1.0 / t2928 / t320;
    (t10516, t10517, t10521, t10523)
}
