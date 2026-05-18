//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1039/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1039<F: Float>(t102801: F, t1992: F, t22897: F, t3792: F, t114104: F, t114119: F, t122507: F, t122533: F, t122535: F, t127402: F, t127403: F, t127404: F, t127408: F, t127412: F) -> F {
    let t128880 = t1992 * t22897 * t102801 * t3792;
    let t128882 = -F::new(0.82246703342411321824e-2) * t122507 + t114104 + t127402 - t127403 - t127404 - t127408 + t127412 + F::new(0.16449340668482264365e-1) * t122533 + F::new(0.76763589786250567036e-1) * t122535 + F::new(0.16449340668482264365e-1) * t128880 + t114119;
    t128882
}
