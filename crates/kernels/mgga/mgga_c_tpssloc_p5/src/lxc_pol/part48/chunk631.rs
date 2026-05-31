//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 631/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk631<F: Float>(t109: F, t577: F, t671: F, t7014: F, t7017: F, t7019: F, t7415: F, t7423: F, t33: F, t68: F, t69: F, t79: F) -> (F, F, F, F, F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t7426 = F::cast_from(0.45e1_f64) * t7415 * t577 + F::cast_from(0.135e2_f64) * t7423 * t671 + t7014 + t7017 + t7019;
    let t8301 = t33 * t33;
    let t8306 = F::cast_from(1.0_f64) / t69 / t68;
    let t8307 = t79 * t79;
    let t8308 = t8306 * t8307;
    let t8326 = piecewise3::<F>(t110, F::cast_from(0.0_f64), F::cast_from(0.0_f64));
    (t7426, t8301, t8306, t8307, t8308, t8326)
}
