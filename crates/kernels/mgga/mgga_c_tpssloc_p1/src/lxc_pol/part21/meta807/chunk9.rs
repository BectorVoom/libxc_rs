//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2819/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2819<F: Float>(t16625: F, t2379: F, t2522: F, t4314: F, t47645: F, t5502: F, t5544: F, t59014: F, t59015: F, t59016: F, t59018: F, t59019: F, t59020: F, t59023: F, t59025: F, t59027: F, t9470: F) -> F {
    let t59602 = -F::new(6.0) * t16625 * t2379 * t4314 - F::new(3.0) * t2522 * t5544 * t9470 + F::new(12.0) * t47645 * t5502 + t59014 + t59015 + t59016 + t59018 + t59019 + t59020 + t59023 + t59025 + t59027;
    t59602
}
