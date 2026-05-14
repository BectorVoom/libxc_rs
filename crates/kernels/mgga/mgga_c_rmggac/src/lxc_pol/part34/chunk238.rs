//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 238/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk238<F: Float>(t132: F, t31: F, t2034: F, t793: F, t333: F, t645: F, t797: F, t338: F, t36: F) -> (F, F, F, F) {
    let t2051 = t132 * t31;
    let t2055 = t793 * t2034;
    let t2057 = t645 * t333;
    let t2058 = t797 * t2057;
    let t2060 = t338 * t36;
    (t2051, t2055, t2058, t2060)
}
