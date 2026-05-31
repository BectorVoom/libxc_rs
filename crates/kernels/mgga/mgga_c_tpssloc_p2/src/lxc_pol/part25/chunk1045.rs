//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1045/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1045<F: Float>(t6999: F, t7217: F, t22754: F, t22757: F, t22762: F, t22766: F, t22768: F, t22771: F, t22774: F, t22777: F, t22780: F, t22784: F, t22786: F, t22789: F, t22795: F, t22798: F, t22800: F) -> (F, F) {
    let t24028 = t7217 * t6999;
    let t24046 = -t22754 / F::cast_from(768.0_f64) - t22757 / F::cast_from(384.0_f64) + t22762 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t22766 - t22768 / F::cast_from(768.0_f64) - F::cast_from(0.40372756094140390853e-3_f64) * t22771 - F::cast_from(0.40372756094140390853e-3_f64) * t22774 + F::cast_from(0.80745512188280781706e-3_f64) * t22777 + F::cast_from(0.56521858531796547194e-2_f64) * t22780 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t22784 - t22786 / F::cast_from(192.0_f64) - t22789 / F::cast_from(96.0_f64) + F::cast_from(0.80745512188280781706e-3_f64) * t22795 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t22798 - t22800 / F::cast_from(24.0_f64);
    (t24028, t24046)
}
