//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 749/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk749<F: Float>(t210: F, t214: F, t5527: F, t5544: F, t2562: F, t2569: F, t2571: F, t2590: F, t4124: F, t4135: F, t787: F, t252: F, t1492: F, t1519: F, t119: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5550 = t210 * t214 * t5527;
    let t5555 = t210 * t214 * t5544;
    let t5558 = t2562 + 0.77777777777777777775e-2 * t4124 + t2569 + 0.49999999999999999998e-2 * t2571 * t5550 + 0.16666666666666666666e-2 * t4135 - 0.16666666666666666666e-2 * t787 * t5555 - t2590;
    let t5559 = t5558 * t252;
    let t5561 = t1492 * t1519;
    let t5567 = t119 * t5527;
    let t5568 = t210 * t5567;
    let t5571 = t119 * t5544;
    let t5572 = t210 * t5571;
    let t5575 = t5558 * t225;
    (t5550, t5555, t5558, t5559, t5561, t5567, t5568, t5571, t5572, t5575)
}
