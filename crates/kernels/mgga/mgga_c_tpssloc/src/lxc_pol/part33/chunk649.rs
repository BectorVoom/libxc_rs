//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 649/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk649<F: Float>(t491: F, t6150: F, t1720: F, t1751: F, t1730: F, t1743: F, t1417: F, t47: F, t480: F, t479: F, t471: F, t225: F) -> (F, F, F, F, F, F, F) {
    let t6151 = t6150 * t491;
    let t6153 = t1720 * t1751;
    let t6158 = t1730 * t1743;
    let t6163 = F::new(1.0) / t47 / t480 / t1417;
    let t6164 = t479 * t6163;
    let t6165 = t471 * t6164;
    let t6168 = t6150 * t225;
    (t6151, t6153, t6158, t6163, t6164, t6165, t6168)
}
