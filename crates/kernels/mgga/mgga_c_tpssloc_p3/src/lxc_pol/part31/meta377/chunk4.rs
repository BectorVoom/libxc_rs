//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1332/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1332<F: Float>(t16610: F, t16666: F, t17119: F, t17131: F, t1534: F, t2: F, t584: F, t5678: F, t690: F) -> (F, F, F) {
    let t17133 = t16610 + t16666 + t17119 + t17131;
    let t17139 = t1534 * t2;
    let t17141 = F::new(2.0) * t17139 * t584;
    let t17149 = t690 * t5678;
    (t17133, t17141, t17149)
}
