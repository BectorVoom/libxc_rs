//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1056/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1056<F: Float>(t14704: F, t1089: F, t12606: F, t1088: F, t123: F) -> (F, F, F) {
    let t14705 = 0.20128333333333333334e0 * t14704;
    let t14706 = t1089 * t12606;
    let t14707 = t1088 * t14706;
    let t14708 = t123 * t14707;
    (t14705, t14706, t14708)
}
