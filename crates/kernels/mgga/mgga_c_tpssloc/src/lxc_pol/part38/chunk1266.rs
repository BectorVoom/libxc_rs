//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1266/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1266<F: Float>(t1799: F, t212: F, t1307: F, t686: F, t16094: F, t12214: F, t131: F, t205: F, t221: F, t3734: F, t5196: F, t3726: F, t5206: F) -> (F, F, F, F, F) {
    let t16095 = t212 * t1799;
    let t16097 = t686 * t16095 * t1307;
    let t16099 = F::new(0.49999999999999999998e-2) * t16094 * t16097;
    let t16100 = t12214 * t131;
    let t16101 = t205 * t16100;
    let t16103 = t221 * t5196 * t3734;
    let t16106 = t3726 * t5206;
    (t16095, t16099, t16101, t16103, t16106)
}
