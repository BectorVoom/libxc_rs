//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 623/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk623<F: Float>(t3139: F, t465: F, t7472: F, t1986: F, t305: F, t7476: F, t118: F, t2001: F, t498: F, t665: F, t2000: F, t797: F, t201: F, t14056: F, t14371: F, t13889: F, t14368: F) -> (F, F, F, F, F, F, F, F) {
    let t69618 = t465 * t3139;
    let t69619 = t7472 * t69618;
    let t69621 = t1986 * t305 * t7476;
    let t69626 = t2001 * t118 * t665 * t498;
    let t69629 = t2000 * t797;
    let t69635 = t201 * t201;
    let t69648 = t14371 * t14056;
    let t69662 = t14368 * t13889;
    (t69618, t69619, t69621, t69626, t69629, t69635, t69648, t69662)
}
