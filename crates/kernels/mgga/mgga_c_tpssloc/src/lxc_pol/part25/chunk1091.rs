//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1091/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1091<F: Float>(t28: F, t265: F, t504: F, t85243: F, t2071: F, t2250: F, t24420: F, t52: F, t607: F, t7150: F, t85296: F, t85337: F, t9258: F, t113: F, t11968: F, t12156: F, t1390: F, t15904: F, t1983: F, t2036: F, t2075: F, t2094: F, t2095: F, t22574: F, t22596: F, t22607: F, t2312: F, t23857: F, t23958: F, t24169: F, t24428: F, t24432: F, t24433: F, t26161: F, t26558: F, t32193: F, t39367: F, t510: F, t55173: F, t55246: F, t650: F, t652: F, t671: F, t6876: F, t7156: F, t7171: F, t7217: F, t7218: F, t83695: F, t83886: F, t84149: F, t84733: F, t85254: F, t9351: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t85339 = piecewise3(t505, 0.0, t85243);
    let t85349 = piecewise3(t401, t85296 + t85337, t85339 * t52 / 2.0 - 3.0 / 2.0 * t24420 * t607 - 3.0 / 2.0 * t7150 * t2250 - t2071 * t9258 / 2.0);
    let t85370 = -6.0 * t1983 * t2095 * t83695 - 9.0 * t22574 * t24432 * t55246 + 9.0 * t22607 * t7171 - 9.0 * t22574 * t24432 * t39367 - 6.0 * t84149 * t510 + 18.0 * t1983 * t84733 * t22596 - 3.0 * t650 * t24428 - 3.0 * t2312 * t7156 - 6.0 * t9351 * t2075 + 3.0 * t22607 * t7218 - 18.0 * t83886 * t24433 - 18.0 * t22574 * t32193 * t15904 - t113 * (t85254 + t85349) + 6.0 * t1983 * t12156 * t2094 * t1390 - t2036 * t11968 + 6.0 * t6876 * t24169 - 6.0 * t652 * t24428 * t671 + 6.0 * t26161 * t26558 * t55173 + 6.0 * t1983 * t7217 * t23857 + 18.0 * t6876 * t23958;
    (t85370,)
}
