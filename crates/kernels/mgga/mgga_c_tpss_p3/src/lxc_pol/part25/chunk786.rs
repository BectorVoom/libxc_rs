//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 786/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk786<F: Float>(t3365: F, t5432: F, t1260: F, t1640: F, t220: F, t3370: F, t339: F, t4511: F, t523: F, t5381: F, t5408: F, t5413: F, t5427: F) -> (F, F) {
    let t5433 = t3365 * t5432;
    let t5448 = -t1260 * t339 * t5408 - t1260 * t339 * t5413 - F::new(2.0) * t1640 * t339 * t4511 + t220 * t523 * t5427 + F::new(2.0) * t3370 * t339 * t5381;
    (t5433, t5448)
}
