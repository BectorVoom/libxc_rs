//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 577/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk577<F: Float>(t14319: F, t14324: F, t14505: F, t14508: F, t14511: F, t14514: F, t14518: F, t14519: F, t14520: F, t15000: F, t15002: F, t15012: F) -> F {
    let t15014 = t15000 - t14505 + t14508 - t14319 + t14324 - t14511 - t14514 + t14518 + F::new(0.19957069503106347607e-1) * t15002 + t14519 - t14520 + t15012;
    t15014
}
