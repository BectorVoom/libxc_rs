//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1081/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1081<F: Float>(t8852: F, t8856: F, t8860: F, t8864: F, t8867: F, t8870: F, t8874: F, t8877: F, t8879: F, t8888: F, t9032: F, t9033: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42289 = F::cast_from(0.30487649791575028314e-3_f64) * t8852;
    let t42290 = F::cast_from(0.30487649791575028314e-3_f64) * t8856;
    let t42291 = F::cast_from(0.30487649791575028314e-3_f64) * t8860;
    let t42292 = F::cast_from(0.30487649791575028314e-3_f64) * t8864;
    let t42293 = F::cast_from(0.11974241701863808564e0_f64) * t8867;
    let t42294 = F::new(2.0) * t8870;
    let t42296 = F::cast_from(0.79828278012425390428e-1_f64) * t8874;
    let t42297 = F::new(0.4726e1) * t8877;
    let t42298 = F::cast_from(0.11974241701863808564e0_f64) * t8879;
    let t42299 = F::new(2.0) * t8888;
    let t42300 = F::new(2.0) * t9032;
    let t42301 = F::cast_from(0.11974241701863808564e0_f64) * t9033;
    (t42289, t42290, t42291, t42292, t42293, t42294, t42296, t42297, t42298, t42299, t42300, t42301)
}
