//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1022/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1022<F: Float>(t14102: F, t4582: F, t3041: F, t4593: F, t1031: F, t4616: F, t1612: F, t3082: F, t1025: F, t1041: F, t1046: F, t10873: F, t10883: F, t10952: F, t10965: F, t14077: F, t14080: F, t14084: F, t14085: F, t14093: F, t14099: F, t1622: F, t3039: F, t3048: F, t3117: F, t378: F, t4585: F, t4590: F, t4600: F, t4636: F) -> (F,) {
    let t14103 = t4582 * t14102;
    let t14106 = t4593 * t3041;
    let t14107 = t4582 * t14106;
    let t14114 = t4616 * t1031;
    let t14117 = t1612 * t3082;
    let t14120 = -t14077 * t1025 / 288.0 - t14080 * t1046 / 432.0 + t14084 + t14085 * t1046 / 2304.0 + t10965 * t1622 / 4608.0 + t3117 * t4636 / 2304.0 + t1041 * t14093 / 4608.0 - t10952 * t4600 / 1536.0 - t3039 * t14099 / 1536.0 - t3039 * t14103 / 3072.0 + t10883 * t14107 / 3072.0 + t3048 * t4585 / 216.0 - 5.0 / 1296.0 * t3048 * t4590 - t14114 * t378 / 288.0 - t14117 / 13824.0 - t10873 / 648.0;
    (t14120,)
}
