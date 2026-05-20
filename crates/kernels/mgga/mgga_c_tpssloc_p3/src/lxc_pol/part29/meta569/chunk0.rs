//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1986/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1986<F: Float>(t13030: F, t225: F, t13062: F, t13378: F, t193: F, t2379: F, t15823: F, t15800: F, t15808: F, t15814: F, t15831: F, t15816: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t47585 = t13030 * t225;
    let t47609 = t13062 * t225;
    let t47618 = t13378 * t225;
    let t47645 = t193 * t2379;
    let t51925 = t15823 * t225;
    let t51928 = t15800 * t225;
    let t51937 = t15808 * t225;
    let t52386 = t15814 * t225;
    let t53658 = t15831 * t225;
    let t53703 = t15816 * t225;
    (t47585, t47609, t47618, t47645, t51925, t51928, t51937, t52386, t53658, t53703)
}
