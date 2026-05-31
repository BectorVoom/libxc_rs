//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1724/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1724<F: Float>(t2057: F, t5527: F, t1484: F, t1530: F, t1877: F, t193: F, t202: F, t24344: F, t2522: F, t26744: F, t28248: F, t29105: F, t4314: F, t5544: F, t5660: F, t5664: F, t7114: F, t7845: F, t870: F) -> (F, F) {
    let t29125 = t2057 * t5527;
    let t29148 = t193 * t202 * t29105 * t870 + F::cast_from(6.0_f64) * t1484 * t2522 * t7845 - F::cast_from(2.0_f64) * t1530 * t1877 * t26744 + F::cast_from(2.0_f64) * t1877 * t24344 * t5664 - t1877 * t5660 * t7114 + F::cast_from(3.0_f64) * t2057 * t2522 * t5544 - F::cast_from(6.0_f64) * t2522 * t28248 * t7114 + F::cast_from(6.0_f64) * t29125 * t4314;
    (t29125, t29148)
}
