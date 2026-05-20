//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1205/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1205<F: Float>(t81779: F, t81785: F, t81789: F, t81795: F, t81797: F, t81799: F, t81801: F, t81804: F, t81808: F, t81810: F, t81812: F, t81814: F, t81819: F, t81822: F, t81825: F, t81829: F, t81833: F, t81836: F, t81839: F, t81843: F) -> F {
    let t84894 = -F::new(5.0) / F::new(32.0) * t81779 - F::cast_from(0.24223653656484234512e-2_f64) * t81785 - F::cast_from(0.18975195364245983701e-1_f64) * t81789 - F::cast_from(0.84782787797694820791e-2_f64) * t81795 - F::cast_from(0.16956557559538964158e-1_f64) * t81797 + F::new(7.0) / F::new(24.0) * t81799 - t81801 / F::new(256.0) + t81804 / F::new(128.0) - F::new(119.0) / F::new(1152.0) * t81808 + F::new(7.0) / F::new(384.0) * t81810 - t81812 / F::new(768.0) + t81814 / F::new(128.0) - t81819 / F::new(128.0) - t81822 / F::new(256.0) + F::new(7.0) / F::new(192.0) * t81825 - F::cast_from(0.50869672678616892475e-1_f64) * t81829 + F::cast_from(0.72670960969452703536e-2_f64) * t81833 - F::cast_from(0.10173934535723378495e0_f64) * t81836 - F::cast_from(0.40372756094140390853e-3_f64) * t81839 + F::cast_from(0.72670960969452703536e-2_f64) * t81843;
    t84894
}
