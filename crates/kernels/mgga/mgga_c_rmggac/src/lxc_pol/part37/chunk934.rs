//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 934/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk934<F: Float>(t15882: F, t321: F, t118: F, t305: F, t71852: F, t71854: F, t76141: F, t77830: F, t77833: F, t77835: F, t77836: F, t77837: F, t77839: F, t77841: F, t77843: F, t80102: F) -> (F, F) {
    let t80402 = t15882 * t321;
    let t80407 = 0.59871208509319042821e-1 * t305 * t80402 - t71852 - t76141 + t71854 - t77830 - 0.39914139006212695214e-1 * t118 * t80102 - t77833 - t77835 - t77836 - t77837 - t77839 + t77841 + t77843;
    (t80402, t80407)
}
