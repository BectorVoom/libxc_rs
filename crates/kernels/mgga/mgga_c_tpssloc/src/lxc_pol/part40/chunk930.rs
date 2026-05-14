//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 930/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk930<F: Float>(t16094: F, t16097: F, t12214: F, t131: F, t205: F, t3726: F, t5206: F, t12199: F, t5202: F, t118: F, t5187: F, t794: F, t3739: F, t12225: F, t16095: F, t2586: F) -> (F, F, F, F, F, F) {
    let t16099 = 0.49999999999999999998e-2 * t16094 * t16097;
    let t16100 = t12214 * t131;
    let t16101 = t205 * t16100;
    let t16106 = t3726 * t5206;
    let t16108 = t12199 * t5202;
    let t16111 = t118 * t794 * t5187;
    let t16113 = 0.16666666666666666666e-2 * t3739 * t16111;
    let t16118 = t12225 * t16095;
    let t16119 = t2586 * t16118;
    (t16099, t16101, t16106, t16108, t16113, t16119)
}
