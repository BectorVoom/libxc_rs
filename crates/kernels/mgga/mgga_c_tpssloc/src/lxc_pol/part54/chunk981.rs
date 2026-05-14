//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 981/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk981<F: Float>(t22767: F, t22780: F, t22799: F, t22805: F, t24049: F, t24050: F, t26234: F, t26236: F, t26238: F, t26240: F, t26246: F, t26249: F, t26286: F, t26290: F, t26293: F, t26295: F, t26299: F, t26303: F, t27012: F, t27019: F, t27032: F, t27049: F) -> (F,) {
    let t27051 = t27012 - t26234 / 768.0 - t26236 / 768.0 - t26238 / 768.0 + 5.0 / 192.0 * t26240 + t22767 + 0.67287926823567318088e-4 * t26246 + t26249 / 768.0 - t27019 + 0.28260929265898273597e-2 * t22780 + t27032 + t22799 + 0.16956557559538964158e-1 * t22805 - t24049 + t24050 + t26286 / 8.0 + 0.16956557559538964158e-1 * t26290 - 0.40372756094140390853e-3 * t26293 + 0.28260929265898273597e-2 * t26295 + 0.24223653656484234512e-2 * t26299 + 0.24223653656484234512e-2 * t26303 + t27049;
    (t27051,)
}
