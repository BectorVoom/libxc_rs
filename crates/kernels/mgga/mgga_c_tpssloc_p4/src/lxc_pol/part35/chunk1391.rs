//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1391/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1391<F: Float>(t1983: F, t28238: F, t5161: F, t19596: F, t7753: F, t28817: F, t7685: F, t191: F, t192: F, t20350: F, t2020: F, t5445: F, t72: F, t7431: F) -> (F, F, F, F, F) {
    let t106744 = F::new(3.0) * t1983 * t28238 * t5161;
    let t106747 = F::new(3.0) * t1983 * t7753 * t19596;
    let t106753 = F::new(18.0) * t7685 * t28817;
    let t106755 = t20350 * t191 * t192;
    let t106756 = t106755 * t2020;
    let t106758 = t72 * t7431 * t5445;
    (t106744, t106747, t106753, t106756, t106758)
}
