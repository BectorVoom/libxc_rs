//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1238/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1238<F: Float>(t1270: F, t13133: F, t1339: F, t13554: F, t16037: F, t1760: F, t1799: F, t18547: F, t19577: F, t19579: F, t19604: F, t19609: F, t20226: F, t20227: F, t20289: F, t20357: F, t20374: F, t20386: F, t2056: F, t21855: F, t21883: F, t21894: F, t21900: F, t25469: F, t3493: F, t3499: F, t3502: F, t509: F, t544: F, t5706: F, t5757: F, t6103: F, t61801: F, t6243: F, t626: F, t6324: F, t6413: F, t68798: F, t71344: F, t71574: F, t71603: F, t71662: F, t71715: F, t71823: F, t71872: F) -> (F,) {
    let t71878 = -4.0 * t71344 * t1339 - 4.0 * t20289 * t3502 + 6.0 * t6243 * t20227 - 6.0 * t18547 * t25469 * t19609 - 6.0 * t61801 * t21900 + 4.0 * t19579 * t20357 * t68798 - 4.0 * t6103 * t20374 - 2.0 * t2056 * t21894 - 2.0 * t3499 * t21894 - 2.0 * t626 * t16037 * t1799 - 4.0 * t13133 * t6324 - 4.0 * t13554 * t6324 - 4.0 * t3493 * t20386 + (t71574 + t71603) * t544 + 6.0 * t19577 * t6413 - t1760 * t21855 * t5757 + 6.0 * t1760 * t20226 * t19604 + 3.0 * t5706 * t21883 + t1760 * t509 * (t71662 + t71715 + t71823 + t71872) * t1270;
    (t71878,)
}
