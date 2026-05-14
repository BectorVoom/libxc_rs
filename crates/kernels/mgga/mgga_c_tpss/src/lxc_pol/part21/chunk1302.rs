//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1302/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1302<F: Float>(t11506: F, t18094: F, t11493: F, t11565: F, t11602: F, t11649: F, t11653: F, t11675: F, t18069: F, t18098: F, t61401: F, t61409: F, t61411: F, t61417: F, t61432: F, t18110: F, t3916: F) -> (F, F) {
    let t64455 = t18094 * t11506 / 576.0;
    let t64470 = 5.0 / 10368.0 * t61401 + t61409 / 1152.0 + t64455 + t18069 * t11675 / 1152.0 + t18069 * t11565 / 2304.0 + t61432 * t11653 / 1152.0 - t18069 * t11649 / 1152.0 - t18098 * t11493 / 768.0 + t18069 * t11602 / 1152.0 - t61411 / 432.0 + t61417 / 864.0;
    let t64477 = t18110 * t3916 / 162.0;
    (t64470, t64477)
}
