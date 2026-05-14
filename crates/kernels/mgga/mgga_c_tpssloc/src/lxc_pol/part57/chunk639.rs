//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 639/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk639<F: Float>(t23030: F, t6643: F, t131: F, t244: F, t209: F, t1878: F) -> (F, F, F) {
    let t23031 = t23030 * t6643;
    let t23032 = 0.26044789391763585244e-1 * t23031;
    let t23033 = t244 * t131;
    let t23034 = t23033 * t209;
    let t23035 = t1878 * t23034;
    (t23031, t23032, t23035)
}
