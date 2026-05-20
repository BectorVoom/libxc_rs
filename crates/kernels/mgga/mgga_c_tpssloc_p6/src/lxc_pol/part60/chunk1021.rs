//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1021/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1021<F: Float>(t652: F, t7467: F, t7890: F, t33214: F, t7802: F, t29211: F, t8526: F, t115262: F, t1983: F, t28826: F, t120955: F, t7687: F) -> (F, F, F, F, F) {
    let t128418 = F::new(4.0) * t652 * t7890 * t7467;
    let t128420 = F::new(4.0) * t33214 * t7802;
    let t128422 = F::new(2.0) * t8526 * t29211;
    let t128429 = F::new(6.0) * t1983 * t115262 * t28826;
    let t128438 = F::new(6.0) * t1983 * t120955 * t7687;
    (t128418, t128420, t128422, t128429, t128438)
}
