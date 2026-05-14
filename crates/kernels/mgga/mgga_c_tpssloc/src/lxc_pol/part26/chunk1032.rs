//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1032/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1032<F: Float>(t23146: F, t2649: F, t23096: F, t23100: F, t23106: F, t23108: F, t23114: F, t23117: F, t23120: F, t23125: F, t23128: F, t23130: F, t23135: F, t23136: F, t23141: F, t23144: F) -> (F,) {
    let t23147 = t23146 * t2649;
    let t23149 = t23096 + 0.24223653656484234512e-2 * t23100 - t23106 + t23108 + 0.6728792682356731809e-4 * t23114 + t23117 / 1536.0 - t23120 + 0.40372756094140390854e-3 * t23125 - t23128 / 192.0 + 5.0 / 384.0 * t23130 + t23135 - t23136 / 384.0 + t23141 + t23144 + t23147 / 192.0;
    (t23149,)
}
