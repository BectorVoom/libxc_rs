//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 870/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk870<F: Float>(t2447: F, t664: F, t321: F, t5148: F, t333: F, t5266: F, t558: F, t71916: F, t2367: F, t698: F, t352: F, t8940: F, t14444: F, t1652: F, t71910: F, t8264: F) -> (F, F, F, F, F, F, F, F, F) {
    let t77960 = t2447 * t664;
    let t77963 = 0.11974241701863808564e0 * t5148 * t77960 * t321;
    let t77966 = 0.11974241701863808564e0 * t5266 * t77960 * t333;
    let t77969 = 0.11974241701863808564e0 * t5266 * t71916 * t558;
    let t77970 = t698 * t2367;
    let t77973 = 0.11974241701863808564e0 * t8940 * t77970 * t352;
    let t77976 = 0.11974241701863808564e0 * t8940 * t14444 * t1652;
    let t77979 = 0.11974241701863808564e0 * t5266 * t71910 * t558;
    let t77980 = t8264 * t2367;
    (t77960, t77963, t77966, t77969, t77970, t77973, t77976, t77979, t77980)
}
