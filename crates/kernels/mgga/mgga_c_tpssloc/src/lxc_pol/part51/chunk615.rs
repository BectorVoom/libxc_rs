//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 615/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk615<F: Float>(t1378: F, t5353: F, t1375: F, t1386: F, t1843: F, t3758: F, t3882: F, t5211: F, t5213: F, t5215: F, t5217: F, t5319: F, t5321: F, t5326: F, t568: F, t1297: F, t1390: F, t193: F, t2426: F, t2486: F, t3819: F, t3821: F, t3825: F, t3827: F, t3832: F, t5167: F, t5169: F, t5187: F, t5263: F, t5265: F, t5267: F, t5268: F, t5269: F, t533: F) -> (F, F, F) {
    let t5354 = t1378 * t5353;
    let t5356 = 2.0 * t1375 * t5326 - t1375 * t5354 - t1386 * t5215 - t1386 * t5321 - t1843 * t3758 - t1843 * t3882 + t5211 * t568 + t5213 * t568 + t5217 * t568 + t5319 * t568;
    let t5360 = t1390 * t193 * t533 * t5356 + 3.0 * t1297 * t193 * t5187 - t2426 - t2486 + t3819 - t3821 + t3825 + t3827 - t3832 + t5167 + t5169 - t5263 + t5265 - t5267 - t5268 - t5269;
    (t5354, t5356, t5360)
}
