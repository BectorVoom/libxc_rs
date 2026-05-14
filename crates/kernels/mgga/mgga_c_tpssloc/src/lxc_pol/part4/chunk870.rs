//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 870/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk870<F: Float>(t4734: F, t690: F, t4778: F, t699: F, t4725: F) -> (F, F, F, F, F) {
    let t14704 = t690 * t4734;
    let t14705 = 0.20128333333333333334e0 * t14704;
    let t14710 = t699 * t4778;
    let t14711 = 0.11038e0 * t14710;
    let t14720 = t690 * t4725;
    (t14704, t14705, t14710, t14711, t14720)
}
