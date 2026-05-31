//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1526/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1526<F: Float>(t1398: F, t1852: F, t1858: F, t22431: F, t22453: F, t3: F, t580: F, t6471: F, t6483: F, t67000: F, t75768: F, t75774: F, t75780: F, t80559: F, t80591: F) -> F {
    let tv4rho44 = t3 * t580 * t80559 + t1398 * t80591 + F::cast_from(4.0_f64) * t1852 * t22453 + F::cast_from(4.0_f64) * t1858 * t22431 + F::cast_from(6.0_f64) * t6471 * t6483 + F::cast_from(4.0_f64) * t67000 + F::cast_from(12.0_f64) * t75768 + F::cast_from(12.0_f64) * t75774 + F::cast_from(4.0_f64) * t75780;
    tv4rho44
}
