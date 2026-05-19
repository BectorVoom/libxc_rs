//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 892/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk892<F: Float>(t623: F, t7190: F, t13841: F, t70225: F, t14063: F, t2411: F, t3151: F, t13823: F, t8465: F, t938: F, t15205: F, t68581: F) -> (F, F, F, F, F) {
    let t75999 = t623 * t7190;
    let t76000 = t75999 * t13841;
    let t76002 = F::cast_from(0.15965655602485078085e0_f64) * t70225;
    let t76017 = t2411 * t14063 * t3151;
    let t76021 = t13823 * t8465 * t938;
    let t76025 = t68581 * t15205;
    (t76000, t76002, t76017, t76021, t76025)
}
