//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1038/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1038<F: Float>(t14444: F, t1652: F, t8940: F, t5266: F, t558: F, t71910: F, t2367: F, t8264: F, t118: F, t76242: F, t27055: F, t77335: F) -> (F, F, F, F, F, F) {
    let t77976 = F::new(0.11974241701863808564e0) * t8940 * t14444 * t1652;
    let t77979 = F::new(0.11974241701863808564e0) * t5266 * t71910 * t558;
    let t77980 = t8264 * t2367;
    let t77982 = F::new(0.39914139006212695214e-1) * t118 * t77980;
    let t77983 = F::new(0.68186654135613354325e-2) * t76242;
    let t77988 = F::new(0.35922725105591425692e0) * t27055 * t77335;
    (t77976, t77979, t77980, t77982, t77983, t77988)
}
