//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 893/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk893<F: Float>(t1469: F, t27: F, t16129: F, t1966: F, t221: F, t69002: F, t15379: F, t68499: F, t326: F, t40998: F, t1986: F, t74179: F) -> (F, F, F) {
    let t75976 = t27 * t1469;
    let t75978 = t1966 * t69002 * t221 * t16129 * t75976;
    let t75993 = t15379 * t68499;
    let t75995 = t326 * t40998;
    let t75997 = t74179 * t1986 * t75995;
    (t75978, t75993, t75997)
}
