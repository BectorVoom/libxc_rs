//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 207/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk207<F: Float>(t1423: F, t1538: F, t109: F, t321: F, t571: F, t333: F, t117: F, t899: F) -> (F, F, F, F, F) {
    let t1539 = t1423 + t1538;
    let t1540 = t1539 * t109;
    let t1544 = t571 * t321;
    let t1547 = t571 * t333;
    let t1550 = t899 * t117;
    (t1539, t1540, t1544, t1547, t1550)
}
