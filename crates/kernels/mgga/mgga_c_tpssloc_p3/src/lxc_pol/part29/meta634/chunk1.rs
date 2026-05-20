//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2083/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2083<F: Float>(t25372: F, t86732: F, t193: F, t201: F, t7540: F, t200: F, t6665: F, t4303: F, t606: F, t1877: F, t1915: F, t9212: F) -> (F, F, F, F, F) {
    let t86734 = F::new(2.0) * t25372 * t86732;
    let t86736 = t193 * t201 * t7540;
    let t86740 = t193 * t200 * t6665;
    let t86746 = t606 * t4303;
    let t86751 = F::new(3.0) * t1877 * t1915 * t9212;
    (t86734, t86736, t86740, t86746, t86751)
}
