//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 739/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk739<F: Float>(t69583: F, t14413: F, t638: F, t7292: F, t14417: F, t2046: F, t7297: F, t2039: F, t2244: F, t270: F, t2227: F, t235: F, t7190: F) -> (F, F, F, F, F) {
    let t71369 = F::new(0.17347588262831798124e-3) * t69583;
    let t71372 = t638 * t7292 * t14413;
    let t71373 = F::new(0.81300399444200075504e-3) * t71372;
    let t71375 = t2046 * t7297 * t14417;
    let t71376 = F::new(0.1951603679568577289e-3) * t71375;
    let t71380 = t638 * t2039 * t2244 * t270;
    let t71400 = t235 * t7190 * t2227;
    (t71369, t71373, t71376, t71380, t71400)
}
