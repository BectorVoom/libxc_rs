//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 774/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk774<F: Float>(t10311: F, t10318: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F, t10589: F, t10591: F, t10597: F, t10600: F, t10789: F, t932: F) -> (F,) {
    let t10804 = 0.20839e0 * t10311 - 0.62517e0 * t10318 - 0.68863333333333333332e0 * t10556 + 0.34431666666666666666e0 * t10558 - 0.103295e1 * t10560 + 0.51647499999999999999e0 * t10562 - 0.57386111111111111112e0 * t10566 + 0.20659e1 * t10569 - 0.309885e1 * t10572 - 0.516475e0 * t10575 + 0.3529725e1 * t10589 + 0.6311625e0 * t10591 + 0.264729375e1 * t10597 - 0.157790625e0 * t10600;
    let t10805 = t10789 + t10804;
    let t10806 = t10805 * t932;
    (t10806,)
}
