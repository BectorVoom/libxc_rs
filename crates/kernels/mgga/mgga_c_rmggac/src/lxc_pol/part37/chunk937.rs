//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 937/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk937<F: Float>(t3281: F, t558: F, t352: F, t5266: F, t69181: F, t76212: F, t76216: F, t76218: F, t76222: F, t76224: F, t77907: F, t77908: F, t77911: F, t77917: F, t77920: F, t77921: F) -> (F, F) {
    let t80429 = t3281 * t558;
    let t80433 = t77907 + t77908 - t77911 + t77917 - t77920 + t77921 + t76212 - t76216 - t76218 + 0.11974241701863808564e0 * t5266 * t80429 * t352 - t76222 - t76224 - t69181;
    (t80429, t80433)
}
