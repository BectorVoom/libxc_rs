//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1013/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1013<F: Float>(t14109: F, t187: F, t8101: F, t2436: F, t4802: F, t10521: F, t10520: F, t14057: F, t14061: F, t14064: F, t14065: F, t14068: F, t14072: F, t14076: F, t14080: F, t1692: F, t2439: F, t3724: F, t3728: F, t750: F, t7945: F, t7954: F, t7960: F, t7972: F, t7975: F, t8112: F, t821: F) -> (F, F, F, F) {
    let t14111 = 0.19751673498613801407e-1 * t14109 * t187;
    let t14112 = 4.0 * t8101;
    let t14113 = t4802 * t2436;
    let t14116 = 2.0 * t10521;
    let t14117 = -6.0 * t14076 * t2439 * t3728 + 3.0 * t14080 * t2439 * t750 - t14113 * t1692 * t821 - 2.0 * t1692 * t3724 * t3728 + t10520 - t14057 + t14061 + t14064 + t14065 + t14068 + t14072 + t14111 + t14112 + t14116 + t7945 - t7954 - t7960 + t7972 + t7975 + t8112;
    (t14111, t14112, t14116, t14117)
}
