//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 739/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk739<F: Float>(t29075: F, t29104: F, t870: F, t1408: F, t1877: F, t2057: F, t24191: F, t24344: F, t25: F, t2522: F, t26744: F, t28249: F, t28252: F, t28256: F, t28456: F, t28459: F, t28462: F, t28972: F, t4314: F, t5397: F, t7114: F, t7475: F, t7545: F, t7845: F) -> (F, F, F) {
    let t29105 = t29075 + t29104;
    let t29106 = t29105 * t870;
    let t29124 = 3.0 * t4314 * t28972 + 3.0 * t2522 * t7845 * t7475 - 3.0 * t24191 * t28249 + 3.0 * t2522 * t2057 * t28252 + 3.0 / 2.0 * t2522 * t2057 * t28256 + t1877 * t29106 * t25 / 2.0 - t1877 * t26744 * t7545 + t1877 * t7845 * t1408 + t1877 * t24344 * t28456 - t1877 * t7114 * t28459 - t1877 * t7114 * t28462 / 2.0 + t1877 * t2057 * t5397 / 2.0;
    (t29105, t29106, t29124)
}
