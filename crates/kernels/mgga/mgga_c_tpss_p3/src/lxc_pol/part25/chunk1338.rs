//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1338/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1338<F: Float>(t1792: F, t19349: F, t19404: F, t19408: F, t20246: F, t6077: F, t62307: F, t62309: F, t62345: F, t67352: F, t67389: F, t67391: F, t67441: F, t69108: F, t69111: F, t69114: F, t69143: F) -> F {
    let t71431 = -t67389 - t67391 - F::new(440.0) / F::new(27.0) * t62307 - F::new(176.0) / F::new(27.0) * t62309 + F::new(20.0) / F::new(3.0) * t19349 * t67352 - F::new(70.0) * t62345 * t69143 - F::new(10.0) / F::new(3.0) * t67441 * t6077 - F::new(10.0) / F::new(3.0) * t20246 * t19404 - F::new(10.0) / F::new(3.0) * t20246 * t19408 - F::new(4.0) / F::new(3.0) * t69108 * t1792 - F::new(4.0) / F::new(3.0) * t69111 * t1792 - F::new(4.0) / F::new(3.0) * t69114 * t1792;
    t71431
}
