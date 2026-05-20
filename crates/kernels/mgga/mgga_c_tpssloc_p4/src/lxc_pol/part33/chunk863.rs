//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 863/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk863<F: Float>(t1675: F, t3331: F, t15026: F, t3623: F, t1706: F, t3428: F, t11529: F, t1709: F, t1174: F, t11588: F, t1714: F, t1716: F, t698: F) -> (F, F, F, F, F, F) {
    let t15207 = t1675 * t3331;
    let t15245 = t15026 * t3623;
    let t15265 = t1706 * t3428;
    let t15299 = t11529 * t1709;
    let t15300 = t1174 * t15299;
    let t15338 = t11588 * t1714;
    let t15363 = t698 * t1716;
    (t15207, t15245, t15265, t15300, t15338, t15363)
}
