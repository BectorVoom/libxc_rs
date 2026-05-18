//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 816/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk816<F: Float>(t20085: F, t2095: F, t24432: F, t28830: F, t23957: F, t28826: F, t26231: F, t26246: F, t26251: F, t26255: F, t26266: F, t26268: F, t28058: F, t28061: F, t28063: F, t28065: F, t28068: F, t28070: F, t28074: F, t28078: F, t28080: F) -> (F, F, F, F) {
    let t29243 = t2095 * t20085;
    let t29247 = t24432 * t28830;
    let t29252 = t23957 * t28826;
    let t29274 = F::new(7.0) / F::new(576.0) * t26231 + F::new(0.13457585364713463618e-3) * t26246 - F::new(7.0) / F::new(576.0) * t26251 + F::new(0.80745512188280781706e-3) * t28058 - F::new(0.40372756094140390853e-3) * t28061 - t28063 / F::new(768.0) - t28065 / F::new(384.0) - F::new(0.40372756094140390853e-3) * t28068 + F::new(7.0) / F::new(144.0) * t26255 + t28070 / F::new(8.0) + F::new(0.16956557559538964158e-1) * t28074 - F::new(0.24223653656484234512e-2) * t28078 - t28080 / F::new(24.0) + F::new(7.0) / F::new(36.0) * t26266 + F::new(0.33913115119077928316e-1) * t26268;
    (t29243, t29247, t29252, t29274)
}
