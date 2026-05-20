//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2308/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2308<F: Float>(t6260: F, t7327: F, t24660: F, t6252: F, t1215: F, t5392: F, t7376: F, t27736: F, t7999: F, t103218: F, t11904: F, t24849: F, t27406: F, t27455: F, t27525: F, t27532: F, t27733: F, t29678: F, t29719: F, t29723: F, t3610: F, t4930: F, t5068: F, t7283: F, t7365: F, t7382: F, t8077: F, t86037: F, t86039: F, t86076: F, t86077: F, t94837: F, t95048: F) -> (F, F) {
    let t103767 = t7327 * t6260;
    let t103774 = t24660 * t6252;
    let t103779 = t5392 * t1215 * t7376;
    let t103799 = t7999 * t27736;
    let t103801 = -F::cast_from(0.27415567780803773942e-2_f64) * t24849 * t103767 * t27532 - F::cast_from(0.54831135561607547884e-2_f64) * t24849 * t94837 * t27525 - F::cast_from(0.54831135561607547883e-2_f64) * t86037 * t103774 * t86039 + F::cast_from(0.36554090374405031923e-2_f64) * t86076 * t86077 * t103779 + F::new(2.0) * t11904 * t29723 - F::cast_from(0.43864908449286038306e-1_f64) * t7999 * t27733 - t95048 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t4930 * t8077 + F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t27455 - F::cast_from(0.26806332941230356743e-1_f64) * t103218 * t7365 + F::new(4.0) * t3610 * t29719 * t5068 + F::cast_from(0.80418998823691070228e-1_f64) * t29678 * t7382 - F::cast_from(0.14621636149762012769e-1_f64) * t103799;
    (t103779, t103801)
}
