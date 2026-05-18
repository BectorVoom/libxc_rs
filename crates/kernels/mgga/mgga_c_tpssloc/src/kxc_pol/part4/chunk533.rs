//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 533/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk533<F: Float>(t2535: F, t761: F, t718: F, t751: F, t15: F, t60: F, t59: F) -> (F, F, F) {
    let t2537 = F::new(0.5848223622634646207e0) * t761 * t2535;
    let t2538 = t718 * t751;
    let t2558 = F::new(1.0) / t60 / t15;
    let t2559 = t59 * t2558;
    (t2537, t2538, t2559)
}
