//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1251/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1251<F: Float>(t114226: F, t1799: F, t22633: F, t22635: F, t120213: F, t120218: F, t120221: F, t120226: F, t120229: F, t120232: F, t120239: F, t120244: F, t120247: F, t120248: F, t1386: F, t16022: F, t16439: F, t31217: F, t32758: F, t3882: F, t5215: F, t8476: F, t8486: F) -> F {
    let t120253 = F::cast_from(0.3289868133696452873e-1_f64) * t22633 * t22635 * t114226 * t1799;
    let t120254 = -t120248 * t1386 + F::cast_from(2.0_f64) * t16022 * t8476 + F::cast_from(2.0_f64) * t16439 * t8476 - t16439 * t8486 - t31217 * t5215 - t32758 * t3882 + t120213 - t120218 - t120221 + t120226 + t120229 - t120232 - t120239 - t120244 + t120247 + t120253;
    t120254
}
