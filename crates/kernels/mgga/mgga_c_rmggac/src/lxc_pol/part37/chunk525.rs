//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 525/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk525<F: Float>(t14319: F, t14324: F, t14505: F, t14508: F, t14511: F, t14514: F, t14518: F, t14519: F, t14520: F, t15000: F, t15002: F, t15012: F, t82: F, t72: F, t302: F, t3285: F) -> (F, F, F, F) {
    let t15014 = t15000 - t14505 + t14508 - t14319 + t14324 - t14511 - t14514 + t14518 + 0.19957069503106347607e-1 * t15002 + t14519 - t14520 + t15012;
    let t15015 = t82 * t15014;
    let t15016 = t72 * t15015;
    let t15017 = t302 * t3285;
    (t15014, t15015, t15016, t15017)
}
