//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1128/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1128<F: Float>(t29748: F, t29793: F, t1241: F, t2154: F, t6243: F, t11606: F, t24615: F, t7300: F, t1409: F, t1760: F, t24602: F, t24601: F, t5979: F, t7286: F, t7285: F, t5975: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t29794 = t29748 + t29793;
    let t29795 = t1241 * t29794;
    let t29797 = t2154 * t6243;
    let t29798 = t11606 * t29797;
    let t29803 = t24615 * t6243;
    let t29804 = t7300 * t29803;
    let t29808 = t24602 * t1409 * t1760;
    let t29809 = t24601 * t29808;
    let t29812 = t7286 * t5979;
    let t29813 = t7285 * t29812;
    let t29816 = t7286 * t5975;
    (t29794, t29795, t29798, t29803, t29804, t29808, t29809, t29812, t29813, t29816)
}
