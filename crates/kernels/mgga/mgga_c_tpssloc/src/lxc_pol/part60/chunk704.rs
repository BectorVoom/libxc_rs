//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 704/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk704<F: Float>(t22827: F, t28101: F, t22833: F, t6396: F, t22820: F, t22826: F, t22859: F, t22864: F, t22868: F, t26272: F, t26295: F, t28085: F, t28089: F, t28091: F, t28093: F, t28095: F, t28097: F) -> (F, F, F) {
    let t28102 = t22827 * t28101;
    let t28104 = t22833 * t6396;
    let t28106 = 0.40372756094140390854e-3 * t26272 + t28085 / 768.0 - t22820 + t22826 + 0.28260929265898273598e-2 * t26295 + t28089 / 1536.0 - t28091 / 1536.0 + 5.0 / 384.0 * t28093 - t28095 / 384.0 - t28097 / 192.0 + 0.24223653656484234512e-2 * t28102 + t22859 + t22864 + t22868 + t28104 / 192.0;
    (t28102, t28104, t28106)
}
