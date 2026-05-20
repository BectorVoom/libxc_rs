//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2673/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2673<F: Float>(t1307: F, t1345: F, t1347: F, t1365: F, t16186: F, t16191: F, t16195: F, t1819: F, t19631: F, t19715: F, t19728: F, t19994: F, t20356: F, t20416: F, t20544: F, t20547: F, t20550: F, t5187: F, t5278: F, t5279: F, t546: F, t6347: F, t6924: F, t74355: F) -> F {
    let t74562 = -F::new(12.0) * t1307 * t1365 * t20416 * t5278 - F::new(360.0) * t1307 * t20356 * t5278 * t6924 + F::new(3.0) * t1347 * t546 * t74355 + F::new(180.0) * t16191 * t19994 * t5278 - F::new(36.0) * t16195 * t5278 * t6347 - F::new(36.0) * t19631 * t5278 * t5279 + F::new(180.0) * t19715 * t5187 * t5278 + F::new(60.0) * t1345 * t20544 + F::new(3.0) * t1345 * t20550 - F::new(36.0) * t16186 * t20547 + F::new(9.0) * t1819 * t19728;
    t74562
}
