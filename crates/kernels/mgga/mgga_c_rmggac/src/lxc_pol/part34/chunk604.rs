//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 604/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk604<F: Float>(t1985: F, t3839: F, t14025: F, t35311: F, t1965: F, t68522: F, t13850: F, t1977: F, t13858: F, t2186: F, t14286: F, t352: F, t262: F, t8620: F, t1322: F, t507: F, t7190: F) -> (F, F, F, F, F, F, F, F, F) {
    let t68626 = t1985 * t3839;
    let t68651 = t14025 * t35311;
    let t68658 = t1965 * t68522;
    let t68660 = t1977 * t68658 * t13850;
    let t68669 = t2186 * t13858;
    let t68684 = t14286 * t352;
    let t68685 = t262 * t68684;
    let t68686 = t8620 * t68685;
    let t68729 = t507 * t7190 * t1322;
    (t68626, t68651, t68658, t68660, t68669, t68684, t68685, t68686, t68729)
}
