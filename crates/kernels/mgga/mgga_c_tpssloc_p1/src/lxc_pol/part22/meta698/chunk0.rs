//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2281/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2281<F: Float>(t18975: F, t3490: F, t3540: F, t6165: F, t19083: F, t3523: F, t19026: F, t3572: F, t19033: F, t11734: F, t19095: F, t15486: F, t5005: F) -> (F, F, F, F, F, F, F) {
    let t65662 = t3490 * t18975;
    let t65664 = t6165 * t3540;
    let t65668 = t19083 * t3523;
    let t65670 = t19026 * t3572;
    let t65672 = t19033 * t3523;
    let t65674 = t11734 * t19095;
    let t65676 = t5005 * t15486;
    (t65662, t65664, t65668, t65670, t65672, t65674, t65676)
}
