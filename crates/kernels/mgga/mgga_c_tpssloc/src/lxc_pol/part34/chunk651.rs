//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 651/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk651<F: Float>(t1458: F, t2075: F, t2057: F, t7475: F, t1492: F, t2047: F, t7074: F, t7076: F, t7078: F, t7082: F, t7494: F, t7498: F, t7501: F, t7504: F, t7506: F, t7508: F) -> (F, F, F, F) {
    let t7806 = t2075 * t1458;
    let t7809 = t2057 * t7475;
    let t7815 = t1492 * t2047;
    let t7823 = -t7074 - t7494 / 24.0 - t7076 - 0.24223653656484234512e-2 * t7498 - t7078 - 0.40372756094140390853e-3 * t7501 + t7504 / 768.0 - t7506 / 768.0 - t7082 - t7508 / 192.0;
    (t7806, t7809, t7815, t7823)
}
