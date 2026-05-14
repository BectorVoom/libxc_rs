//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 954/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk954<F: Float>(t24230: F, t24231: F, t25109: F, t25126: F, t25133: F, t25140: F, t25144: F, t28380: F, t28384: F, t28386: F, t28390: F, t28397: F, t28399: F, t28401: F, t28403: F, t29025: F) -> (F,) {
    let t29039 = 0.33913115119077928316e-1 * t25109 + t28380 / 96.0 - 0.24223653656484234512e-2 * t28384 + t28386 / 8.0 + 0.16956557559538964158e-1 * t28390 + 0.56521858531796547194e-2 * t25126 + 0.13457585364713463618e-3 * t25133 + 0.48447307312968469024e-2 * t28397 + 7.0 / 36.0 * t25140 - t28399 / 96.0 + 5.0 / 192.0 * t28401 + 7.0 / 576.0 * t25144 - t28403 / 24.0 + t24230 + t24231;
    let t29040 = t29025 + t29039;
    (t29040,)
}
