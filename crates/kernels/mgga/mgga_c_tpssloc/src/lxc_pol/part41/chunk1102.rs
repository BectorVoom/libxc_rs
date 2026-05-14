//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1102/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1102<F: Float>(t28: F, t265: F, t504: F, t17133: F, t19266: F, t19274: F, t1081: F, t1260: F, t1409: F, t1649: F, t16558: F, t17141: F, t1768: F, t18196: F, t3966: F, t4324: F, t506: F, t5099: F, t52: F, t5398: F, t5669: F, t5966: F, t607: F, t6279: F, t873: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t19276 = piecewise3(t505, t19266 + t19274, t17133);
    let t19288 = piecewise3(t401, t17133 * t28 / 2.0 + t5669 * t1081 / 2.0 + t4324 * t1649 - t17141 + t873 * t5966 / 2.0 + t265 * t18196 / 2.0, t19276 * t52 / 2.0 - t6279 * t607 / 2.0 - t5099 * t1409 - t1768 * t3966 - t1260 * t5398 / 2.0 - t506 * t16558 / 2.0);
    (t19288,)
}
