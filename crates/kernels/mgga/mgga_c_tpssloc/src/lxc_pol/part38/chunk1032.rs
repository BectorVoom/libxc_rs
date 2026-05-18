//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1032/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1032<F: Float>(t584: F, t95: F, t16: F, t4053: F, t1449: F, t2350: F, t9398: F, t100: F, t2349: F, t2219: F, t662: F, t2354: F, t4059: F) -> (F, F, F, F, F, F) {
    let t12781 = t95 * t584;
    let t12784 = t4053 * t16;
    let t12792 = t9398 * t1449 * t2350;
    let t12795 = t100 * t2349;
    let t12796 = t2219 * t662;
    let t12799 = t4059 * t2354;
    (t12781, t12784, t12792, t12795, t12796, t12799)
}
