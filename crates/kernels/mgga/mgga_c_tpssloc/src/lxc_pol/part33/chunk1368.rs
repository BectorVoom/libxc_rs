//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1368/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1368<F: Float>(t106731: F, t1874: F, t20347: F, t89: F, t28030: F, t7461: F, t20563: F, t24995: F, t8945: F, t1983: F, t28238: F, t5161: F) -> (F, F, F, F, F) {
    let t106733 = F::new(6.0) * t106731 * t1874;
    let t106734 = t89 * t20347;
    let t106736 = F::new(2.0) * t106734 * t1874;
    let t106738 = F::new(6.0) * t28030 * t7461;
    let t106741 = F::new(18.0) * t24995 * t8945 * t20563;
    let t106744 = F::new(3.0) * t1983 * t28238 * t5161;
    (t106733, t106736, t106738, t106741, t106744)
}
