//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1380/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1380<F: Float>(t5: F, t67342: F, t67362: F, t67387: F, t67407: F, t67434: F, t67462: F, t67489: F, t67514: F, t117: F, t65440: F, t65442: F, t65444: F) -> (F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t67518 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t67342 + t67362 + t67387 + t67407 + t67434 + t67462 + t67489 + t67514);
    let t67519 = t67518 * t117;
    let t67531 = F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t65440;
    let t67532 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t65442;
    let t67533 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t65444;
    (t67519, t67531, t67532, t67533)
}
