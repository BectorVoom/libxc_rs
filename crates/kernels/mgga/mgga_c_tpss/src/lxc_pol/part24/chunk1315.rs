//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1315/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1315<F: Float>(t14349: F, t1705: F, t935: F, t1395: F, t14367: F, t14372: F, t14423: F, t14424: F, t1702: F, t1710: F, t17984: F, t17993: F, t18000: F, t19724: F, t19727: F, t19736: F, t19744: F, t19762: F, t19775: F, t19794: F, t21290: F, t21317: F, t21320: F, t21331: F, t21342: F, t3721: F, t3722: F, t4784: F, t5565: F, t5568: F, t5571: F, t5572: F, t5580: F, t61226: F, t6135: F, t6137: F, t6143: F, t64028: F, t64135: F, t69912: F, t818: F) -> (F,) {
    let t70189 = t1705 * t14349 * t935;
    let t70210 = -12.0 * t5571 * t18000 * t6137 * t3721 + 2.0 * t64135 * t6143 + 2.0 * t19736 * t19775 - 2.0 * t19727 * t3722 + 2.0 * t5565 * t14372 - t5565 * t14424 + 2.0 * t17993 * t21331 + 2.0 * t17984 * t4784 - t5568 * t21342 + 12.0 * t61226 * t64028 * t19762 - 2.0 * t6135 * t19794 + 2.0 * t5571 * t5572 * t21290 * t818 - t70189 * t1710 + 2.0 * t5571 * t5572 * t1702 * t14423 + t69912 * t5580 - 12.0 * t19736 * t19744 - 6.0 * t5571 * t18000 * t21320 * t818 + 4.0 * t5571 * t5572 * t19724 * t1395 + 4.0 * t17993 * t21317 + 4.0 * t5565 * t14367;
    (t70210,)
}
