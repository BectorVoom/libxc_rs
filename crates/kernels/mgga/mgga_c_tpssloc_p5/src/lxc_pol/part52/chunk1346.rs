//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1346/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1346<F: Float>(t32762: F, t6883: F, t1985: F, t214: F, t225: F, t26328: F, t567: F, t7722: F, t6907: F, t120334: F, t120337: F, t120340: F, t120425: F, t120436: F, t120488: F, t120528: F, t1375: F, t1378: F, t1807: F, t22656: F, t26366: F, t31117: F, t31181: F, t31217: F, t32766: F, t3882: F, t5321: F, t539: F, t568: F, t6963: F, t7750: F) -> (F, F) {
    let t120532 = t6883 * t32762;
    let t120533 = F::cast_from(0.38381794893125283518e-1_f64) * t120532;
    let t120542 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t214 * t26328 * t225 * t567;
    let t120544 = t214 * t7722;
    let t120547 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t120544 * t6907;
    let t120548 = t120334 - t120337 - t120340 + t539 * t120425 * t568 + F::new(4.0) * t26366 * t6963 + F::new(4.0) * t3882 * t32766 - F::new(6.0) * t5321 * t31117 - t120436 - t1375 * t1378 * (t120488 + t120528) - t120533 + t1807 * t31181 * t568 - F::new(2.0) * t22656 * t7750 + t120542 - t5321 * t31217 - t120547;
    (t120544, t120548)
}
