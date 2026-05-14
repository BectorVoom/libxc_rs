//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1225/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1225<F: Float>(t1692: F, t1812: F, t7622: F, t555: F, t1288: F, t18728: F, t18803: F, t18812: F, t19681: F, t19685: F, t19810: F, t20417: F, t20526: F, t2439: F, t35530: F, t5849: F, t5853: F, t6153: F, t62610: F, t62820: F, t6331: F, t63794: F, t63797: F, t63817: F, t63860: F, t63881: F, t64249: F, t64292: F) -> (F, F, F) {
    let t66631 = 3.0 * t1692 * t1812 * t7622;
    let t66641 = t1692 * t1812 * t555;
    let t66656 = -t1692 * t62820 * t6153 / 2.0 + t1692 * t18803 * t1288 / 2.0 + 3.0 * t35530 * t6331 - 6.0 * t20417 * t63860 - t66631 + 3.0 * t18728 * t63797 + 3.0 * t2439 * t5849 * t19685 + 3.0 / 2.0 * t2439 * t1812 * t64292 + t66641 - 3.0 * t20526 * t64249 + t1692 * t18812 * t63794 - t1692 * t5853 * t63817 / 2.0 + 3.0 * t2439 * t5849 * t19681 - 3.0 * t62610 * t19810 - 3.0 * t18728 * t63881;
    (t66631, t66641, t66656)
}
