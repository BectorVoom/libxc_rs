//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1114/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1114<F: Float>(t22705: F, t6978: F, t22704: F, t2006: F, t3787: F, t3793: F, t154: F, t2558: F) -> (F, F, F, F, F) {
    let t22706 = t22705 * t6978;
    let t22707 = t22704 * t22706;
    let t22709 = t3787 * t2006;
    let t22710 = t22709 * t3793;
    let t22715 = t2558 * t154;
    (t22706, t22707, t22709, t22710, t22715)
}
