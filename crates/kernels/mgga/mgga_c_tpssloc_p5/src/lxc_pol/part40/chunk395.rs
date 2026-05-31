//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 395/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk395<F: Float>(t25: F, t28: F, t1268: F, t650: F, t671: F, t522: F, t588: F, t592: F, t514: F, t606: F, t1081: F, t517: F, t157: F, zeta_threshold: F) -> (F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t1271 = F::cast_from(2.0_f64) * t1268 * t671 + t650;
    let t1274 = F::cast_from(4.0_f64) * t588 * t522;
    let t1276 = F::cast_from(4.0_f64) * t592 * t522;
    let t1279 = piecewise3::<F>(t26, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t514 * t606);
    let t1282 = piecewise3::<F>(t29, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t517 * t1081);
    let t1284 = (t1279 + t1282) * t157;
    (t1271, t1274, t1276, t1284)
}
