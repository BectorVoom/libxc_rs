//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 655/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk655<F: Float>(t71581: F, t14572: F, t7487: F, t14559: F, t2019: F, t2020: F, t14696: F, t7345: F, t14588: F, t504: F, t70018: F, t70021: F, t14469: F, t2604: F, t14589: F, t7269: F) -> (F, F, F, F, F, F, F, F, F) {
    let t71582 = 0.81300399444200075504e-3 * t71581;
    let t71583 = t7487 * t14572;
    let t71594 = t2019 * t2020 * t14559;
    let t71606 = t7345 * t14696;
    let t71607 = 0.15243824895787514157e-3 * t71606;
    let t71608 = t504 * t14588;
    let t71619 = 0.50557909407869413937e0 * t70018;
    let t71620 = 0.17347588262831798124e-3 * t70021;
    let t71628 = t2604 * t14469;
    let t71630 = t14589 * t7269;
    (t71582, t71583, t71594, t71607, t71608, t71619, t71620, t71628, t71630)
}
