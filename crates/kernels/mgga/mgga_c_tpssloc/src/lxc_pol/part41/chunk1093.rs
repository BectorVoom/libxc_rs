//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1093/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1093<F: Float>(t11546: F, t18206: F, t11738: F, t1174: F, t1218: F, t1227: F, t1232: F, t15591: F, t15594: F, t15754: F, t1737: F, t1748: F, t19077: F, t19080: F, t19083: F, t19087: F, t19090: F, t19096: F, t19101: F, t3490: F, t4889: F, t5002: F, t5005: F, t5014: F, t5030: F, t5033: F, t6207: F, t6211: F) -> (F,) {
    let t19106 = t11546 * t18206;
    let t19117 = t11738 * t19077 / 3072.0 - t19080 * t1218 / 288.0 + t19083 * t1232 / 432.0 + t15754 / 648.0 - t1174 * t19087 / 72.0 + 11.0 / 324.0 * t19090 - 2.0 / 81.0 * t4889 * t5033 - t19096 / 4608.0 - t3490 * t6207 / 4608.0 - t1227 * t19101 / 4608.0 - t3490 * t6211 / 2304.0 - 7.0 / 648.0 * t1174 * t19106 + t15591 * t1737 / 1536.0 + t5002 * t5014 / 1536.0 - t15594 * t1748 / 2304.0 - t5005 * t5030 / 2304.0;
    (t19117,)
}
