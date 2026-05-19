//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1130/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1130<F: Float>(t18554: F, t4934: F, t1178: F, t16558: F, t1177: F, t1184: F, t460: F, t6138: F, t11556: F, t1174: F, t1187: F, t15401: F, t15405: F, t15422: F, t18321: F, t18536: F, t18543: F, t18546: F, t18550: F, t3447: F, t4889: F, t4913: F, t4931: F) -> F {
    let t18555 = t4934 * t18554;
    let t18558 = t1178 * t16558;
    let t18559 = t1177 * t18558;
    let t18563 = t6138 * t1184 * t460;
    let t18564 = t4934 * t18563;
    let t18569 = F::cast_from(0.14814814814814814815e-2_f64) * t18536 - F::cast_from(0.81481481481481481481e-2_f64) * t18321 * t1187 + F::cast_from(0.44444444444444444444e-2_f64) * t4889 * t4931 + t11556 + F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t18543 + F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t18546 + t15401 - t15405 + t15422 - F::cast_from(0.16666666666666666666e-2_f64) * t1174 * t18550 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t18555 - F::cast_from(0.27777777777777777777e-3_f64) * t1174 * t18559 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t18564 + F::cast_from(0.14814814814814814814e-2_f64) * t4889 * t4913;
    t18569
}
