//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3186/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3186<F: Float>(t11668: F, t11678: F, t11692: F, t15478: F, t15569: F, t15659: F, t18395: F, t18946: F, t19000: F, t3577: F, t3578: F, t45114: F, t45128: F, t4723: F, t52893: F, t52897: F, t52908: F, t52917: F, t52926: F, t52932: F, t53176: F, t65014: F, t65452: F, t66073: F, t66076: F, t66079: F, t66084: F, t66092: F) -> F {
    let t66111 = t66073 / F::cast_from(3456.0_f64) - t66076 / F::cast_from(1728.0_f64) - t66079 / F::cast_from(1728.0_f64) + t52908 / F::cast_from(1152.0_f64) - t52917 / F::cast_from(864.0_f64) - t66084 / F::cast_from(576.0_f64) + t15569 * t15478 / F::cast_from(216.0_f64) - t45114 * t52897 * t15659 * t53176 / F::cast_from(128.0_f64) + t66092 / F::cast_from(576.0_f64) - F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t52893 * t45128 * t65014 + t52926 / F::cast_from(324.0_f64) + t52932 / F::cast_from(54.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t3577 * t11668 * t4723 * t65452 - t11678 * t3578 * t18946 * t19000 / F::cast_from(576.0_f64) + t11692 * t3578 * t53176 * t18395 / F::cast_from(1152.0_f64);
    t66111
}
