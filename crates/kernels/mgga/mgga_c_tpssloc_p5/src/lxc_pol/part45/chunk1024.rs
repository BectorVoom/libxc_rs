//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1024/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1024<F: Float>(t114223: F, t114225: F, t114230: F, t114234: F, t114241: F, t114243: F, t114247: F, t114254: F, t114256: F, t114262: F, t115572: F, t115577: F, t115583: F, t115586: F, t22653: F, t22913: F, t31601: F, t3758: F, t7194: F) -> F {
    let t115590 = t114223 + F::cast_from(0.82246703342411321824e-2_f64) * t115572 + t114225 + t114230 + t114234 - t114241 - t114243 - t114247 + F::cast_from(4.0_f64) * t3758 * t31601 - F::cast_from(0.82246703342411321825e-2_f64) * t115577 + t114254 - t114256 + F::cast_from(4.0_f64) * t7194 * t22653 + F::cast_from(0.16449340668482264365e-1_f64) * t115583 - F::cast_from(0.16449340668482264365e-1_f64) * t115586 + F::cast_from(2.0_f64) * t7194 * t22913 - t114262;
    t115590
}
