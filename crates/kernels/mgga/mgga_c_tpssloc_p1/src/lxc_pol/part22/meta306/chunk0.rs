//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1477/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1477<F: Float>(t14704: F, t4778: F, t699: F, t4725: F, t690: F) -> (F, F, F, F) {
    let t14705 = F::cast_from(0.20128333333333333334e0_f64) * t14704;
    let t14710 = t699 * t4778;
    let t14711 = F::new(0.11038e0) * t14710;
    let t14720 = t690 * t4725;
    (t14705, t14710, t14711, t14720)
}
