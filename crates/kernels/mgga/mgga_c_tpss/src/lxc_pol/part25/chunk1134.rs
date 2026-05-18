//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1134/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1134<F: Float>(t3009: F, t5199: F, t2973: F, t5177: F, t1082: F, t1089: F, t12244: F, t4068: F, t11976: F, t4109: F, t1042: F, t5082: F) -> (F, F, F, F, F) {
    let t15601 = F::new(0.17315859105681463759e2) * t3009 * t5199;
    let t15602 = t2973 * t5177;
    let t15603 = t15602 * t1082;
    let t15605 = F::new(0.11696447245269292414e1) * t1089 * t15603;
    let t15607 = F::new(4.0) * t12244 * t4068;
    let t15609 = F::new(0.32163958997385070134e2) * t11976 * t4109;
    let t15610 = t5082 * t1042;
    (t15601, t15605, t15607, t15609, t15610)
}
