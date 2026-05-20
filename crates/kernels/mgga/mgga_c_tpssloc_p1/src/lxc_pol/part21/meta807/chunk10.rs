//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2820/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2820<F: Float>(t4303: F, t16625: F, t193: F, t202: F, t2522: F, t2553: F, t2752: F, t4314: F, t5527: F, t59029: F, t59031: F, t59033: F, t59034: F, t59035: F, t59038: F, t59040: F, t59043: F, t59046: F, t59049: F, t9470: F) -> F {
    let t59609 = t4303 * t4303;
    let t59614 = -F::new(2.0) * t193 * t202 * t2752 * t59609 - F::new(3.0) * t16625 * t2522 * t2553 - F::new(6.0) * t4314 * t5527 * t9470 - t59029 + t59031 + t59033 + t59034 + t59035 + t59038 + t59040 + t59043 - t59046 - t59049;
    t59614
}
