//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1279/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1279<F: Float>(t2056: F, t21190: F, t4347: F, t1976: F, t4573: F, t4570: F, t615: F, t77: F, t10289: F, t1290: F, t3418: F, t3426: F, t3432: F, t1680: F, t18345: F, t19393: F, t19404: F, t19408: F, t21116: F, t21136: F, t5503: F, t6077: F, t62027: F, t62030: F, t65336: F, t65339: F) -> (F, F, F) {
    let t69084 = 2.0 * t2056 * t21190;
    let t69086 = 2.0 * t4347 * t21190;
    let t69087 = t1976 * t4573;
    let t69097 = t77 * t615 * t4570;
    let t69108 = t10289 * t1290;
    let t69111 = t3418 * t3426;
    let t69114 = t3418 * t3432;
    let t69117 = t69087 * t1680 / 3.0 + t21136 * t5503 / 3.0 - 5.0 * t62027 * t21116 - 5.0 * t62030 * t21116 - 5.0 * t18345 * t69097 + 5.0 / 3.0 * t65336 * t6077 + 5.0 / 3.0 * t65339 * t6077 + 5.0 / 3.0 * t19393 * t19404 + 5.0 / 3.0 * t19393 * t19408 + 2.0 / 3.0 * t69108 * t1680 + 2.0 / 3.0 * t69111 * t1680 + 2.0 / 3.0 * t69114 * t1680;
    (t69084, t69086, t69117)
}
