//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 817/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk817<F: Float>(t6999: F, t7217: F, t22754: F, t22757: F, t22762: F, t22766: F, t22768: F, t22771: F, t22774: F, t22777: F, t22780: F, t22784: F, t22786: F, t22789: F, t22795: F, t22798: F, t22800: F) -> (F, F) {
    let t24028 = t7217 * t6999;
    let t24046 = -t22754 / F::new(768.0) - t22757 / F::new(384.0) + t22762 / F::new(384.0) + F::new(7.0) / F::new(576.0) * t22766 - t22768 / F::new(768.0) - F::new(0.40372756094140390853e-3) * t22771 - F::new(0.40372756094140390853e-3) * t22774 + F::new(0.80745512188280781706e-3) * t22777 + F::new(0.56521858531796547194e-2) * t22780 + F::new(7.0) / F::new(144.0) * t22784 - t22786 / F::new(192.0) - t22789 / F::new(96.0) + F::new(0.80745512188280781706e-3) * t22795 + F::new(7.0) / F::new(36.0) * t22798 - t22800 / F::new(24.0);
    (t24028, t24046)
}
