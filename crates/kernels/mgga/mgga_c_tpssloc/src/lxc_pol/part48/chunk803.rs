//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 803/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk803<F: Float>(t3243: F, t7363: F, t24776: F, t2148: F, t3471: F, t3616: F, t7376: F, t7375: F, t225: F, t7319: F, t7364: F, t24757: F, t493: F) -> (F, F, F, F, F) {
    let t24777 = t7363 * t3243;
    let t24778 = t24776 * t24777;
    let t24781 = t3471 * t2148;
    let t24784 = t3616 * t7376;
    let t24785 = t7375 * t24784;
    let t24788 = t7319 * t225;
    let t24789 = t24788 * t7364;
    let t24792 = t493 * t24757;
    (t24778, t24781, t24785, t24789, t24792)
}
