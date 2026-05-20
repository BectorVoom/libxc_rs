//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1038/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1038<F: Float>(t12083: F, t182: F, t1294: F, t9722: F, t172: F, t3681: F, t763: F, t2528: F, t3691: F, t9919: F, t12051: F, t12053: F, t12055: F, t12057: F, t12059: F, t9789: F, t9793: F, t9797: F) -> (F, F, F, F, F, F, F) {
    let t12085 = F::cast_from(0.19751673498613801407e-1_f64) * t12083 * t182;
    let t12087 = F::cast_from(0.10389515463408878255e3_f64) * t1294 * t9722;
    let t12088 = t3681 * t172;
    let t12089 = t12088 * t763;
    let t12090 = F::cast_from(0.17544670867903938621e1_f64) * t12089;
    let t12091 = t3691 * t2528;
    let t12092 = F::cast_from(0.51947577317044391276e2_f64) * t12091;
    let t12094 = F::cast_from(0.35089341735807877242e1_f64) * t1294 * t9919;
    let t12095 = t12051 + t12053 + t12055 - t12057 - t12059 + t12085 - t9789 + t12087 - t12090 - t12092 - t12094 + t9793 + t9797;
    (t12085, t12087, t12088, t12090, t12092, t12094, t12095)
}
