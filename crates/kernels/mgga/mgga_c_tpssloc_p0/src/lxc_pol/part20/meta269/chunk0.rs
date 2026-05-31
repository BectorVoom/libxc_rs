//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1426/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1426<F: Float>(t2393: F, t374: F, t376: F, t370: F, t10250: F, t977: F, t3158: F, t964: F, t10335: F, t221: F, t339: F, t2955: F, t995: F) -> (F, F, F, F, F, F, F) {
    let t10375 = t374 * t2393 * t376;
    let t10377 = t370 * t10375 / F::cast_from(10368.0_f64);
    let t10378 = t977 * t10250;
    let t10381 = t964 * t3158;
    let t10383 = t221 * t10335;
    let t10385 = F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t339 * t10383;
    let t10388 = t2955 * t995;
    (t10375, t10377, t10378, t10381, t10383, t10385, t10388)
}
