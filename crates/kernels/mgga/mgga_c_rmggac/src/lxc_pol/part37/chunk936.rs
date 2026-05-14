//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 936/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk936<F: Float>(t305: F, t326: F, t77875: F, t77878: F, t77881: F, t77883: F, t77884: F, t77887: F, t77888: F, t77889: F, t77898: F, t77899: F, t77900: F, t77904: F, t80280: F, t80341: F) -> (F,) {
    let t80426 = 0.59871208509319042821e-1 * t305 * t80341 - 0.59871208509319042821e-1 * t326 * t80280 - t77875 - t77878 - t77881 - t77883 - t77884 + t77887 + t77888 + t77889 - t77898 + t77899 + t77900 - t77904;
    (t80426,)
}
