//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 624/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk624<F: Float>(t15672: F, t884: F, t8041: F, t8936: F, t1356: F, t15030: F, t15033: F, t15068: F, t15082: F, t15199: F, t3282: F, t551: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15673 = t884 * t15672;
    let t15674 = F::cast_from(0.11974241701863808564e0_f64) * t15673;
    let t15675 = t8041 * t8936;
    let t15676 = t1356 * t15675;
    let t15677 = F::cast_from(0.11974241701863808564e0_f64) * t15676;
    let t15856 = F::cast_from(0.32526727992809621482e-5_f64) * t15030;
    let t15857 = F::cast_from(0.32526727992809621482e-5_f64) * t15033;
    let t15858 = F::cast_from(0.17519306092901367186e-5_f64) * t15068;
    let t15859 = F::cast_from(0.76860658247009135562e-5_f64) * t15082;
    let t15861 = F::cast_from(0.31062809106223861414e-2_f64) * t15199;
    let t15862 = t3282 * t551;
    (t15674, t15675, t15677, t15856, t15857, t15858, t15859, t15861, t15862)
}
