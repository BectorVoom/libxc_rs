//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1047/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1047<F: Float>(t42085: F, t8443: F, t118: F, t1986: F, t571: F, t615: F, t7717: F, t117: F, t33235: F, t2295: F, t31057: F, t46391: F) -> (F, F, F, F) {
    let t47821 = t42085 * t8443;
    let t47825 = t1986 * t118 * t571 * t615;
    let t47826 = t7717 * t47825;
    let t47830 = t33235 * t117;
    let t47831 = t47830 * t2295;
    let t47833 = t31057 * t46391;
    (t47821, t47826, t47831, t47833)
}
