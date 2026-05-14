//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 638/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk638<F: Float>(t6793: F, t202: F, t6665: F, t1877: F, t1915: F, t193: F, t2522: F, t6670: F, t776: F, t868: F, t870: F, t28: F, t1081: F, t6666: F, t1873: F, t2314: F) -> (F, F, F, F, F, F) {
    let t6794 = 1.0 / t6793;
    let t6829 = t202 * t6665;
    let t6834 = -t1877 * t6670 * t868 + 3.0 * t1915 * t2522 * t776 + t193 * t6829 * t870;
    let t6841 = t28 * t776;
    let t6848 = t28 * t868;
    let t6855 = 3.0 / 2.0 * t2522 * t1915 * t6841 + t1877 * t6666 * t28 / 2.0 - t1877 * t6670 * t6848 / 2.0 + t1877 * t1915 * t1081 / 2.0;
    let t6867 = 2.0 * t2314 * t1873;
    (t6794, t6834, t6841, t6848, t6855, t6867)
}
