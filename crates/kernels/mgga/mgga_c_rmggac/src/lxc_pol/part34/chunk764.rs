//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 764/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk764<F: Float>(t13841: F, t75999: F, t70225: F, t14063: F, t2411: F, t3151: F, t13823: F, t8465: F, t938: F, t15205: F, t68581: F, t13819: F, t8343: F, t13872: F, t15296: F, t13876: F) -> (F, F, F, F, F, F, F, F) {
    let t76000 = t75999 * t13841;
    let t76002 = 0.15965655602485078085e0 * t70225;
    let t76017 = t2411 * t14063 * t3151;
    let t76021 = t13823 * t8465 * t938;
    let t76025 = t68581 * t15205;
    let t76027 = t13819 * t8343;
    let t76029 = t15296 * t13872;
    let t76031 = t15296 * t13876;
    (t76000, t76002, t76017, t76021, t76025, t76027, t76029, t76031)
}
