//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1432/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1432<F: Float>(t1164: F, t43689: F, t43692: F, t78287: F, t18622: F, t64451: F, t21833: F, t4869: F, t5989: F, t64257: F, t11292: F, t1156: F) -> (F, F, F, F, F) {
    let t78291 = F::cast_from(0.91082604192152556044e5_f64) * t1164 * t43689 * t78287 * t43692;
    let t78294 = F::cast_from(0.61524113149298439947e4_f64) * t1164 * t64451 * t18622;
    let t78296 = F::cast_from(0.14035736694323150897e2_f64) * t4869 * t21833;
    let t78298 = F::new(12.0) * t64257 * t5989;
    let t78302 = F::cast_from(0.14035736694323150897e2_f64) * t1164 * t11292 * t78287 * t1156;
    (t78291, t78294, t78296, t78298, t78302)
}
