//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 742/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk742<F: Float>(t28: F, t265: F, t504: F, t29148: F, t1409: F, t2071: F, t29188: F, t52: F, t5398: F, t7884: F, t29156: F, t5161: F, t7940: F, t1458: F, t7890: F, t113: F, t1442: F, t1459: F, t1774: F, t1849: F, t1983: F, t2075: F, t2096: F, t27188: F, t28821: F, t28943: F, t28952: F, t28959: F, t28969: F, t4028: F, t510: F, t5450: F, t5457: F, t5460: F, t5494: F, t652: F, t7042: F, t7458: F, t7685: F, t7787: F, t7802: F, t7806: F, t7900: F, t7941: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t29189 = piecewise3(t505, 0.0, t29148);
    let t29196 = piecewise3(t401, t29188, t29189 * t52 / 2.0 - t7884 * t1409 - t2071 * t5398 / 2.0);
    let t29197 = t29156 + t29196;
    let t29201 = t7940 * t5161;
    let t29205 = t7890 * t1458;
    let t29210 = -4.0 * t27188 * t1459 - 2.0 * t7042 * t5494 - t28943 * t510 + 2.0 * t7685 * t7941 - 2.0 * t652 * t28952 - 4.0 * t4028 * t7806 - 4.0 * t7042 * t5460 - 2.0 * t28959 * t510 - t5450 * t2075 - 2.0 * t1442 * t7890 - 2.0 * t7787 * t1774 + 2.0 * t7900 * t1849 + 3.0 * t1983 * t28969 - t113 * t29197 - 2.0 * t5457 * t2075 - 2.0 * t1983 * t29201 + t28821 * t2096 - 4.0 * t652 * t29205 - 4.0 * t7458 * t7802;
    (t29197, t29201, t29205, t29210)
}
