//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 994/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk994<F: Float>(t121506: F, t1484: F, t6552: F, t6637: F, t114655: F, t121501: F, t126433: F, t126437: F, t126441: F, t127959: F, t127963: F, t31394: F, t33388: F, t4166: F, t5585: F, t5612: F, t5617: F, t812: F) -> F {
    let t127967 = t6552 * t6637 * t121506 * t1484;
    let t127979 = t126433 - t126437 + t126441 + F::cast_from(0.49348022005446793095e-1_f64) * t127959 + F::cast_from(0.3289868133696452873e-1_f64) * t127963 - F::cast_from(0.3289868133696452873e-1_f64) * t127967 + F::cast_from(0.16449340668482264365e-1_f64) * t121501 - t812 * t31394 * t5612 + F::new(2.0) * t812 * t114655 * t5585 - t812 * t31394 * t5617 - F::new(2.0) * t4166 * t33388;
    t127979
}
