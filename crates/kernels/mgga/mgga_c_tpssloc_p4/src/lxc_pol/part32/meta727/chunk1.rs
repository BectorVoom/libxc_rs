//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2354/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2354<F: Float>(t104729: F, t104976: F, t104977: F, t104990: F, t1458: F, t19534: F, t24932: F, t27863: F, t27888: F, t33690: F, t4072: F, t5493: F, t671: F, t7266: F, t96238: F, t96659: F, t96661: F, t96663: F, t96665: F) -> F {
    let t104995 = F::cast_from(4.0_f64) * t104977 * t1458 + F::cast_from(2.0_f64) * t104990 * t671 + F::cast_from(4.0_f64) * t1458 * t96238 + F::cast_from(2.0_f64) * t19534 * t7266 + F::cast_from(2.0_f64) * t24932 * t5493 + F::cast_from(4.0_f64) * t27863 * t4072 + F::cast_from(2.0_f64) * t27888 * t5493 + F::cast_from(4.0_f64) * t33690 * t4072 + F::cast_from(2.0_f64) * t104729 + t104976 + t96659 + t96661 + t96663 + t96665;
    t104995
}
