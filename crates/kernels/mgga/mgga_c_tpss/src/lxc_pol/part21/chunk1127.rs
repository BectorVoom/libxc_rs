//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1127/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1127<F: Float>(t1253: F, t1639: F, t4459: F, t532: F, t1649: F, t3255: F, t1219: F, t4487: F, t10111: F, t10204: F, t1233: F, t1260: F, t12892: F, t12958: F, t13032: F, t13059: F, t1640: F, t220: F, t3261: F, t3327: F, t3332: F, t3374: F, t339: F, t4417: F, t4460: F, t4498: F, t4499: F, t4508: F, t4511: F, t523: F) -> (F,) {
    let t13063 = t1253 * t1639;
    let t13067 = t532 * t4459;
    let t13094 = t3255 * t1649;
    let t13098 = t1219 * t4487;
    let t13108 = 2.0 * t10111 * t4498 * t4499 - t10204 * t1640 * t339 - 2.0 * t1233 * t13063 * t4508 - 2.0 * t1233 * t13067 * t4508 - 2.0 * t1233 * t13098 * t339 - t1260 * t12958 * t339 - 6.0 * t12892 * t13059 * t4499 + t13032 * t220 * t523 + 4.0 * t13063 * t4417 * t4498 + 4.0 * t13067 * t4417 * t4498 + 2.0 * t13094 * t3261 * t339 + 6.0 * t3261 * t4498 * t4499 - t3327 * t339 * t4511 - t3327 * t4499 * t4508 - t3332 * t339 * t4511 - t3332 * t4499 * t4508 - 2.0 * t3374 * t339 * t4460;
    (t13108,)
}
