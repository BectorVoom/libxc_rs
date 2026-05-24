//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 611/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk611<F: Float>(t333: F, t664: F, t352: F, t305: F, t326: F, t5266: F, t7571: F, t7672: F, t7772: F, t7775: F, t7830: F, t7832: F, t7838: F, t7842: F, t7845: F, t7847: F, t7849: F, t7853: F, t7856: F, t7859: F, t7863: F, t7865: F, t7867: F, t7869: F, t7877: F, t793: F, t838: F) -> F {
    let t7879 = t664 * t333;
    let t7880 = t7879 * t352;
    let t7883 = -F::cast_from(0.6818665413561335432e-1_f64) * t7830 - F::cast_from(0.72732431077987577943e-1_f64) * t7832 - F::cast_from(0.68186654135613354322e-2_f64) * t7838 - F::cast_from(0.18183107769496894486e-1_f64) * t7842 - F::cast_from(0.20455996240684006296e-1_f64) * t7845 - F::cast_from(0.10227998120342003148e-1_f64) * t7847 + F::cast_from(0.13637330827122670864e-1_f64) * t7849 + F::cast_from(0.34093327067806677161e-2_f64) * t7853 - F::cast_from(0.14967802127329760705e-1_f64) * t7856 + F::cast_from(0.19957069503106347607e-1_f64) * t7859 + F::cast_from(0.11974241701863808564e0_f64) * t793 * t7775 + F::cast_from(0.8980681276397856423e-1_f64) * t7863 - F::cast_from(0.17961362552795712846e0_f64) * t7865 - F::cast_from(0.5987120850931904282e-1_f64) * t7867 + F::cast_from(0.17961362552795712846e0_f64) * t7869 + F::cast_from(0.23948483403727617128e0_f64) * t838 * t7772 - F::cast_from(0.59871208509319042821e-1_f64) * t326 * t7672 + F::cast_from(0.59871208509319042821e-1_f64) * t305 * t7571 + F::cast_from(0.2993560425465952141e-1_f64) * t7877 + F::cast_from(0.23948483403727617128e0_f64) * t5266 * t7880;
    t7883
}
