//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 532/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk532<F: Float>(t1019: F, t4640: F, t1040: F, t1611: F, t1626: F, t225: F, t1057: F, t4639: F, t193: F, t336: F) -> (F, F, F, F, F) {
    let t4641 = t4640 * t1019;
    let t4644 = t1611 * t1040;
    let t4660 = t1626 * t225;
    let t4669 = t4639 * t1057;
    let t4700 = t193 * t336;
    (t4641, t4644, t4660, t4669, t4700)
}
