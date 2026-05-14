//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1091/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1091<F: Float>(t25: F, t265: F, t394: F, t108522: F, t108096: F, t108466: F, t1409: F, t20217: F, t2064: F, t29149: F, t40: F, t5398: F, t7865: F, t106618: F, t106621: F, t106636: F, t106640: F, t106647: F, t106671: F, t106686: F, t106706: F, t106712: F, t108452: F, t1877: F, t20390: F, t2057: F, t24191: F, t2522: F, t26756: F, t28: F, t28771: F, t28789: F, t29106: F, t4314: F, t5966: F, t7114: F, t7649: F, t7845: F, t92319: F, t93000: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t108523 = piecewise3(t395, 0.0, t108522);
    let t108533 = piecewise3(t115, t108096 + t108466, t108523 * t40 / 2.0 + 3.0 / 2.0 * t29149 * t1409 + 3.0 / 2.0 * t7865 * t5398 + t2064 * t20217 / 2.0);
    let t108574 = -9.0 * t92319 * t28771 + t1877 * t108452 * t28 / 2.0 + t1877 * t2057 * t20390 / 2.0 - 9.0 / 2.0 * t24191 * t106621 + 3.0 * t1877 * t93000 * t28789 - 3.0 / 2.0 * t1877 * t7114 * t106712 - t1877 * t7114 * t106636 / 2.0 + 3.0 / 2.0 * t1877 * t7845 * t5966 - 9.0 * t24191 * t106706 - 3.0 / 2.0 * t1877 * t7114 * t106686 + 9.0 / 2.0 * t2522 * t2057 * t106647 + 3.0 * t26756 * t106618 + 9.0 * t4314 * t2057 * t106640 + 9.0 / 2.0 * t2522 * t29106 * t7649 + 9.0 * t24191 * t106671;
    (t108533, t108574)
}
