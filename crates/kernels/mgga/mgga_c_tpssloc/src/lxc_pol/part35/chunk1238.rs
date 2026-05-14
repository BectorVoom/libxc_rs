//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1238/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1238<F: Float>(t105727: F, t105732: F, t105741: F, t105745: F, t105755: F, t105759: F, t105763: F, t105766: F, t105770: F, t105773: F, t1877: F, t1915: F, t1916: F, t20216: F, t22959: F, t25: F, t25013: F, t2522: F, t25372: F, t28241: F, t28249: F, t4314: F, t5397: F, t7541: F, t7545: F, t86736: F, t98054: F) -> (F,) {
    let t105776 = -3.0 / 2.0 * t1877 * t98054 * t7545 + t1877 * t105727 * t25 / 2.0 + 9.0 * t22959 * t105732 + 9.0 * t4314 * t7541 * t28241 + 3.0 / 2.0 * t1877 * t7541 * t5397 + 9.0 / 2.0 * t2522 * t1915 * t105741 + 9.0 / 2.0 * t2522 * t1915 * t105745 + t1877 * t1915 * t20216 / 2.0 - 9.0 * t86736 * t28249 - 9.0 / 2.0 * t22959 * t105755 - 9.0 / 2.0 * t22959 * t105759 - 9.0 * t25013 * t105763 - 9.0 * t22959 * t105766 + 3.0 * t25372 * t105770 + 3.0 * t105773 * t1916;
    (t105776,)
}
