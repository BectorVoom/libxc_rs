//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1150/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1150<F: Float>(t1792: F, t18649: F, t19388: F, t19396: F, t19404: F, t19408: F, t19411: F, t19414: F, t19417: F, t20246: F, t20255: F, t20257: F, t5489: F, t5492: F, t5785: F, t5794: F, t6077: F, t6080: F, t6304: F) -> (F,) {
    let t20259 = -5.0 / 3.0 * t5785 * t19408 - 2.0 / 3.0 * t19411 * t1792 - 2.0 / 3.0 * t19414 * t1792 - 2.0 / 3.0 * t19417 * t1792 - 2.0 / 3.0 * t6080 * t5794 - 5.0 / 3.0 * t5785 * t19388 - 2.0 / 3.0 * t5492 * t6304 - 5.0 / 3.0 * t20246 * t5489 - 2.0 / 3.0 * t19396 * t1792 - 5.0 / 3.0 * t18649 * t6077 - 5.0 / 3.0 * t5785 * t19404 + 16.0 / 9.0 * t20255 + 40.0 / 9.0 * t20257;
    (t20259,)
}
