//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1113/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1113<F: Float>(t22633: F, t22635: F, t31090: F, t97721: F, t1377: F, t7749: F, t1307: F, t225: F, t32708: F, t1992: F, t32693: F, t80650: F, t113934: F, t113941: F, t114197: F, t120180: F, t120184: F, t1386: F, t16030: F, t16460: F, t1843: F, t2016: F, t22670: F, t26366: F, t32758: F, t3758: F, t6993: F, t7750: F, t8476: F, t8486: F, t90732: F) -> (F,) {
    let t120196 = 0.6579736267392905746e-1 * t22633 * t22635 * t31090 * t97721;
    let t120197 = t1377 * t7749;
    let t120201 = 0.3289868133696452873e-1 * t22633 * t22635 * t120197 * t1307;
    let t120203 = t32708 * t225;
    let t120209 = 0.3289868133696452873e-1 * t1992 * t80650 * t32693;
    let t120210 = -t114197 * t1843 - t120203 * t1386 - t16030 * t8486 + 2.0 * t16460 * t8476 - 2.0 * t2016 * t90732 - 2.0 * t22670 * t7750 - 2.0 * t26366 * t6993 - t32758 * t3758 + t113934 - t113941 + t120180 + t120184 - t120196 + t120201 + t120209;
    (t120210,)
}
