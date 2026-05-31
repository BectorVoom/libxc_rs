//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1219/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1219<F: Float>(t85166: F, t870: F, t1877: F, t2057: F, t2058: F, t22961: F, t22968: F, t23296: F, t23299: F, t23302: F, t24191: F, t24335: F, t24339: F, t25: F, t2522: F, t26563: F, t606: F, t7110: F, t7114: F, t81470: F, t81476: F, t81486: F, t81509: F, t81513: F, t81548: F, t82320: F, t82330: F, t84797: F, t84800: F) -> (F, F) {
    let t85167 = t85166 * t870;
    let t85187 = -F::cast_from(9.0_f64) * t84797 * t22961 + F::cast_from(3.0_f64) * t1877 * t84800 * t23296 - F::cast_from(9.0_f64) * t24191 * t81548 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t2057 * t81509 + F::cast_from(3.0_f64) * t82320 * t2058 - F::cast_from(9.0_f64) * t26563 * t81486 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t7114 * t81513 + F::cast_from(9.0_f64) * t26563 * t81470 + t1877 * t85167 * t25 / F::cast_from(2.0_f64) + F::cast_from(9.0_f64) * t24191 * t81476 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t24335 * t606 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2522 * t7110 * t22968 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t24339 * t23302 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t24191 * t82330 - F::cast_from(3.0_f64) * t1877 * t24339 * t23299;
    (t85167, t85187)
}
