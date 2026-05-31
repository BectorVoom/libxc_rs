//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 225/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk225<F: Float>(t1369: F, t326: F, t1587: F, t27: F, t29: F, t847: F) -> (F, F, F) {
    let t1609 = t326 * t1369;
    let t1612 = t1587 * t29 * t27;
    let t1614 = F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t1612 + t847;
    (t1609, t1612, t1614)
}
