//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 395/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk395<F: Float>(t201: F, t8687: F, t1614: F, t36: F, t262: F, t2103: F, t1587: F, t2115: F, t265: F, t551: F, t558: F, t2118: F, t2100: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8688 = t8687 * t201;
    let t8700 = t36 * t1614;
    let t8701 = t262 * t8700;
    let t8702 = t2103 * t8701;
    let t8704 = t36 * t1587;
    let t8705 = t262 * t8704;
    let t8706 = t2115 * t8705;
    let t8708 = t265 * t551;
    let t8709 = t262 * t8708;
    let t8710 = t2115 * t8709;
    let t8712 = t265 * t558;
    let t8713 = t262 * t8712;
    let t8714 = t2118 * t8713;
    let t8716 = t2100 * t8709;
    let t8718 = t2103 * t8713;
    let t8720 = t2118 * t8701;
    (t8688, t8700, t8701, t8702, t8704, t8705, t8706, t8708, t8709, t8710, t8712, t8713, t8714, t8716, t8718, t8720)
}
