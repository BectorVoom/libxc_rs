//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 730/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk730<F: Float>(t34760: F, t7716: F, t20: F, t2018: F, t2021: F, t4747: F, t7345: F, t7766: F, t7344: F, t7552: F, t7558: F, t131: F, t1341: F) -> (F, F, F, F, F) {
    let t34764 = t7716 * t34760;
    let t34772 = t4747 * t20 * t2018 * t2021;
    let t34773 = F::cast_from(0.15243824895787514157e-3_f64) * t34772;
    let t34784 = t7345 * t7766;
    let t34785 = F::cast_from(0.45731474687362542471e-3_f64) * t34784;
    let t34786 = t7344 * t7552;
    let t34787 = t34786 * t7558;
    let t34788 = F::cast_from(0.65053455985619242968e-4_f64) * t34787;
    let t34790 = t131 * t1341;
    (t34764, t34773, t34785, t34788, t34790)
}
