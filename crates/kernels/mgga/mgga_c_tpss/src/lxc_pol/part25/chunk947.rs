//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 947/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk947<F: Float>(t5366: F, t541: F, t10019: F, t10028: F, t1196: F, t1206: F, t12673: F, t1268: F, t1270: F, t12757: F, t12769: F, t12780: F, t13627: F, t13631: F, t13637: F, t13641: F, t13645: F, t13671: F, t13808: F, t13810: F, t13943: F, t1625: F, t198: F, t3183: F, t4397: F, t4478: F, t4524: F, t4528: F, t4532: F, t509: F, t9972: F, t9980: F) -> (F,) {
    let t13950 = t541 * t5366;
    let t13954 = -t12757 + 2.0 * t4524 * t13627 * t1268 + t13631 - t12769 - t9972 + 12.0 * t4532 * t4528 * t4478 - t13637 + 6.0 * t3183 * t12673 * t1625 - 3.0 * t3183 * t13641 * t1206 - t9980 + t13645 + t10019 + t12780 + 3.0 * t198 * t1196 * t13671 + t198 * t509 * t13943 * t1270 - t10028 - t13808 + 6.0 * t3183 * t4528 * t4397 - t13810 + 6.0 * t4532 * t13950 * t1206;
    (t13954,)
}
