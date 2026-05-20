//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1169/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1169<F: Float>(t31130: F, t3887: F, t1375: F, t31094: F, t31096: F, t31103: F, t31106: F, t31111: F, t31113: F, t31115: F, t31117: F, t31122: F, t31126: F, t31129: F, t6958: F, t6993: F) -> (F, F) {
    let t31131 = t3887 * t31130;
    let t31136 = F::new(4.0) * t1375 * t31096 - F::new(6.0) * t1375 * t31117 + F::new(2.0) * t1375 * t31131 - F::new(2.0) * t6958 * t6993 + t31094 + t31103 - t31106 + t31111 - t31113 + t31115 - t31122 - t31126 + t31129;
    (t31131, t31136)
}
