//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1136/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1136<F: Float>(t15626: F, t2911: F, t5081: F, t9495: F, t1042: F, t9493: F, t4192: F, t4198: F, t4181: F, t4197: F, t1089: F, t3009: F, t5191: F) -> (F, F, F, F, F) {
    let t15628 = F::new(0.32163958997385070134e2) * t2911 * t15626;
    let t15629 = t5081 * t9495;
    let t15630 = t15629 * t1042;
    let t15632 = F::new(0.51726012919273400301e3) * t9493 * t15630;
    let t15634 = F::new(0.23392894490538584828e1) * t4192 * t4198;
    let t15635 = t4197 * t4181;
    let t15637 = F::new(0.23392894490538584828e1) * t1089 * t15635;
    let t15639 = F::new(0.11696447245269292414e1) * t3009 * t5191;
    (t15628, t15632, t15634, t15637, t15639)
}
