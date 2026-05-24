//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1019/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1019<F: Float>(t46525: F, t4669: F, t1652: F, t40940: F, t41532: F, t41535: F, t41537: F, t41550: F, t46765: F, t46770: F, t46772: F, t46774: F, t46779: F, t46782: F, t5266: F, t570: F, t793: F, t8946: F) -> F {
    let t46784 = t4669 * t46525;
    let t46786 = t41532 - t41535 - t41537 + F::cast_from(0.39914139006212695213e-1_f64) * t46765 + F::cast_from(0.23948483403727617128e0_f64) * t5266 * t40940 * t570 - F::cast_from(0.8980681276397856423e-1_f64) * t46770 + F::cast_from(0.17961362552795712846e0_f64) * t46772 + F::cast_from(0.44903406381989282115e-1_f64) * t46774 + F::cast_from(0.23948483403727617128e0_f64) * t5266 * t8946 * t1652 + F::cast_from(0.11974241701863808564e0_f64) * t793 * t46779 - t41550 - F::cast_from(0.8980681276397856423e-1_f64) * t46782 + F::cast_from(0.8980681276397856423e-1_f64) * t46784;
    t46786
}
