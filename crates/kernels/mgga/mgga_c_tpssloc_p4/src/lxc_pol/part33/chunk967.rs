//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 967/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk967<F: Float>(t10143: F, t1484: F, t16625: F, t193: F, t202: F, t20777: F, t20778: F, t20800: F, t20815: F, t21066: F, t2522: F, t4310: F, t5544: F, t766: F, t870: F, t9820: F, t9824: F, t9876: F, t9884: F, t9887: F, t9890: F) -> F {
    let t21073 = F::cast_from(2.0_f64) * t10143 * t193 * t202 * t20778 + t193 * t202 * t21066 * t870 - F::cast_from(9.0_f64) * t1484 * t16625 * t2522 + F::cast_from(3.0_f64) * t193 * t20800 * t766 + F::cast_from(9.0_f64) * t2522 * t4310 * t5544 - t20777 + t20815 - t9820 - t9824 - t9876 - t9884 + t9887 + t9890;
    t21073
}
