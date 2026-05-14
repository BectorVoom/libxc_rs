//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1199/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1199<F: Float>(t1860: F, t19345: F, t38: F, t6470: F, t1981: F, t10292: F, t5965: F, t18350: F, t1861: F, t19192: F, t19229: F, t19232: F, t19235: F, t19342: F, t19349: F, t19388: F, t19396: F, t19404: F, t19408: F, t5489: F, t5492: F, t5966: F, t6077: F, t6472: F, t6475: F) -> (F, F, F, F, F) {
    let t20713 = t1860 * t19345;
    let t20718 = t38 * t6470;
    let t20719 = t1981 * t20718;
    let t20728 = t10292 * t5965;
    let t20741 = -5.0 * t19232 * t19342 - 5.0 / 3.0 * t18350 * t20713 - 5.0 / 3.0 * t19349 * t19235 + 5.0 / 6.0 * t20719 * t5489 + t5492 * t6472 / 3.0 + 5.0 / 6.0 * t5966 * t19388 + t5492 * t6475 / 3.0 + 5.0 / 6.0 * t20728 * t5489 + t19396 * t1861 / 3.0 + 5.0 / 6.0 * t19229 * t6077 + 5.0 / 6.0 * t19192 * t6077 + 5.0 / 6.0 * t5966 * t19404 + 5.0 / 6.0 * t5966 * t19408;
    (t20713, t20718, t20719, t20728, t20741)
}
