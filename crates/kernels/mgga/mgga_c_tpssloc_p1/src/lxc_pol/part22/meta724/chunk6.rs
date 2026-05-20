//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2376/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2376<F: Float>(t41684: F, t41863: F, t68460: F, t68464: F, t68468: F, t68472: F, t68479: F, t68483: F, t68486: F, t68489: F, t68492: F, t68494: F) -> F {
    let t68496 = F::new(0.49671e0) * t68460 + F::new(0.49671e0) * t68464 - F::new(0.82785e-1) * t68468 - F::new(0.82785e-1) * t68472 + F::cast_from(0.31310740740740740741e0_f64) * t41684 + F::cast_from(0.24528888888888888889e0_f64) * t41863 - F::cast_from(0.89459259259259259259e0_f64) * t68479 - F::new(0.72462e1) * t68483 + F::new(0.36231e1) * t68486 - F::cast_from(0.60384999999999999999e0_f64) * t68489 - F::cast_from(0.60384999999999999999e0_f64) * t68492 + F::cast_from(0.20128333333333333333e0_f64) * t68494;
    t68496
}
