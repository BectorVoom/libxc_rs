//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1345/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1345<F: Float>(t10461: F, t1163: F, t13133: F, t19261: F, t20706: F, t20786: F, t3538: F, t485: F, t5986: F, t5991: F, t65510: F, t65512: F, t65515: F, t65525: F, t65527: F, t65530: F, t65532: F, t65535: F, t65538: F, t65540: F, t65543: F, t65548: F, t65897: F, t68151: F) -> (F,) {
    let t68695 = -4.0 * t10461 * t5986 - 2.0 * t1163 * t20786 - 4.0 * t13133 * t5991 - 4.0 * t19261 * t3538 - 4.0 * t20706 * t3538 - t485 * t68151 - t65510 - t65512 + t65515 - t65525 - t65527 + t65530 + t65532 - t65535 + t65538 + t65540 - t65543 + t65548 + t65897;
    (t68695,)
}
