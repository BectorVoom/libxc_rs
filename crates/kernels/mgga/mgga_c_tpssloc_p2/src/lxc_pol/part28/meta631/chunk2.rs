//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1979/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1979<F: Float>(t87262: F, t87270: F, t87272: F, t81789: F, t81795: F, t81797: F, t81799: F, t81808: F, t81810: F, t81825: F, t81836: F, t84896: F, t84897: F, t87274: F, t87276: F, t87278: F, t87280: F, t87284: F) -> F {
    let t92607 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t87262;
    let t92614 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t87270;
    let t92615 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t87272;
    let t92623 = t92607 - F::cast_from(0.12650130242830655801e-1_f64) * t81789 - F::cast_from(0.28260929265898273597e-2_f64) * t81795 - F::cast_from(0.56521858531796547194e-2_f64) * t81797 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t81799 - F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t81808 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t81810 - t92614 + t92615 + t87274 / F::cast_from(384.0_f64) + t87276 / F::cast_from(192.0_f64) + t87278 / F::cast_from(192.0_f64) + t87280 / F::cast_from(192.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t81825 - F::cast_from(0.33913115119077928316e-1_f64) * t81836 - t84896 - t84897 - t87284 / F::cast_from(48.0_f64);
    t92623
}
