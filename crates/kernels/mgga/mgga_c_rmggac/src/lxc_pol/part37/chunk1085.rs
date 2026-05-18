//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1085/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1085<F: Float>(t1614: F, t3282: F, t1664: F, t3285: F, t289: F, t75689: F, t75692: F, t75695: F, t75700: F, t75703: F, t75718: F, t77774: F, t77775: F, t77782: F, t77785: F, t77788: F, t77791: F, t77792: F, t77793: F, t77794: F, t884: F) -> (F, F) {
    let t80294 = t3282 * t1614;
    let t80297 = t1664 * t3285;
    let t80300 = t77774 + t77775 - F::new(0.81756761766873046873e-6) * t75689 + F::new(0.52557918278704101561e-6) * t75692 + F::new(0.87596530464506835932e-6) * t75695 - F::new(0.87596530464506835932e-6) * t75700 + F::new(0.17519306092901367187e-6) * t75703 - t77782 + F::new(0.59871208509319042821e-1) * t884 * t80294 - t77785 + t77788 + t77791 - t75718 - F::new(0.2363e1) * t289 * t80297 - t77792 + t77793 + t77794;
    (t80294, t80300)
}
