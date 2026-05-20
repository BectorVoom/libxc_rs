//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2405/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2405<F: Float>(t41684: F, t41863: F, t68460: F, t68464: F, t68468: F, t68472: F, t68479: F, t68483: F, t68486: F, t68489: F, t68492: F, t68494: F) -> F {
    let t68798 = F::cast_from(0.49293999999999999999e0_f64) * t68460 + F::cast_from(0.49293999999999999999e0_f64) * t68464 - F::cast_from(0.82156666666666666668e-1_f64) * t68468 - F::cast_from(0.82156666666666666668e-1_f64) * t68472 + F::cast_from(0.31003950617283950618e0_f64) * t41684 + F::cast_from(0.2434271604938271605e0_f64) * t41863 - F::cast_from(0.88582716049382716048e0_f64) * t68479 - F::cast_from(0.71752000000000000002e1_f64) * t68483 + F::cast_from(0.35876000000000000001e1_f64) * t68486 - F::cast_from(0.59793333333333333333e0_f64) * t68489 - F::cast_from(0.59793333333333333333e0_f64) * t68492 + F::cast_from(0.19931111111111111111e0_f64) * t68494;
    t68798
}
