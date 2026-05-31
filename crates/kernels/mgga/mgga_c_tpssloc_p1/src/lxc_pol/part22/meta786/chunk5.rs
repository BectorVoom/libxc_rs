//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2720/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2720<F: Float>(t1396: F, t1398: F, t1404: F, t1852: F, t1858: F, t20149: F, t20186: F, t22431: F, t22453: F, t3: F, t5364: F, t5381: F, t580: F, t6471: F, t6483: F, t66964: F, t66967: F, t66976: F, t66987: F, t66989: F, t66991: F, t67000: F, t75764: F, t75768: F, t75774: F, t75780: F, t75827: F) -> F {
    let tv4rho43 = t3 * t580 * t75764 + t1396 * t22453 + t1398 * t75827 + t1404 * t22431 + F::cast_from(3.0_f64) * t1852 * t20186 + F::cast_from(3.0_f64) * t1858 * t20149 + F::cast_from(3.0_f64) * t5364 * t6483 + F::cast_from(3.0_f64) * t5381 * t6471 + F::cast_from(6.0_f64) * t66964 + F::cast_from(3.0_f64) * t66967 + F::cast_from(3.0_f64) * t66976 + F::cast_from(3.0_f64) * t66987 + F::cast_from(3.0_f64) * t66989 + F::cast_from(6.0_f64) * t66991 + t67000 + F::cast_from(3.0_f64) * t75768 + F::cast_from(3.0_f64) * t75774 + t75780;
    tv4rho43
}
