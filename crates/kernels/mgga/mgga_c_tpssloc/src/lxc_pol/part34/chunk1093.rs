//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1093/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1093<F: Float>(t28: F, t265: F, t504: F, t108522: F, t108574: F, t108616: F, t1409: F, t20217: F, t2071: F, t29189: F, t52: F, t5398: F, t7884: F, t101150: F, t102386: F, t106755: F, t106971: F, t108533: F, t113: F, t1458: F, t1459: F, t1774: F, t1983: F, t20293: F, t20347: F, t2040: F, t20563: F, t20720: F, t2075: F, t2095: F, t2096: F, t24432: F, t24995: F, t26905: F, t27188: F, t28821: F, t28826: F, t28834: F, t28943: F, t28959: F, t29197: F, t29214: F, t29252: F, t4028: F, t5460: F, t652: F, t67001: F, t7042: F, t74014: F, t7685: F, t7687: F, t7943: F, t9016: F, t93966: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t108618 = piecewise3(t505, 0.0, t108522);
    let t108628 = piecewise3(t401, t108574 + t108616, t108618 * t52 / 2.0 - 3.0 / 2.0 * t29189 * t1409 - 3.0 / 2.0 * t7884 * t5398 - t2071 * t20217 / 2.0);
    let t108649 = -2.0 * t7042 * t20720 - 2.0 * t652 * t2075 * t20347 - 2.0 * t67001 * t2040 - 6.0 * t102386 * t1459 - 6.0 * t4028 * t29214 - 6.0 * t652 * t29197 * t1458 - 12.0 * t27188 * t5460 - 3.0 * t28821 * t7943 - t20293 * t2075 - 6.0 * t28959 * t1774 + t106755 * t2096 + 9.0 * t1983 * t26905 * t28834 - t113 * (t108533 + t108628) + 18.0 * t24995 * t9016 * t20563 - 18.0 * t24995 * t24432 * t106971 - 3.0 * t28943 * t1774 + 18.0 * t1983 * t93966 * t28826 + 9.0 * t1983 * t101150 * t7687 + 18.0 * t7685 * t29252 - t1983 * t2095 * t74014;
    (t108649,)
}
