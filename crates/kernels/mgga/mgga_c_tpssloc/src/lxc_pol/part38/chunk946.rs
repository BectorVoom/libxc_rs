//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 946/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk946<F: Float>(t2341: F, t92: F, t2219: F, t659: F, t2248: F, t4049: F, t584: F, t95: F, t16: F, t4053: F, t1449: F, t2350: F, t9398: F, t100: F, t2349: F, t662: F) -> (F, F, F, F, F, F, F, F) {
    let t12774 = t92 * t2341;
    let t12775 = t2219 * t659;
    let t12778 = t4049 * t2248;
    let t12781 = t95 * t584;
    let t12784 = t4053 * t16;
    let t12792 = t9398 * t1449 * t2350;
    let t12795 = t100 * t2349;
    let t12796 = t2219 * t662;
    (t12774, t12775, t12778, t12781, t12784, t12792, t12795, t12796)
}
