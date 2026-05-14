//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 644/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk644<F: Float>(t14642: F, t14138: F, t14142: F, t14205: F, t14686: F, t14697: F, t14272: F, t14396: F, t16156: F, t3219: F, t34881: F, t68401: F, t68414: F, t1550: F, t699: F, t7596: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t70720 = 0.19863479950205658386e-4 * t14642;
    let t70721 = 0.60975299583150056624e-3 * t14138;
    let t70722 = 0.86737941314158990616e-4 * t14142;
    let t70735 = 0.162600798888400151e-2 * t14205;
    let t70741 = 0.79828278012425390426e-1 * t14686;
    let t70745 = 0.30487649791575028314e-3 * t14697;
    let t70746 = 0.17347588262831798124e-3 * t14272;
    let t70748 = t16156 * t14396;
    let t70754 = t34881 * t3219;
    let t70797 = 0.36765206969775206063e-5 * t68401;
    let t70799 = 0.51300288795035171252e-6 * t68414;
    let t70806 = t1550 * t699 * t7596;
    (t70720, t70721, t70722, t70735, t70741, t70745, t70746, t70748, t70754, t70797, t70799, t70806)
}
