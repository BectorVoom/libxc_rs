//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2721/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2721<F: Float>(t1345: F, t1347: F, t1348: F, t16148: F, t16176: F, t16186: F, t16191: F, t16202: F, t1819: F, t1821: F, t19702: F, t19725: F, t19728: F, t1995: F, t3734: F, t3839: F, t3843: F, t3847: F, t5272: F, t5278: F, t5283: F, t546: F, t56275: F, t56486: F, t6347: F, t6404: F, t6408: F, t6411: F) -> F {
    let t57298 = F::new(60.0) * t1995 * t3734 * t5278 * t6347 + F::new(3.0) * t1347 * t546 * t56275 + F::new(240.0) * t16148 * t16191 * t5278 - F::new(24.0) * t3843 * t546 * t56486 + F::new(6.0) * t1345 * t19728 + F::new(6.0) * t1348 * t19702 + F::new(6.0) * t16176 * t1821 - F::new(24.0) * t16186 * t19725 + F::new(6.0) * t16202 * t1819 - F::new(12.0) * t3839 * t6408 + F::new(3.0) * t3839 * t6411 + F::new(3.0) * t3847 * t6404 + F::new(12.0) * t5272 * t5283;
    t57298
}
