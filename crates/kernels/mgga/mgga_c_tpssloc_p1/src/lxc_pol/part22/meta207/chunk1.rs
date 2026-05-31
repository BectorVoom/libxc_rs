//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1200/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1200<F: Float>(t3383: F, t3390: F, t4721: F, t4770: F, t5973: F, t5977: F, t5981: F, t5993: F, t6000: F, t6006: F, t6008: F, t6012: F, t6015: F, t6018: F) -> F {
    let t6084 = -F::cast_from(0.1294625e1_f64) * t5993 + F::cast_from(0.258925e1_f64) * t6000 + t3383 - F::cast_from(0.20128333333333333334e0_f64) * t4721 - F::cast_from(0.20128333333333333333e0_f64) * t5973 + F::cast_from(0.60385e0_f64) * t5977 + F::cast_from(0.301925e0_f64) * t5981 + F::cast_from(0.82524375e-1_f64) * t6006 + F::cast_from(0.16504875e0_f64) * t6008 + t3390 - F::cast_from(0.11038e0_f64) * t4770 - F::cast_from(0.27595e-1_f64) * t6012 + F::cast_from(0.16557e0_f64) * t6015 + F::cast_from(0.82785e-1_f64) * t6018;
    t6084
}
