//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2266;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2267;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta561<F: Float>(t1196: F, t16558: F, t974: F, t1215: F, t1653: F, t15659: F, t3578: F, t1177: F, t18221: F, t18237: F, t1735: F, t4724: F, t11668: F, t18232: F, t3440: F, t1017: F, t6163: F, t1210: F, t1207: F, t11665: F, t11678: F, t1174: F, t11834: F, t1218: F, t15569: F, t15717: F, t15719: F, t15722: F, t15740: F, t3577: F, t4889: F, t4950: F, t4954: F, t4969: F, t5046: F, t6192: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t18996, t18997, t19000, t19001, t19002, t19005, t19010, t19015) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2266::<F>(t1196, t16558, t974, t1215, t1653, t15659, t3578, t1177, t18221, t18237, t1735, t4724);
        let (t19016, t19025, t19026, t19029) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2267::<F>(t11668, t19015, t18232, t3440, t1017, t6163, t1210, t1207, t11665, t11678, t1174, t11834, t1218, t15569, t15717, t15719, t15722, t15740, t18997, t19002, t19005, t19010, t3577, t4889, t4950, t4954, t4969, t5046, t6192);
    (t18996, t19000, t19001, t19002, t19015, t19016, t19025, t19026, t19029)
}
