//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 730/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk730<F: Float>(t13807: F, t14402: F, t13810: F, t13817: F, t14415: F, t14419: F, t14552: F, t13968: F, t14557: F, t14065: F, t14079: F, t14085: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t70657 = F::cast_from(0.36366215538993788974e-1_f64) * t13807;
    let t70658 = F::new(2.0) * t14402;
    let t70659 = F::cast_from(0.60975299583150056624e-3_f64) * t13810;
    let t70661 = F::cast_from(0.32526727992809621482e-4_f64) * t13817;
    let t70667 = F::cast_from(0.30487649791575028314e-3_f64) * t14415;
    let t70668 = F::cast_from(0.43368970657079495312e-4_f64) * t14419;
    let t70679 = F::cast_from(0.18183107769496894486e-1_f64) * t14552;
    let t70680 = F::cast_from(0.32526727992809621482e-4_f64) * t13968;
    let t70681 = F::cast_from(0.81300399444200075504e-3_f64) * t14557;
    let t70705 = F::cast_from(0.58171619854173713846e-4_f64) * t14065;
    let t70707 = F::cast_from(0.10909864661698136691e0_f64) * t14079;
    let t70708 = F::cast_from(0.51300288795035171252e-6_f64) * t14085;
    (t70657, t70658, t70659, t70661, t70667, t70668, t70679, t70680, t70681, t70705, t70707, t70708)
}
