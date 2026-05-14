//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 574/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk574<F: Float>(t4882: F, t4883: F, t1164: F, t1171: F, t1706: F, t1420: F, t972: F, t1709: F, t3431: F, t1174: F, t3439: F, t60: F, t461: F, t4724: F, t1409: F, t3450: F) -> (F, F, F, F, F, F, F) {
    let t4884 = t4882 * t4883;
    let t4886 = 0.17315859105681463759e2 * t1164 * t4884;
    let t4887 = t1706 * t1171;
    let t4889 = t1420 * t972;
    let t4896 = t3431 * t1709;
    let t4897 = t1174 * t4896;
    let t4899 = t60 * t3439;
    let t4900 = t4899 * t461;
    let t4901 = t4900 * t4724;
    let t4904 = t3450 * t1409;
    (t4886, t4887, t4889, t4897, t4899, t4901, t4904)
}
