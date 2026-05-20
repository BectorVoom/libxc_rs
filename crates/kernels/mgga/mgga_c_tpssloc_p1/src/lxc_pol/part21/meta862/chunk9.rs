//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3138/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3138<F: Float>(t51039: F, t51041: F, t51043: F, t51051: F, t51053: F, t64074: F, t64076: F, t64079: F, t64082: F, t64085: F, t64087: F, t64089: F, t64092: F) -> F {
    let t64943 = -F::new(40.0) / F::new(27.0) * t51039 + F::new(4.0) / F::new(9.0) * t51041 + F::new(4.0) / F::new(3.0) * t51043 + F::new(20.0) / F::new(81.0) * t51051 + F::new(8.0) / F::new(9.0) * t51053 - F::new(4.0) / F::new(27.0) * t64074 - F::new(4.0) / F::new(9.0) * t64076 + t64079 / F::new(9.0) + t64082 / F::new(3.0) + F::new(2.0) * t64085 + F::new(8.0) / F::new(9.0) * t64087 + F::new(4.0) / F::new(3.0) * t64089 - F::new(2.0) / F::new(3.0) * t64092;
    t64943
}
