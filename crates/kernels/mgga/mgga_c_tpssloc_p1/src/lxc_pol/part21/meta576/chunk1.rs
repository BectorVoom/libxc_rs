//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2296/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2296<F: Float>(t19529: F, t656: F, t12747: F, t12750: F, t12752: F, t19471: F, t19474: F, t19477: F, t19480: F, t19483: F, t64: F, t9358: F, t9359: F) -> (F, F) {
    let t19530 = t656 * t19529;
    let t19533 = -t9358 - F::new(11.0) / F::new(9.0) * t9359 - F::new(22.0) / F::new(9.0) * t12747 - t12750 + t12752 - F::new(2.0) / F::new(3.0) * t19471 - F::new(3.0) / F::new(4.0) * t64 * t19474 + t64 * t19477 / F::new(2.0) + t19480 / F::new(3.0) + t64 * t19483 / F::new(4.0) - t64 * t19530 / F::new(8.0);
    (t19530, t19533)
}
