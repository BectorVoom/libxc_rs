//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1301/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1301<F: Float>(t110899: F, t110904: F, t110910: F, t111601: F, t111602: F, t111604: F, t111641: F, t111683: F, t1396: F, t1398: F, t1858: F, t20149: F, t2193: F, t30218: F, t30500: F, t5364: F, t5381: F, t6471: F, t8171: F, t8241: F, t8256: F) -> F {
    let t111692 = F::cast_from(2.0_f64) * t5364 * t8256 + t1396 * t30500 + t111601 + t110899 + F::cast_from(2.0_f64) * t111602 + t111604 + t110904 + t1398 * (t111641 + t111683) + F::cast_from(2.0_f64) * t30218 * t1858 + t6471 * t8171 + t20149 * t2193 + F::cast_from(2.0_f64) * t8241 * t5381 + t110910;
    t111692
}
