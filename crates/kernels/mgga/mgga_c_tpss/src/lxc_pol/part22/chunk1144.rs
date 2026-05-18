//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1144/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1144<F: Float>(t9959: F, t9961: F, t9963: F, t9966: F, t2345: F, t4438: F, t4397: F, t541: F, t3234: F, t4533: F, t177: F, t4377: F) -> (F, F, F, F, F, F, F, F) {
    let t12754 = F::new(12.0) * t9959;
    let t12755 = F::new(4.0) * t9961;
    let t12756 = F::new(4.0) * t9963;
    let t12757 = F::new(80.0) * t9966;
    let t12758 = t4438 * t2345;
    let t12759 = F::new(0.11696447245269292414e1) * t12758;
    let t12760 = t541 * t4397;
    let t12764 = t4533 * t3234;
    let t12767 = t4377 * t177;
    (t12754, t12755, t12756, t12757, t12759, t12760, t12764, t12767)
}
