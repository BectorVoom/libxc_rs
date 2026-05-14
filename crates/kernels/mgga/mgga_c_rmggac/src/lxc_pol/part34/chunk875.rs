//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 875/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk875<F: Float>(t72011: F, t76292: F, t76311: F, t78017: F, t78018: F, t78019: F, t78020: F, t78021: F, t78024: F, t78027: F, t78028: F, t78031: F, t78034: F, t72027: F, t118: F, t77416: F) -> (F, F, F) {
    let t78035 = -t78017 - t78018 - t78019 + t78020 + t78021 - t78024 - t78027 + t76292 - t78028 + t72011 + t78031 + t78034 + t76311;
    let t78036 = 0.11974241701863808564e0 * t72027;
    let t78038 = 0.39914139006212695214e-1 * t118 * t77416;
    (t78035, t78036, t78038)
}
