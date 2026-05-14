//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 369/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk369<F: Float>(t271: F, t7554: F, t131: F, t31: F, t640: F, t27: F, t3118: F, t2123: F, t874: F, t36: F) -> (F, F, F, F, F, F) {
    let t7555 = t7554 * t271;
    let t7556 = t131 * t31;
    let t7557 = t640 * t7556;
    let t7561 = t27 * t3118;
    let t7567 = t874 * t2123;
    let t7577 = t874 * t36;
    (t7555, t7556, t7557, t7561, t7567, t7577)
}
