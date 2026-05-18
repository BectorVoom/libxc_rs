//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1018/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1018<F: Float>(t1634: F, t5943: F, t3174: F, t1052: F, t1635: F, t17575: F, t17588: F, t18074: F, t21663: F, t21669: F, t21677: F, t21682: F, t21684: F, t21689: F, t388: F, t4557: F, t4660: F, t5920: F, t5944: F) -> (F, F, F) {
    let t21691 = t1634 * t5943;
    let t21692 = t3174 * t21691;
    let t21697 = -t1052 * t21663 - F::new(6.0) * t1052 * t21677 + F::new(6.0) * t1052 * t21692 - F::new(3.0) * t1635 * t17575 - F::new(6.0) * t1635 * t17588 - F::new(3.0) * t1635 * t18074 + F::new(3.0) * t21669 * t388 + t21682 * t388 + F::new(3.0) * t21684 * t388 + t21689 * t388 + F::new(6.0) * t4557 * t5920 - F::new(3.0) * t4557 * t5944 + F::new(6.0) * t4660 * t5920 - F::new(3.0) * t4660 * t5944;
    (t21691, t21692, t21697)
}
