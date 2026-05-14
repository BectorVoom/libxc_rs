//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1218/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1218<F: Float>(t6323: F, t645: F, t116: F, t21907: F, t13133: F, t13554: F, t13565: F, t1760: F, t1800: F, t1845: F, t18690: F, t19620: F, t20289: F, t20343: F, t20368: F, t20379: F, t2056: F, t21180: F, t21868: F, t21908: F, t3493: F, t3499: F, t3542: F, t42710: F, t44034: F, t485: F, t50656: F, t5706: F, t5809: F, t5816: F, t6103: F, t626: F, t6328: F, t68838: F, t69023: F, t69026: F, t71159: F) -> (F, F, F) {
    let t71184 = t645 * t6323;
    let t71212 = t116 * t21907;
    let t71259 = -12.0 * t19620 * t18690 * t68838 - 2.0 * t42710 * t1800 - 2.0 * t50656 * t1800 - 2.0 * t13565 * t5809 - 4.0 * t20289 * t3542 - t1760 * t1845 * t44034 + 2.0 * t5706 * t21868 - 2.0 * t13565 * t5816 - 4.0 * t69023 * t1800 - 4.0 * t69026 * t1800 - 4.0 * t21180 * t5809 - 4.0 * t3493 * t20379 - 4.0 * t6103 * t20368 - 2.0 * t626 * t485 * t71159 - 4.0 * t13133 * t6328 - 4.0 * t13554 * t6328 - 4.0 * t3493 * t20343 - 2.0 * t2056 * t21908 - 2.0 * t3499 * t21908;
    (t71184, t71212, t71259)
}
