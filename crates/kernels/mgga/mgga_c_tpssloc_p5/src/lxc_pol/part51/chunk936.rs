//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 936/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk936<F: Float>(t23030: F, t6643: F, t131: F, t244: F, t6612: F, t835: F, t812: F, t831: F, t2627: F, t59: F, t2617: F, t6613: F) -> (F, F, F, F, F, F, F, F) {
    let t23031 = t23030 * t6643;
    let t23032 = F::cast_from(0.26044789391763585244e-1_f64) * t23031;
    let t23033 = t244 * t131;
    let t23040 = t6612 * t835;
    let t23041 = t812 * t23040;
    let t23042 = t23041 * t831;
    let t23043 = F::new(7.0) / F::new(1152.0) * t23042;
    let t23046 = t2627 * t59;
    let t23053 = t2617 * t6613;
    (t23031, t23032, t23033, t23041, t23042, t23043, t23046, t23053)
}
