//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2128/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2128<F: Float>(t26135: F, t7676: F, t2314: F, t28017: F, t5113: F, t1873: F, t96356: F, t28002: F, t6534: F, t12725: F, t7467: F, t75560: F) -> (F, F, F, F, F, F, F) {
    let t96667 = F::new(4.0) * t7676 * t26135;
    let t96669 = F::new(2.0) * t2314 * t28017;
    let t96671 = F::new(2.0) * t5113 * t28017;
    let t96673 = F::new(4.0) * t96356 * t1873;
    let t96675 = F::new(4.0) * t28002 * t6534;
    let t96677 = F::new(4.0) * t12725 * t7467;
    let t96679 = F::new(2.0) * t75560 * t1873;
    (t96667, t96669, t96671, t96673, t96675, t96677, t96679)
}
