//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 758/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk758<F: Float>(t368: F, t383: F, t353: F, t1935: F, t378: F, t8384: F) -> (F, F, F) {
    let t8387 = t383 * t368;
    let t8388 = t353 * t8387;
    let t8391 = 0.40372756094140390856e-3 * t1935 * t8384 + t8388 * t378 / 1536.0;
    (t8387, t8388, t8391)
}
