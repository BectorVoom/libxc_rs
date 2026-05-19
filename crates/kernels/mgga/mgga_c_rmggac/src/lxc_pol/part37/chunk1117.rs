//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1117/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1117<F: Float>(t14953: F, t1562: F, t14977: F, t15865: F, t4041: F, t4985: F, t71804: F, t76103: F, t76108: F, t78486: F, t78487: F, t78488: F, t78491: F, t78493: F, t78495: F, t78497: F, t78498: F, t78499: F, t78500: F, t78501: F) -> F {
    let t80517 = t1562 * t14953;
    let t80521 = -t78486 + t78487 + F::cast_from(0.59871208509319042821e-1_f64) * t4985 * t14977 - t71804 - t78488 - F::cast_from(0.58171619854173713844e-5_f64) * t76103 - F::cast_from(0.21814357445315142691e-4_f64) * t76108 - t78491 - F::new(0.2363e1) * t80517 + t78493 - t78495 - t78497 + F::cast_from(0.59871208509319042821e-1_f64) * t4041 * t15865 + t78498 + t78499 + t78500 + t78501;
    t80521
}
