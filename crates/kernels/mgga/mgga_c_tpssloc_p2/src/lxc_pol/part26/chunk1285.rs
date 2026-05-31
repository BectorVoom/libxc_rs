//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1285/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1285<F: Float>(t23097: F, t2679: F, t776: F, t815: F, t23061: F, t6604: F, t23099: F, t6605: F, t9661: F, t232: F, t47320: F, t81779: F, t81785: F, t81789: F, t81795: F, t81797: F, t81799: F, t81801: F, t81804: F, t81808: F, t81810: F, t81812: F, t81814: F, t81819: F, t81822: F, t81825: F, t81829: F) -> F {
    let t81833 = t23097 * t815 * t2679 * t776;
    let t81835 = t23061 * t6604;
    let t81836 = t81835 * t23099;
    let t81839 = t6605 * t815 * t9661;
    let t81843 = t23097 * t815 * t47320 * t232;
    let t81845 = -F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t81779 - F::cast_from(0.12111826828242117256e-2_f64) * t81785 - F::cast_from(0.94875976821229918508e-2_f64) * t81789 - F::cast_from(0.42391393898847410397e-2_f64) * t81795 - F::cast_from(0.84782787797694820794e-2_f64) * t81797 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t81799 - t81801 / F::cast_from(512.0_f64) + t81804 / F::cast_from(256.0_f64) - F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t81808 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t81810 - t81812 / F::cast_from(1536.0_f64) + t81814 / F::cast_from(256.0_f64) - t81819 / F::cast_from(256.0_f64) - t81822 / F::cast_from(512.0_f64) + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t81825 - F::cast_from(0.25434836339308446237e-1_f64) * t81829 + F::cast_from(0.36335480484726351768e-2_f64) * t81833 - F::cast_from(0.50869672678616892476e-1_f64) * t81836 - F::cast_from(0.20186378047070195427e-3_f64) * t81839 + F::cast_from(0.36335480484726351768e-2_f64) * t81843;
    t81845
}
