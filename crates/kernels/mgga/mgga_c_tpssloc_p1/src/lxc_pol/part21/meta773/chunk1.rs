//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2676/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2676<F: Float>(t25: F, t54402: F, t2: F, t584: F, t606: F, t11987: F, t15989: F, t16557: F, t19606: F, t19611: F, t21: F, t2249: F, t3665: F, t3704: F, t39861: F, t5170: F, t53825: F, t5397: F, t6305: F, t9: F, t9212: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t56219 = F::new(32.0) * t54402;
    let t56226 = t606 * t2 * t584;
    let t56247 = piecewise3::<F>(t26, F::new(0.0), -F::new(56.0) / F::new(81.0) * t39861 * t6305 * t3665 + F::new(64.0) / F::new(27.0) * t15989 * t56226 + F::new(8.0) / F::new(27.0) * t19606 * t2249 - F::new(16.0) / F::new(9.0) * t3704 * t9 * t21 - F::new(8.0) / F::new(9.0) * t5170 * t584 + F::new(8.0) / F::new(3.0) * t5170 * t9212 + F::new(8.0) / F::new(27.0) * t11987 * t5397 * t3665 - F::new(4.0) / F::new(9.0) * t3704 * t16557 * t606 - F::new(2.0) / F::new(9.0) * t19611 * t2249 + t53825);
    (t56219, t56226, t56247)
}
