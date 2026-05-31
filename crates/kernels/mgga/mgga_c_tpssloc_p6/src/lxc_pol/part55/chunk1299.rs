//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1299/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1299<F: Float>(t117693: F, t117695: F, t118335: F, t118337: F, t125074: F, t125966: F, t125970: F, t125975: F, t1404: F, t1858: F, t2174: F, t27908: F, t3: F, t32630: F, t34386: F, t5381: F, t580: F, t8920: F) -> F {
    let t125979 = t125966 * t3 * t580 + t1404 * t34386 + t1858 * t32630 + F::cast_from(2.0_f64) * t2174 * t27908 + t5381 * t8920 + t117693 + t117695 + F::cast_from(2.0_f64) * t118335 + F::cast_from(2.0_f64) * t118337 + t125074 + t125970 + F::cast_from(2.0_f64) * t125975;
    t125979
}
