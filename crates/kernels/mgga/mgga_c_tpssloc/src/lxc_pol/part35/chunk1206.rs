//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1206/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1206<F: Float>(t104122: F, t24682: F, t460: F, t52: F, t6144: F, t18356: F, t24729: F, t27614: F, t4997: F, t1730: F, t27603: F, t27598: F, t5001: F, t2132: F, t24746: F, t1210: F, t24721: F, t29593: F) -> (F, F, F, F, F, F, F, F) {
    let t104239 = t24682 * t104122 * t460;
    let t104280 = t52 * t6144;
    let t104282 = t24682 * t104280 * t460;
    let t104294 = t24729 * t18356;
    let t104296 = t27614 * t4997;
    let t104300 = t1730 * t27603;
    let t104303 = t5001 * t27598;
    let t104337 = t2132 * t104280 * t24746;
    let t104355 = t24721 * t1210 * t29593;
    (t104239, t104282, t104294, t104296, t104300, t104303, t104337, t104355)
}
