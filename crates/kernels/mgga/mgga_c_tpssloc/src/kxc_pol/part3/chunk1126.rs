//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1126/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1126<F: Float>(t4778: F, t699: F, t1113: F, t14706: F, t136: F, t4725: F, t690: F) -> (F, F, F, F) {
    let t14710 = t699 * t4778;
    let t14711 = F::cast_from(0.11038e0_f64) * t14710;
    let t14712 = t1113 * t14706;
    let t14713 = t136 * t14712;
    let t14720 = t690 * t4725;
    (t14710, t14711, t14713, t14720)
}
