//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 720/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk720<F: Float>(t5624: F, t6621: F, t5572: F, t6581: F, t23141: F, t23144: F, t25109: F, t25126: F, t25133: F, t26644: F, t26646: F, t28380: F, t28384: F, t28386: F, t28390: F, t28397: F, t28399: F) -> (F, F, F) {
    let t28401 = t6621 * t5624;
    let t28403 = t6581 * t5572;
    let t28405 = 0.16956557559538964159e-1 * t25109 + t28380 / 192.0 - 0.12111826828242117256e-2 * t28384 + t28386 / 16.0 + 0.84782787797694820792e-2 * t28390 + 0.28260929265898273598e-2 * t25126 + 0.6728792682356731809e-4 * t25133 + 0.24223653656484234512e-2 * t28397 + t26644 - t28399 / 192.0 + 5.0 / 384.0 * t28401 + t26646 - t28403 / 48.0 + t23141 + t23144;
    (t28401, t28403, t28405)
}
