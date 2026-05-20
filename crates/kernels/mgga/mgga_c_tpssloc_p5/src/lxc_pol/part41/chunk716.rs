//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 716/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk716<F: Float>(t1671: F, t3259: F, t1117: F, t3264: F, t1661: F, t3270: F, t1102: F, t3238: F, t3274: F, t4721: F, t4726: F, t4731: F, t4735: F) -> (F, F, F, F, F, F) {
    let t4744 = F::new(1.0) * t3259 * t1671;
    let t4745 = t1671 * t1117;
    let t4747 = F::new(2.0) * t3264 * t4745;
    let t4748 = t3270 * t1661;
    let t4749 = t4748 * t1102;
    let t4756 = t3274 - t3238 / F::new(9.0) - t4721 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t4726 + F::new(2.0) / F::new(3.0) * t4731 + t4735 / F::new(3.0);
    (t4744, t4745, t4747, t4748, t4749, t4756)
}
