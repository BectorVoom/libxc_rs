//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1053/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1053<F: Float>(t15357: F, t457: F, t460: F, t974: F, t1716: F, t698: F, t1174: F, t3435: F, t4889: F, t135: F, t4930: F, t1420: F, t1887: F, t337: F, t11593: F, t4904: F) -> (F, F, F, F, F, F) {
    let t15359 = t457 * t15357 * t460;
    let t15360 = t974 * t15359;
    let t15363 = t698 * t1716;
    let t15364 = t1174 * t15363;
    let t15366 = t4889 * t3435;
    let t15372 = t135 * t4930;
    let t15374 = 0.55555555555555555554e-3 * t1174 * t15372;
    let t15376 = t1420 * t337 * t1887;
    let t15379 = t11593 * t4904;
    (t15360, t15364, t15366, t15374, t15376, t15379)
}
