//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1345/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1345<F: Float>(t1165: F, t13133: F, t13554: F, t1799: F, t19305: F, t19656: F, t20319: F, t2056: F, t21180: F, t21227: F, t21907: F, t3493: F, t4347: F, t5815: F, t6234: F, t6323: F, t69026: F, t69069: F, t69072: F, t71159: F) -> F {
    let t71603 = F::new(2.0) * t1165 * t71159 + F::new(4.0) * t13133 * t6323 + F::new(4.0) * t13554 * t6323 + F::new(4.0) * t1799 * t69026 + F::new(2.0) * t1799 * t69069 + F::new(2.0) * t1799 * t69072 + F::new(4.0) * t19305 * t6323 + F::new(4.0) * t19656 * t6323 + F::new(4.0) * t20319 * t3493 + F::new(4.0) * t20319 * t6234 + F::new(2.0) * t2056 * t21907 + F::new(4.0) * t21180 * t5815 + F::new(2.0) * t21227 * t5815 + F::new(2.0) * t21907 * t4347;
    t71603
}
