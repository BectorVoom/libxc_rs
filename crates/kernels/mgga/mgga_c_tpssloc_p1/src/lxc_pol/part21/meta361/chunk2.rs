//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1783/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1783<F: Float>(t13381: F, t13385: F, t13388: F, t13390: F, t13393: F, t13397: F, t13398: F, t13401: F, t13404: F, t13407: F, t13414: F, t13417: F, t13423: F, t2617: F, t2729: F, t2733: F, t2736: F, t4166: F, t4281: F, t4291: F, t4292: F, t4296: F, t812: F) -> F {
    let t13425 = -F::cast_from(2.0_f64) * t13381 * t4291 + F::cast_from(4.0_f64) * t13385 * t4281 - t13388 * t4291 - F::cast_from(2.0_f64) * t13390 * t4292 + F::cast_from(4.0_f64) * t13393 * t4281 - F::cast_from(6.0_f64) * t13397 * t13398 + F::cast_from(6.0_f64) * t13401 * t4281 + F::cast_from(2.0_f64) * t13404 * t4281 - F::cast_from(2.0_f64) * t13407 * t812 - t13414 * t812 + F::cast_from(2.0_f64) * t13417 * t812 - t13423 * t812 - F::cast_from(2.0_f64) * t2617 * t4296 + F::cast_from(2.0_f64) * t2729 * t4166 - F::cast_from(2.0_f64) * t2733 * t4166 - t2736 * t4166;
    t13425
}
