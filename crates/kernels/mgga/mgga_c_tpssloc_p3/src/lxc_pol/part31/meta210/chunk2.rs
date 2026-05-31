//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 935/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk935<F: Float>(t1528: F, t259: F, t4147: F, t4268: F, t5559: F, t5561: F, t5632: F, t5637: F, t5658: F, t855: F) -> F {
    let t5660 = -F::cast_from(2.0_f64) * t1528 * t4147 - F::cast_from(2.0_f64) * t1528 * t4268 + t259 * t5559 + F::cast_from(2.0_f64) * t259 * t5561 + t259 * t5632 + F::cast_from(2.0_f64) * t5637 * t855 - t5658 * t855;
    t5660
}
