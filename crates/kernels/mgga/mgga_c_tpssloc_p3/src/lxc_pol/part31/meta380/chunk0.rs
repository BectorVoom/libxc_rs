//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1345/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1345<F: Float>(t10595: F, t5698: F, t896: F, t4362: F, t4370: F, t2798: F, t5705: F, t10599: F, t4378: F, t2815: F, t10296: F, t10542: F, t10545: F, t10556: F, t13552: F, t13566: F, t13675: F, t13679: F, t17173: F, t17180: F, t17185: F) -> (F, F, F, F, F, F, F) {
    let t17210 = t10595 * t5698;
    let t17211 = t17210 * t896;
    let t17213 = t4362 * t4370;
    let t17215 = t2798 * t5705;
    let t17216 = t17215 * t896;
    let t17218 = t10599 * t5698;
    let t17219 = t17218 * t896;
    let t17221 = t4378 * t4370;
    let t17223 = t2815 * t5705;
    let t17224 = t17223 * t896;
    let t17238 = F::new(0.12077e1) * t17173 - t13675 + F::cast_from(0.36793333333333333333e-1_f64) * t13552 + t13679 - F::cast_from(0.40256666666666666668e0_f64) * t13566 - F::cast_from(0.91983333333333333333e-1_f64) * t10296 - t10542 - t10545 - F::cast_from(0.20128333333333333333e0_f64) * t17180 + F::new(0.60385e0) * t17185 - F::cast_from(0.13418888888888888889e0_f64) * t10556;
    (t17211, t17213, t17216, t17219, t17221, t17224, t17238)
}
