//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 870/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk870<F: Float>(t41373: F, t41377: F, t41379: F, t41381: F, t275: F, t9598: F, t1347: F, t2479: F, t1562: F, t8048: F, t2474: F, t934: F) -> (F, F, F, F, F, F, F, F) {
    let t43629 = F::cast_from(0.10643770401656718724e0_f64) * t41373;
    let t43631 = F::cast_from(0.36366215538993788972e-1_f64) * t41377;
    let t43632 = F::cast_from(0.48488287385325051964e-1_f64) * t41379;
    let t43633 = F::cast_from(0.11289648083414479539e-2_f64) * t41381;
    let t43654 = F::new(2.0) * t275 * t9598;
    let t43680 = t1347 * t2479;
    let t43722 = F::new(0.4726e1) * t1562 * t8048;
    let t43723 = t934 * t2474;
    (t43629, t43631, t43632, t43633, t43654, t43680, t43722, t43723)
}
