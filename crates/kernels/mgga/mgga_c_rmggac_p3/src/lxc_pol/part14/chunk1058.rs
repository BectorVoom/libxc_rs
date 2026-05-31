//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1058/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1058<F: Float>(t8876: F, t942: F, t4961: F, t668: F, t1971: F, t3351: F, t5194: F, t880: F, t235: F, t2379: F, t26093: F, t289: F, t36748: F, t36753: F, t36754: F, t36756: F, t36758: F, t36797: F, t36802: F, t36804: F, t36806: F, t36809: F, t36811: F, t36814: F, t41386: F, t515: F) -> F {
    let t41929 = F::cast_from(0.4726e1_f64) * t942 * t8876;
    let t41932 = t4961 * t668;
    let t41949 = t3351 * t1971 * t880 * t5194;
    let t41951 = -t41929 + F::cast_from(0.59871208509319042821e-1_f64) * t26093 * t2379 - F::cast_from(0.4726e1_f64) * t289 * t41932 - F::cast_from(0.30487649791575028314e-3_f64) * t36748 - t36753 - F::cast_from(0.30487649791575028314e-3_f64) * t36754 + F::cast_from(0.60975299583150056628e-3_f64) * t36756 + F::cast_from(0.96056421943322389208e-3_f64) * t36758 - t36797 + t36802 + F::cast_from(0.16260079888840015101e-2_f64) * t36804 + F::cast_from(0.19211284388664477842e-2_f64) * t36806 + F::cast_from(0.16260079888840015101e-2_f64) * t36809 + F::cast_from(0.19211284388664477842e-2_f64) * t36811 - F::cast_from(0.15243824895787514157e-3_f64) * t36814 - F::cast_from(0.19957069503106347607e-1_f64) * t235 * t515 * t41386 + F::cast_from(0.10215503974391481455e-3_f64) * t41949;
    t41951
}
