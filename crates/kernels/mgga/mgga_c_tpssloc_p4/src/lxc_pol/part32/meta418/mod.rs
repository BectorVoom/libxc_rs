//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1616;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1617;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1618;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1619;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta418<F: Float>(t15453: F, t17686: F, t4582: F, t17635: F, t4972: F, t1090: F, t6230: F, t3578: F, t6219: F, t4997: F, t5002: F, t11784: F, t248: F, t5971: F, t1227: F, t5019: F, t4993: F, t5005: F, t1202: F, t6164: F, t5024: F, t11692: F, t11792: F, t11821: F, t15671: F, t15691: F, t15699: F, t15740: F, t3577: F, t488: F, t4950: F, t1196: F, t16558: F, t974: F, t1215: F, t1653: F, t15659: F, t1177: F, t18221: F, t18237: F, t1735: F, t4724: F, t11668: F, t18232: F, t3440: F, t1017: F, t6163: F, t1210: F, t1207: F, t11665: F, t11678: F, t1174: F, t11834: F, t1218: F, t15569: F, t15717: F, t15719: F, t15722: F, t4889: F, t4954: F, t4969: F, t5046: F, t6192: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t18955, t18959, t18965, t18969, t18972, t18975) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1616::<F>(t15453, t17686, t4582, t17635, t4972, t1090, t6230, t3578, t6219, t4997, t5002, t11784, t248, t5971);
        let t18989 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1617::<F>(t1227, t18975, t4997, t5019, t4993, t5005, t1202, t6164, t5024, t11692, t11792, t11821, t15671, t15691, t15699, t15740, t18955, t18959, t18965, t18969, t18972, t3577, t488, t4950);
        let (t18997, t19002, t19005, t19010, t19015) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1618::<F>(t1196, t16558, t974, t1215, t1653, t15659, t3578, t1177, t18221, t18237, t1735, t4724);
        let (t19016, t19024, t19029) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1619::<F>(t11668, t19015, t18232, t3440, t1017, t6163, t1210, t1207, t11665, t11678, t1174, t11834, t1218, t15569, t15717, t15719, t15722, t15740, t18997, t19002, t19005, t19010, t3577, t4889, t4950, t4954, t4969, t5046, t6192);
    (t18955, t18959, t18965, t18969, t18975, t18989, t18997, t19002, t19016, t19024, t19029)
}
