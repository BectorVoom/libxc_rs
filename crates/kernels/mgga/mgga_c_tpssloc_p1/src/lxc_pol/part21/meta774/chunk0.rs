//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2680/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2680<F: Float>(t25: F, t54408: F, t54411: F, t12061: F, t15937: F, t16557: F, t19547: F, t19552: F, t21: F, t2249: F, t3664: F, t3665: F, t39419: F, t5134: F, t5397: F, t54347: F, t56226: F, t584: F, t606: F, t6305: F, t9: F, t9212: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t56298 = F::new(4.0) * t54408;
    let t56299 = F::new(2.0) * t54411;
    let t56323 = piecewise3::<F>(t26, F::new(0.0), F::new(40.0) / F::new(81.0) * t39419 * t6305 * t3665 - F::new(64.0) / F::new(27.0) * t15937 * t56226 - F::new(8.0) / F::new(27.0) * t19547 * t2249 + F::new(32.0) / F::new(9.0) * t3664 * t9 * t21 + F::new(16.0) / F::new(9.0) * t5134 * t584 - F::new(16.0) / F::new(3.0) * t5134 * t9212 - F::new(8.0) / F::new(27.0) * t12061 * t5397 * t3665 + F::new(8.0) / F::new(9.0) * t3664 * t16557 * t606 + F::new(4.0) / F::new(9.0) * t19552 * t2249 + t54347);
    (t56298, t56299, t56323)
}
