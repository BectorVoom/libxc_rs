//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1156/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1156<F: Float>(t1799: F, t1824: F, t550: F, t1339: F, t22827: F, t22833: F, t6396: F, t22820: F, t22826: F, t22859: F, t22864: F, t22868: F, t26272: F, t26295: F, t28085: F, t28089: F, t28091: F, t28093: F, t28095: F, t28097: F) -> (F, F, F) {
    let t28099 = t1799 * t1824;
    let t28100 = t28099 * t550;
    let t28101 = t1339 * t28100;
    let t28102 = t22827 * t28101;
    let t28104 = t22833 * t6396;
    let t28106 = F::new(0.40372756094140390854e-3) * t26272 + t28085 / F::new(768.0) - t22820 + t22826 + F::new(0.28260929265898273598e-2) * t26295 + t28089 / F::new(1536.0) - t28091 / F::new(1536.0) + F::new(5.0) / F::new(384.0) * t28093 - t28095 / F::new(384.0) - t28097 / F::new(192.0) + F::new(0.24223653656484234512e-2) * t28102 + t22859 + t22864 + t22868 + t28104 / F::new(192.0);
    (t28100, t28101, t28106)
}
