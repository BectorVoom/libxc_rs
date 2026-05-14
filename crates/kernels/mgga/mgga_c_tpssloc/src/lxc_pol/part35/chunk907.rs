//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 907/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk907<F: Float>(t40: F, t52: F, t13107: F, t1530: F, t5664: F, t20217: F, t20234: F, t4104: F, t5398: F, t634: F, t767: F, t4111: F, t638: F, t771: F, zeta_threshold: F) -> (F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t20777 = 0.51947577317044391276e2 * t13107;
    let t20778 = t5664 * t1530;
    let t20790 = piecewise3(t146, 0.0, 8.0 / 27.0 * t634 * t20234 - 2.0 / 3.0 * t4104 * t5398 + 2.0 / 3.0 * t767 * t20217);
    let t20798 = piecewise3(t150, 0.0, -8.0 / 27.0 * t638 * t20234 - 2.0 / 3.0 * t4111 * t5398 - 2.0 / 3.0 * t771 * t20217);
    let t20800 = t20790 / 2.0 + t20798 / 2.0;
    (t20777, t20778, t20800)
}
