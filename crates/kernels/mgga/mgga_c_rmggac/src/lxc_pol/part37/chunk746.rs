//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 746/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk746<F: Float>(t14516: F, t7288: F, t2227: F, t36: F, t2123: F, t698: F, t664: F, t305: F, t71835: F, t265: F) -> (F, F, F, F, F, F) {
    let t71892 = t14516 * t7288;
    let t71903 = t2227 * t36;
    let t71910 = t698 * t2123;
    let t71916 = t2227 * t664;
    let t71940 = t305 * t71835;
    let t71949 = t698 * t265;
    (t71892, t71903, t71910, t71916, t71940, t71949)
}
