//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1449/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1449<F: Float>(t11692: F, t1748: F, t18395: F, t19047: F, t22208: F, t22246: F, t22258: F, t22314: F, t3578: F, t5005: F, t5019: F, t5024: F, t53083: F, t6221: F, t65528: F, t72223: F, t72225: F, t72229: F, t72248: F, t72251: F, t72253: F, t72384: F, t72767: F) -> F {
    let t78713 = F::cast_from(19.0_f64) / F::cast_from(216.0_f64) * t72223 - t5019 * t22246 / F::cast_from(144.0_f64) + F::cast_from(5.0_f64) / F::cast_from(1728.0_f64) * t72225 + t72229 / F::cast_from(192.0_f64) - F::cast_from(19.0_f64) / F::cast_from(216.0_f64) * t72384 * t1748 + t19047 * t6221 / F::cast_from(512.0_f64) + F::cast_from(5.0_f64) / F::cast_from(243.0_f64) * t5024 * t22208 + t11692 * t3578 * t72767 * t18395 / F::cast_from(384.0_f64) - t72248 / F::cast_from(384.0_f64) - t65528 / F::cast_from(2304.0_f64) + t72251 / F::cast_from(54.0_f64) + t72253 / F::cast_from(54.0_f64) + t53083 * t22314 / F::cast_from(24.0_f64) - t5005 * t22258 / F::cast_from(192.0_f64);
    t78713
}
