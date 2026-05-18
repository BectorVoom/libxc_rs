//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 632/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk632<F: Float>(t25: F, t265: F, t394: F, t5669: F, t5954: F, t1408: F, t1409: F, t1534: F, t1642: F, t396: F, t40: F, t5397: F, t5398: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t5955 = piecewise3::<f64>(t395, t5954, t5669);
    let t5962 = piecewise3::<f64>(t115, t5669 * t25 / F::new(2.0) + t1534 * t1408 + t265 * t5397 / F::new(2.0), t5955 * t40 / F::new(2.0) + t1642 * t1409 + t396 * t5398 / F::new(2.0));
    (t5955, t5962)
}
