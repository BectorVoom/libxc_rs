//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 941/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk941<F: Float>(t13602: F, t1553: F, t2403: F, t4392: F, t699: F, t13550: F, t13563: F, t1543: F, t2791: F, t2970: F, t4343: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13603 = F::new(2.0) / F::new(9.0) * t13602;
    let t13642 = t2403 * t1553;
    let t13644 = t699 * t4392;
    let t13645 = F::cast_from(0.10954222222222222222e0_f64) * t13644;
    let t13650 = F::cast_from(0.19931111111111111111e0_f64) * t13602;
    let t13675 = F::new(0.22076e0) * t13550;
    let t13679 = F::cast_from(0.13418888888888888889e0_f64) * t13563;
    let t13709 = F::new(0.11038e0) * t13644;
    let t13712 = F::cast_from(0.20128333333333333334e0_f64) * t13602;
    let t13727 = t1543 * t2791;
    let t13748 = t2970 * t4343;
    let t13750 = t973 * t13748 / F::new(216.0);
    (t13603, t13642, t13644, t13645, t13650, t13675, t13679, t13709, t13712, t13727, t13750)
}
