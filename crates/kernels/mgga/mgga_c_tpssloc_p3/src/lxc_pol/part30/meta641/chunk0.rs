//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2051/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2051<F: Float>(t14137: F, t6765: F, t7583: F, t83138: F, t25644: F, t82926: F, t23512: F, t25486: F, t23519: F, t25492: F, t1597: F, t607: F) -> (F, F, F, F, F, F) {
    let t88339 = F::new(5.0) / F::new(5184.0) * t6765 * t14137;
    let t88341 = F::cast_from(0.20186378047070195428e-3_f64) * t83138 * t7583;
    let t88348 = t82926 * t25644;
    let t88351 = t23512 * t25486;
    let t88354 = t23519 * t25492;
    let t88360 = t607 * t1597;
    (t88339, t88341, t88348, t88351, t88354, t88360)
}
