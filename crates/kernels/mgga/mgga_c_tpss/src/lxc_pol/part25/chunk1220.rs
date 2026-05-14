//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1220/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1220<F: Float>(t116: F, t21785: F, t13119: F, t1339: F, t13798: F, t16037: F, t1663: F, t1760: F, t1796: F, t19577: F, t19620: F, t20134: F, t20137: F, t20224: F, t20322: F, t20346: F, t20368: F, t21017: F, t21253: F, t21750: F, t21790: F, t21856: F, t26848: F, t3493: F, t4478: F, t5314: F, t5706: F, t5799: F, t5939: F, t624: F, t6243: F, t63042: F, t6436: F, t6437: F, t646: F, t65533: F, t67541: F, t68868: F, t7383: F) -> (F, F) {
    let t71308 = t21785 * t116;
    let t71343 = 12.0 * t19620 * t7383 * t13798 - 2.0 * t71308 * t646 + 2.0 * t20322 * t1663 - t624 * t21750 - t5799 * t5314 - t1796 * t16037 + 6.0 * t1760 * t63042 * t21017 - t21253 * t5939 + 12.0 * t19620 * t26848 * t4478 + 12.0 * t68868 * t20134 + 6.0 * t6243 * t20137 + t5706 * t21856 - 2.0 * t6243 * t20224 + 2.0 * t19577 * t6437 - 2.0 * t1760 * t6436 * t13119 - 6.0 * t65533 * t20346 - 2.0 * t5706 * t21790 - 4.0 * t3493 * t20368 - 4.0 * t67541 * t1339;
    (t71308, t71343)
}
