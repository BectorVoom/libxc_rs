//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 936/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk936<F: Float>(t25: F, t28: F, t11988: F, t12061: F, t12064: F, t2249: F, t514: F, t9257: F, t528: F, t1081: F, t3672: F, t11122: F, t12001: F, t3231: F, t517: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t12070 = piecewise3::<F>(t26, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t12061 * t11988 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t12064 * t2249 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t514 * t9257);
    let t12072 = F::cast_from(1.0_f64) / t528 / t28;
    let t12075 = t3672 * t1081;
    let t12081 = piecewise3::<F>(t29, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t12072 * t12001 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t12075 * t3231 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t517 * t11122);
    (t12070, t12081)
}
