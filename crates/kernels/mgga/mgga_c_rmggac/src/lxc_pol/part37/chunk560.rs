//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 560/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk560<F: Float>(t14576: F, t1356: F, t14498: F, t2144: F, t698: F, t507: F) -> (F, F, F, F) {
    let t14577 = F::new(0.39914139006212695214e-1) * t14576;
    let t14578 = t1356 * t14498;
    let t14579 = F::new(0.39914139006212695214e-1) * t14578;
    let t14580 = t2144 * t698;
    let t14581 = t507 * t14580;
    (t14577, t14579, t14580, t14581)
}
