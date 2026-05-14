//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 888/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk888<F: Float>(t68540: F, t68543: F, t68550: F, t74321: F, t74378: F, t76968: F, t76969: F, t76970: F, t76972: F, t76973: F, t76974: F, t76975: F, t76976: F, t76977: F, t76978: F, t76979: F, t76980: F) -> (F,) {
    let t80084 = 0.31062809106223861414e-2 * t74321 - t76968 + t76969 - t76970 - t76972 + t76973 + t76974 + t68540 - t68543 + t76975 + t76976 - t76977 + t68550 + t76978 - t76979 - t76980 - 0.17519306092901367186e-5 * t74378;
    (t80084,)
}
