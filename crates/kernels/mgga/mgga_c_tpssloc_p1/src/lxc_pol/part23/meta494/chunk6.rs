//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1525/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1525<F: Float>(t79729: F, t80558: F, t1401: F, t1458: F, t16524: F, t1851: F, t20162: F, t20347: F, t22445: F, t22448: F, t28893: F, t3941: F, t5371: F, t5456: F, t5493: F, t55388: F, t577: F, t75784: F, t79817: F, t79825: F) -> (F, F) {
    let t80559 = t79729 + t80558;
    let t80591 = F::cast_from(0.45e1_f64) * t80559 * t577 + F::cast_from(54.0_f64) * t75784 * t1458 + F::cast_from(162.0_f64) * t55388 * t5456 + F::cast_from(81.0_f64) * t20162 * t5493 + F::cast_from(108.0_f64) * t1851 * t22445 + F::cast_from(324.0_f64) * t16524 * t22448 + F::cast_from(54.0_f64) * t5371 * t20347 + F::cast_from(162.0_f64) * t28893 * t5493 + F::cast_from(81.0_f64) * t3941 * t79825 + F::cast_from(108.0_f64) * t3941 * t1458 * t20347 + F::cast_from(0.135e2_f64) * t1401 * t79817;
    (t80559, t80591)
}
