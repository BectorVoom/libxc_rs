//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2156/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2156<F: Float>(t3032: F, t52434: F, t3505: F, t3514: F, t11835: F, t4889: F, t1174: F, t1725: F, t2402: F, t3506: F, t4979: F, t49850: F) -> (F, F, F, F, F) {
    let t53371 = t52434 * t3032;
    let t53372 = t53371 * t3505;
    let t53399 = t53371 * t3514;
    let t53433 = t4889 * t11835;
    let t53434 = t53433 / F::cast_from(162.0_f64);
    let t53440 = t1174 * t2402 * t1725;
    let t53452 = t3506 * t49850 * t4979;
    (t53372, t53399, t53434, t53440, t53452)
}
