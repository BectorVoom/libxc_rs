//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 842/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk842<F: Float>(t333: F, t5266: F, t77960: F, t558: F, t71916: F, t2367: F, t698: F, t352: F, t8940: F, t14444: F, t1652: F, t71910: F, t8264: F, t118: F, t76242: F, t27055: F, t77335: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t77966 = 0.11974241701863808564e0 * t5266 * t77960 * t333;
    let t77969 = 0.11974241701863808564e0 * t5266 * t71916 * t558;
    let t77970 = t698 * t2367;
    let t77973 = 0.11974241701863808564e0 * t8940 * t77970 * t352;
    let t77976 = 0.11974241701863808564e0 * t8940 * t14444 * t1652;
    let t77979 = 0.11974241701863808564e0 * t5266 * t71910 * t558;
    let t77980 = t8264 * t2367;
    let t77982 = 0.39914139006212695214e-1 * t118 * t77980;
    let t77983 = 0.68186654135613354325e-2 * t76242;
    let t77988 = 0.35922725105591425692e0 * t27055 * t77335;
    (t77966, t77969, t77970, t77973, t77976, t77979, t77980, t77982, t77983, t77988)
}
