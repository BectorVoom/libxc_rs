//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2217/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2217<F: Float>(t10110: F, t17056: F, t25168: F, t25169: F, t25233: F, t25330: F, t259: F, t2713: F, t28317: F, t4142: F, t4147: F, t4268: F, t5636: F, t6662: F, t7510: F, t82120: F, t82123: F, t855: F, t92458: F, t98291: F, t98305: F) -> F {
    let t98309 = -t92458 + F::cast_from(0.16449340668482264365e-1_f64) * t82120 - t82123 - F::new(6.0) * t25168 * t25169 * t17056 + F::cast_from(0.9869604401089358619e-1_f64) * t98291 + F::new(2.0) * t2713 * t28317 - F::new(6.0) * t855 * t10110 * t6662 * t5636 + F::new(2.0) * t4142 * t7510 * t259 + F::new(4.0) * t4147 * t25233 - F::cast_from(0.16449340668482264365e-1_f64) * t98305 - F::new(2.0) * t4268 * t25330;
    t98309
}
