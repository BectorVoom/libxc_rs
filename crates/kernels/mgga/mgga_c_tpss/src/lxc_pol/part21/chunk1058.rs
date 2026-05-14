//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1058/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1058<F: Float>(t2885: F, t4079: F, t1027: F, t2877: F, t4087: F, t1985: F, t4046: F, t1038: F, t141: F, t4048: F, t664: F) -> (F, F, F, F, F) {
    let t11864 = t2885 * t4079;
    let t11865 = t11864 * t1027;
    let t11867 = t4087 * t2877;
    let t11869 = t4046 * t1985;
    let t11870 = t1038 * t11869;
    let t11871 = t141 * t11870;
    let t11873 = t664 * t4048;
    (t11865, t11867, t11869, t11871, t11873)
}
