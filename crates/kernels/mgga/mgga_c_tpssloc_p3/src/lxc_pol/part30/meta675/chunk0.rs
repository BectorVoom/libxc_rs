//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2104/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2104<F: Float>(t91548: F, t2022: F, t6483: F, t671: F, t28821: F, t6997: F, t1441: F, t4072: F, t1874: F, t28002: F, t6525: F, t7450: F) -> (F, F, F, F, F, F, F, F) {
    let t93906 = F::cast_from(0.3289868133696452873e-1_f64) * t91548;
    let t96348 = t2022 * t6483;
    let t96351 = t2022 * t671;
    let t96355 = t28821 * t6997;
    let t96356 = t1441 * t4072;
    let t96358 = F::cast_from(4.0_f64) * t96356 * t1874;
    let t96360 = F::cast_from(4.0_f64) * t28002 * t6525;
    let t96361 = t7450 * t671;
    (t93906, t96348, t96351, t96355, t96356, t96358, t96360, t96361)
}
