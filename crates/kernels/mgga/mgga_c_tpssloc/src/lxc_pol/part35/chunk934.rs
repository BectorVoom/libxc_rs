//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 934/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk934<F: Float>(t21180: F, t901: F, t1547: F, t5698: F, t10599: F, t10595: F, t13598: F, t13642: F, t17149: F, t17165: F, t17175: F, t17286: F, t17288: F, t17290: F, t21161: F, t21168: F, t21181: F) -> (F, F, F, F) {
    let t21183 = t901 * t21180;
    let t21185 = t5698 * t1547;
    let t21186 = t10599 * t21185;
    let t21188 = t10595 * t21185;
    let t21193 = -0.34731666666666666667e0 * t13642 + 0.62517e0 * t21161 - 0.68863333333333333332e0 * t13598 + 0.34431666666666666666e0 * t17149 - 0.103295e1 * t17165 + 0.51647499999999999999e0 * t17175 - 0.104195e0 * t21168 + 0.3529725e1 * t21181 + 0.6311625e0 * t21183 - 0.157790625e0 * t21186 + 0.264729375e1 * t21188 + 0.69463333333333333335e-1 * t17286 - 0.41678000000000000001e0 * t17288 + 0.20839e0 * t17290;
    (t21183, t21186, t21188, t21193)
}
