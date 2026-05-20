//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2904/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2904<F: Float>(t60449: F, t60465: F, t60482: F, t60498: F, t60513: F, t60529: F, t60546: F, t60562: F, t893: F, t913: F, t41623: F, t5730: F) -> (F, F) {
    let t60568 = F::new(1.0) * t893 * (t60449 + t60465 + t60482 + t60498 + t60513 + t60529 + t60546 + t60562) * t913;
    let t60570 = F::cast_from(0.16081979498692535067e2_f64) * t41623 * t5730;
    (t60568, t60570)
}
