//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1348/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1348<F: Float>(t3255: F, t6419: F, t5380: F, t5918: F, t62375: F, t67138: F, t69489: F, t69491: F, t69493: F, t69495: F, t69497: F, t69499: F, t69501: F, t69503: F, t69505: F, t69507: F) -> (F, F, F) {
    let t71725 = t3255 * t6419;
    let t71748 = t5918 * t5380;
    let t71776 = t67138 + t69489 / F::cast_from(96.0_f64) - F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t69491 - t62375 - t69493 / F::cast_from(48.0_f64) - t69495 / F::cast_from(128.0_f64) + t69497 / F::cast_from(128.0_f64) + t69499 / F::cast_from(192.0_f64) - t69501 / F::cast_from(768.0_f64) - t69503 / F::cast_from(96.0_f64) - t69505 / F::cast_from(768.0_f64) - F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t69507;
    (t71725, t71748, t71776)
}
