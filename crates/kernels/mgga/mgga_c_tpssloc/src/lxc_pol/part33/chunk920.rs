//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 920/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk920<F: Float>(t25: F, t28: F, t19593: F, t1408: F, t6305: F, t12061: F, t20216: F, t5134: F, t514: F, t5397: F, t1649: F, t6312: F, t12072: F, t5142: F, t517: F, t5966: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t20372 = F::new(12.0) * t19593;
    let t20376 = t6305 * t1408;
    let t20384 = piecewise3::<F>(t26, F::new(0.0), -F::new(8.0) / F::new(27.0) * t12061 * t20376 + F::new(4.0) / F::new(3.0) * t5134 * t5397 + F::new(4.0) / F::new(3.0) * t514 * t20216);
    let t20385 = t6312 * t1649;
    let t20390 = -t20216;
    let t20394 = piecewise3::<F>(t29, F::new(0.0), -F::new(8.0) / F::new(27.0) * t12072 * t20385 + F::new(4.0) / F::new(3.0) * t5142 * t5966 + F::new(4.0) / F::new(3.0) * t517 * t20390);
    (t20372, t20376, t20384, t20385, t20390, t20394)
}
