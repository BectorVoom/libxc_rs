//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1185/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1185<F: Float>(t1385: F, t1799: F, t22633: F, t22635: F, t31090: F, t114285: F, t26215: F, t225: F, t32727: F, t114226: F, t120213: F, t120218: F, t120221: F, t120226: F, t120229: F, t120232: F, t120239: F, t1386: F, t16022: F, t16439: F, t31217: F, t32758: F, t3882: F, t5215: F, t8476: F, t8486: F) -> (F,) {
    let t120240 = t1799 * t1385;
    let t120244 = 0.6579736267392905746e-1 * t22633 * t22635 * t31090 * t120240;
    let t120247 = 0.3289868133696452873e-1 * t22633 * t114285 * t26215;
    let t120248 = t32727 * t225;
    let t120253 = 0.3289868133696452873e-1 * t22633 * t22635 * t114226 * t1799;
    let t120254 = -t120248 * t1386 + 2.0 * t16022 * t8476 + 2.0 * t16439 * t8476 - t16439 * t8486 - t31217 * t5215 - t32758 * t3882 + t120213 - t120218 - t120221 + t120226 + t120229 - t120232 - t120239 - t120244 + t120247 + t120253;
    (t120254,)
}
