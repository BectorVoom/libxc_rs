//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 982/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk982<F: Float>(t2085: F, t212: F, t22642: F, t6890: F, t214: F, t7191: F, t6888: F, t6891: F, t22916: F, t31611: F, t22751: F, t31645: F) -> (F, F, F, F, F) {
    let t115330 = t22642 * t212 * t2085 * t6890;
    let t115331 = F::new(0.82246703342411321824e-2) * t115330;
    let t115332 = t214 * t7191;
    let t115334 = t6888 * t115332 * t6891;
    let t115337 = t6888 * t31611 * t22916;
    let t115339 = t22751 * t31645;
    (t115331, t115332, t115334, t115337, t115339)
}
