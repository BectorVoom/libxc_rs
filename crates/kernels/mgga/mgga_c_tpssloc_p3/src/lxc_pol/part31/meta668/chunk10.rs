//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1975/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1975<F: Float>(t225: F, t29099: F, t13463: F, t17057: F, t17063: F, t17092: F, t25168: F, t26582: F, t4268: F, t7087: F, t7107: F, t7830: F, t87042: F, t87050: F, t92394: F, t92486: F, t98315: F, t98319: F, t98322: F) -> (F, F) {
    let t101509 = t29099 * t225;
    let t101540 = F::cast_from(24.0_f64) * t25168 * t92394 * t17063 + F::cast_from(2.0_f64) * t7087 * t17057 - F::cast_from(0.3289868133696452873e-1_f64) * t98315 - F::cast_from(0.3289868133696452873e-1_f64) * t98319 + F::cast_from(0.16449340668482264365e-1_f64) * t98322 - F::cast_from(2.0_f64) * t17092 * t7107 + t92486 - t87042 + F::cast_from(4.0_f64) * t4268 * t26582 + F::cast_from(4.0_f64) * t13463 * t7830 - F::cast_from(0.46058153871750340221e0_f64) * t87050;
    (t101509, t101540)
}
