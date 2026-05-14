//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 917/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk917<F: Float>(t1992: F, t22635: F, t31558: F, t6460: F, t122166: F, t6888: F, t7691: F, t102948: F, t113963: F, t12021: F, t122178: F, t122210: F, t122297: F, t127220: F, t127229: F, t127242: F, t127249: F, t127316: F, t1375: F, t1843: F, t2015: F, t2016: F, t2092: F, t28187: F, t29360: F, t33323: F, t3887: F, t6439: F, t7194: F, t8636: F, t97558: F, t97740: F) -> (F,) {
    let t128705 = t1992 * t22635 * t31558 * t6460;
    let t128724 = t6888 * t122166 * t7691;
    let t128726 = -t7194 * t28187 + 0.16449340668482264365e-1 * t128705 + 2.0 * t1375 * t3887 * t29360 * t2015 + t127220 - 0.16449340668482264365e-1 * t122178 + t127229 - 12.0 * t97740 * t33323 - t113963 - 2.0 * t122297 * t1843 - t127242 - 6.0 * t1375 * t12021 * t8636 * t6439 - t97558 * t2092 - t102948 * t2016 + t127249 + 0.38381794893125283518e-1 * t122210 - 0.3289868133696452873e-1 * t128724 + t127316;
    (t128726,)
}
