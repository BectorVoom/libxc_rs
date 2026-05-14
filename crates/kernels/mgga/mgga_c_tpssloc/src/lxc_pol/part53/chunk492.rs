//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 492/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk492<F: Float>(t300: F, t4865: F, t4833: F, t1687: F, t1166: F, t1703: F, t3411: F, t1694: F, t3375: F, t1157: F, t1164: F, t1147: F, t1156: F, t4857: F, t3400: F, t1155: F, t3403: F) -> (F, F, F, F, F, F, F, F) {
    let t4866 = t300 * t4865;
    let t4868 = 0.19751673498613801407e-1 * t300 * t4833;
    let t4869 = t300 * t1687;
    let t4871 = 0.5848223622634646207e0 * t4869 * t1166;
    let t4873 = 0.5848223622634646207e0 * t3411 * t1703;
    let t4874 = t3375 * t1694;
    let t4875 = t4874 * t1157;
    let t4877 = 0.11696447245269292414e1 * t1164 * t4875;
    let t4879 = t1147 * t4857 * t1156;
    let t4881 = 0.5848223622634646207e0 * t1164 * t4879;
    let t4882 = t3400 * t1694;
    let t4883 = t3403 * t1155;
    (t4866, t4868, t4871, t4873, t4877, t4881, t4882, t4883)
}
