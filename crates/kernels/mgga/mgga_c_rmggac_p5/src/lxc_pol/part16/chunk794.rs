//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 794/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk794<F: Float>(t270: F, t574: F, t290: F, t2010: F, t7755: F, t1664: F, t7556: F, t2012: F, t7349: F, t2019: F, t640: F, t7764: F) -> (F, F, F, F, F) {
    let t38815 = t574 * t270;
    let t38816 = t290 * t38815;
    let t38818 = t2010 * t7755 * t38816;
    let t38820 = t1664 * t7556;
    let t38822 = t7349 * t2012 * t38820;
    let t38826 = t2019 * t7764 * t640 * t38815;
    (t38816, t38818, t38820, t38822, t38826)
}
