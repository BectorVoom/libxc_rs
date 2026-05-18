//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1173/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1173<F: Float>(t11593: F, t4904: F, t11570: F, t3961: F, t11569: F, t1174: F, t15332: F, t15335: F, t15341: F, t15360: F, t15364: F, t15366: F, t15374: F, t15376: F, t3447: F, t3452: F, t3472: F, t3478: F, t4889: F) -> F {
    let t15379 = t11593 * t4904;
    let t15382 = t11570 * t3961;
    let t15383 = t11569 * t15382;
    let t15386 = -F::new(0.55555555555555555554e-3) * t3447 * t15332 - F::new(0.16666666666666666666e-2) * t3447 * t15335 + t15341 - F::new(0.83333333333333333332e-3) * t1174 * t15360 + F::new(0.18518518518518518518e-3) * t15364 + F::new(0.14814814814814814815e-2) * t15366 + F::new(0.22222222222222222222e-2) * t4889 * t3472 + F::new(0.22222222222222222222e-2) * t4889 * t3478 - t15374 - F::new(0.14814814814814814815e-2) * t15376 * t3452 + F::new(0.27777777777777777777e-3) * t3447 * t15379 - F::new(0.74074074074074074072e-3) * t3447 * t15383;
    t15386
}
