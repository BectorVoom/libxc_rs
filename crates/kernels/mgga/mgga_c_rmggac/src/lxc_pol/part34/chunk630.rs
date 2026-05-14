//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 630/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk630<F: Float>(t118: F, t1986: F, t495: F, t665: F, t69157: F, t7204: F, t69161: F, t7192: F, t140: F, t212: F, t3151: F, t4071: F, t672: F, t1330: F, t236: F, t899: F) -> (F, F, F, F, F) {
    let t69971 = t1986 * t118 * t665 * t495;
    let t69976 = t7204 * t69157;
    let t69983 = t7192 * t69161;
    let t69995 = t672 * t212 * t4071 * t140 * t3151;
    let t70018 = t899 * t236 * t1330;
    (t69971, t69976, t69983, t69995, t70018)
}
