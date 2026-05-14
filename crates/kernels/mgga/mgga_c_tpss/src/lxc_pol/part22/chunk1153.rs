//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1153/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1153<F: Float>(t114: F, t1795: F, t645: F, t19588: F, t18396: F, t18622: F, t19591: F, t19593: F) -> (F, F) {
    let t115 = 1.0 < t114;
    let t20294 = t1795 * t645;
    let t20315 = 2.0 / 3.0 * t19588;
    let t20319 = piecewise3(t115, 0.0, t18622 + t18396 + t20315 + t19591 / 2.0 - t19593 / 4.0);
    (t20294, t20319)
}
