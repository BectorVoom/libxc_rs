//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1244/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1244<F: Float>(t193: F, t2378: F, t262: F, t39658: F, t40977: F, t41270: F, t41273: F, t41275: F, t41278: F, t41281: F, t41283: F, t41286: F, t41289: F, t41292: F, t41296: F, t4314: F, t776: F, t868: F, t870: F, t9458: F, t9516: F) -> F {
    let t41603 = F::new(24.0) * t193 * t868 * t870 * t9458 + F::new(24.0) * t262 * t4314 * t776 * t9516 + F::new(18.0) * t193 * t2378 * t40977 - t39658 + t41270 + t41273 + t41275 + t41278 + t41281 + t41283 + t41286 + t41289 + t41292 + t41296;
    t41603
}
