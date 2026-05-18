//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 338/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk338<F: Float>(t25: F, t28: F, t1268: F, t650: F, t671: F, t522: F, t588: F, t592: F, t514: F, t606: F, t1081: F, t517: F, t157: F, zeta_threshold: F) -> (F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t1271 = F::new(2.0) * t1268 * t671 + t650;
    let t1274 = F::new(4.0) * t588 * t522;
    let t1276 = F::new(4.0) * t592 * t522;
    let t1279 = piecewise3::<f64>(t26, F::new(0.0), F::new(4.0) / F::new(3.0) * t514 * t606);
    let t1282 = piecewise3::<f64>(t29, F::new(0.0), F::new(4.0) / F::new(3.0) * t517 * t1081);
    let t1284 = (t1279 + t1282) * t157;
    (t1271, t1274, t1276, t1284)
}
