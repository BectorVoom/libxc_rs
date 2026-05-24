//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 813/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk813<F: Float>(t5542: F, t8607: F, t674: F, t2004: F, t7677: F, t8571: F, t34659: F, t34662: F, t34665: F, t38312: F, t38315: F, t38318: F, t38322: F, t38326: F, t38328: F, t38336: F, t38340: F, t38344: F, t38348: F, t38352: F, t5928: F, t7704: F) -> (F, F, F) {
    let t38354 = t8607 * t5542;
    let t38355 = t38354 * t674;
    let t38356 = t38355 * t2004;
    let t38358 = t8571 * t7677;
    let t38360 = F::cast_from(0.81300399444200075504e-3_f64) * t38312 + t38315 - F::cast_from(0.33335697577410973224e-1_f64) * t38318 + F::cast_from(0.66671395154821946448e-1_f64) * t34659 - F::cast_from(0.1951603679568577289e-3_f64) * t38322 + F::cast_from(0.30487649791575028314e-3_f64) * t38326 - F::cast_from(0.5987120850931904282e-1_f64) * t38328 - F::cast_from(0.11974241701863808564e0_f64) * t5928 * t7704 + F::cast_from(0.29810146462873361018e-2_f64) * t34662 + F::cast_from(0.29810146462873361018e-2_f64) * t34665 + F::cast_from(0.15243824895787514157e-3_f64) * t38336 + F::cast_from(0.15243824895787514157e-3_f64) * t38340 + F::cast_from(0.30487649791575028314e-3_f64) * t38344 + F::cast_from(0.15243824895787514157e-3_f64) * t38348 - F::cast_from(0.85129199786595678796e-5_f64) * t38352 - F::cast_from(0.85129199786595678796e-5_f64) * t38356 - F::cast_from(0.42564599893297839398e-5_f64) * t38358;
    (t38354, t38355, t38360)
}
