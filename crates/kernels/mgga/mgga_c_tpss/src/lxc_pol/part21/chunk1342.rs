//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1342/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1342<F: Float>(t1630: F, t60730: F, t60685: F, t60696: F, t60709: F, t60713: F, t65552: F, t65553: F, t65555: F, t65557: F, t65559: F, t65562: F, t65564: F, t18436: F, t4409: F, t12996: F, t5716: F) -> (F, F, F) {
    let t65567 = t60730 * t1630;
    let t65569 = t65552 + t65553 / 192.0 - 5.0 / 192.0 * t65555 - t65557 / 96.0 - t60685 + t65559 / 768.0 - t65562 - 7.0 / 1152.0 * t60696 - t65564 + 7.0 / 2304.0 * t60709 + 7.0 / 2304.0 * t60713 - 35.0 / 216.0 * t65567;
    let t65570 = t18436 * t4409;
    let t65571 = 7.0 / 72.0 * t65570;
    let t65572 = t5716 * t12996;
    (t65569, t65571, t65572)
}
