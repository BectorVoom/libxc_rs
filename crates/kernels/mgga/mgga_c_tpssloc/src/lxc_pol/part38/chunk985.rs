//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 985/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk985<F: Float>(t11135: F, t11203: F, t135: F, t3477: F, t1174: F, t1176: F, t698: F, t1179: F, t3431: F, t3460: F, t3456: F, t3439: F) -> (F, F, F, F, F, F, F, F) {
    let t11459 = F::new(0.55403703703703703703e-1) * t11135;
    let t11487 = F::new(20.0) / F::new(27.0) * t11203;
    let t11513 = t135 * t3477;
    let t11514 = t1174 * t11513;
    let t11529 = t698 * t1176;
    let t11530 = t11529 * t1179;
    let t11531 = t1174 * t11530;
    let t11533 = t3431 * t3460;
    let t11534 = t1174 * t11533;
    let t11536 = t3431 * t3456;
    let t11537 = t1174 * t11536;
    let t11539 = t135 * t3439;
    (t11459, t11487, t11514, t11529, t11531, t11534, t11537, t11539)
}
