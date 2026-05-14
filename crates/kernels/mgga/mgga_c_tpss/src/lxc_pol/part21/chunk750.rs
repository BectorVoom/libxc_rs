//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 750/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk750<F: Float>(t141: F, t3797: F, t3754: F, t861: F, t3758: F, t2455: F, t2499: F, t2512: F, t2513: F, t3746: F, t3751: F, t3756: F, t3760: F, t3774: F, t3782: F, t3790: F, t3792: F, t3795: F) -> (F, F, F, F, F, F) {
    let t3798 = t141 * t3797;
    let t3800 = t861 * t3754;
    let t3801 = t141 * t3800;
    let t3803 = t861 * t3758;
    let t3804 = t141 * t3803;
    let t3806 = -0.9494625e0 * t3774 + 0.1898925e1 * t3782 + t2499 + 0.99655555555555555557e-1 * t2455 + 0.99655555555555555557e-1 * t3746 - 0.19931111111111111111e0 * t3751 + 0.59793333333333333334e0 * t3756 - 0.29896666666666666667e0 * t3760 + 0.15358125e0 * t3790 + 0.3071625e0 * t3792 + t2512 + 0.54771111111111111111e-1 * t2513 + 0.54771111111111111111e-1 * t3795 - 0.27385555555555555556e-1 * t3798 + 0.16431333333333333333e0 * t3801 - 0.82156666666666666667e-1 * t3804;
    (t3798, t3800, t3801, t3803, t3804, t3806)
}
