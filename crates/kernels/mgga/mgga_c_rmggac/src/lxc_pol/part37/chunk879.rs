//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 879/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk879<F: Float>(t1356: F, t14980: F, t5144: F, t5267: F, t5888: F, t70797: F, t73569: F, t739: F, t73936: F, t76779: F, t76780: F, t76781: F, t76787: F, t76790: F, t76792: F, t76794: F, t76796: F, t76799: F, t76800: F, t76801: F, t76802: F, t884: F) -> (F,) {
    let t80014 = 0.87596530464506835932e-6 * t73936 + t76779 - t76780 - t76781 + t76787 + t76790 - t76792 + t70797 + t76794 + t76796 + t76799 + 0.11974241701863808564e0 * t739 * t14980 * t5144 - 0.11974241701863808564e0 * t884 * t14980 * t5267 - 0.11974241701863808564e0 * t1356 * t73569 * t5888 + t76800 + t76801 - t76802;
    (t80014,)
}
