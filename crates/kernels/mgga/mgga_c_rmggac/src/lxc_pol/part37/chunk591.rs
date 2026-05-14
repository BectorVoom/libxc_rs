//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 591/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk591<F: Float>(t1411: F, t7754: F, t1540: F, t880: F, t49: F, t529: F, t2410: F, t7228: F, t3350: F, t623: F, t7191: F, t1679: F, t7203: F, t16501: F, t7363: F, t1966: F) -> (F, F, F, F, F, F, F, F) {
    let t38855 = t7754 * t1411;
    let t38973 = t1540 * t880;
    let t39116 = t49 * t529;
    let t39207 = t2410 * t7228;
    let t39277 = t39207 * t3350;
    let t39570 = t623 * t7191;
    let t39705 = t1679 * t7203;
    let t39850 = t7363 * t16501;
    let t39851 = t1966 * t39850;
    (t38855, t38973, t39116, t39207, t39277, t39570, t39705, t39851)
}
