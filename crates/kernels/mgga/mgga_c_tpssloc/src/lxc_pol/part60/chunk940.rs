//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 940/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk940<F: Float>(t32792: F, t6547: F, t23204: F, t32866: F, t6562: F, t32809: F, t8335: F, t86893: F, t214: F, t7510: F, t32867: F, t32875: F) -> (F, F, F, F, F, F, F) {
    let t118858 = t6547 * t32792;
    let t118885 = t6562 * t23204 * t32866;
    let t118893 = t6547 * t32809;
    let t118903 = t6562 * t86893 * t8335;
    let t118910 = t214 * t7510;
    let t118915 = t6547 * t32867;
    let t118927 = t6547 * t32875;
    (t118858, t118885, t118893, t118903, t118910, t118915, t118927)
}
