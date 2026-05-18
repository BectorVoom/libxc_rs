//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 814/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk814<F: Float>(t1156: F, t6068: F, t3383: F, t3390: F, t4721: F, t4770: F, t5973: F, t5977: F, t5981: F, t5993: F, t6000: F, t6006: F, t6008: F, t6012: F, t6015: F, t6018: F) -> (F, F) {
    let t6069 = t6068 * t1156;
    let t6084 = -F::new(0.1294625e1) * t5993 + F::new(0.258925e1) * t6000 + t3383 - F::new(0.20128333333333333334e0) * t4721 - F::new(0.20128333333333333333e0) * t5973 + F::new(0.60385e0) * t5977 + F::new(0.301925e0) * t5981 + F::new(0.82524375e-1) * t6006 + F::new(0.16504875e0) * t6008 + t3390 - F::new(0.11038e0) * t4770 - F::new(0.27595e-1) * t6012 + F::new(0.16557e0) * t6015 + F::new(0.82785e-1) * t6018;
    (t6069, t6084)
}
