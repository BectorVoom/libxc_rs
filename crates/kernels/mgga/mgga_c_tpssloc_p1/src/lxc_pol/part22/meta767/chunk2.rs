//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2594/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2594<F: Float>(t19033: F, t4993: F, t19046: F, t5018: F, t5023: F, t6169: F, t11546: F, t1174: F, t1218: F, t1227: F, t1230: F, t1232: F, t15498: F, t1737: F, t1748: F, t19026: F, t19087: F, t22214: F, t22218: F, t248: F, t3490: F, t4889: F, t5014: F, t5030: F, t6211: F, t66147: F, t66150: F, t71148: F, t71158: F) -> F {
    let t72302 = t19033 * t4993;
    let t72304 = t19046 * t5018;
    let t72307 = t6169 * t5023;
    let t72333 = -F::new(19.0) / F::new(1296.0) * t72302 - t72304 * t1218 / F::new(192.0) + t72307 * t1232 / F::new(288.0) + t15498 * t6211 / F::new(144.0) - t3490 * t22214 / F::new(4608.0) - t1227 * t248 * t1230 * t71148 / F::new(4608.0) - t3490 * t22218 / F::new(768.0) + F::new(19.0) / F::new(576.0) * t66147 * t1737 + F::new(19.0) / F::new(576.0) * t19026 * t5014 - F::new(19.0) / F::new(864.0) * t66150 * t1748 - F::new(19.0) / F::new(864.0) * t19033 * t5030 - F::new(7.0) / F::new(216.0) * t1174 * t11546 * t71158 + t4889 * t19087 / F::new(9.0);
    t72333
}
