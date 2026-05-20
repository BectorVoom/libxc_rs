//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1979/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1979<F: Float>(t87262: F, t87270: F, t87272: F, t81789: F, t81795: F, t81797: F, t81799: F, t81808: F, t81810: F, t81825: F, t81836: F, t84896: F, t84897: F, t87274: F, t87276: F, t87278: F, t87280: F, t87284: F) -> F {
    let t92607 = F::new(7.0) / F::new(576.0) * t87262;
    let t92614 = F::new(7.0) / F::new(144.0) * t87270;
    let t92615 = F::new(7.0) / F::new(576.0) * t87272;
    let t92623 = t92607 - F::cast_from(0.12650130242830655801e-1_f64) * t81789 - F::cast_from(0.28260929265898273597e-2_f64) * t81795 - F::cast_from(0.56521858531796547194e-2_f64) * t81797 + F::new(7.0) / F::new(72.0) * t81799 - F::new(119.0) / F::new(1728.0) * t81808 + F::new(7.0) / F::new(1152.0) * t81810 - t92614 + t92615 + t87274 / F::new(384.0) + t87276 / F::new(192.0) + t87278 / F::new(192.0) + t87280 / F::new(192.0) + F::new(7.0) / F::new(576.0) * t81825 - F::cast_from(0.33913115119077928316e-1_f64) * t81836 - t84896 - t84897 - t87284 / F::new(48.0);
    t92623
}
