//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2662/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2662<F: Float>(t25: F, t54323: F, t5157: F, t9874: F, t5137: F, t591: F, t11988: F, t12061: F, t1408: F, t15937: F, t15940: F, t16: F, t2: F, t3664: F, t39419: F, t5134: F, t514: F, t53805: F, t53808: F, t53814: F, t53817: F, t584: F, t606: F, t9257: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t54324 = F::new(12.0) * t54323;
    let t54325 = t5157 * t9874;
    let t54326 = F::cast_from(0.56968947174242584612e-3_f64) * t54325;
    let t54347 = F::new(32.0) * t5137 * t591;
    let t54349 = piecewise3::<F>(t26, F::new(0.0), F::new(40.0) / F::new(81.0) * t39419 * t1408 * t11988 - F::new(16.0) / F::new(9.0) * t12061 * t2 * t53805 - F::new(8.0) / F::new(9.0) * t15937 * t53808 + F::new(8.0) / F::new(3.0) * t3664 * t584 * t606 - F::new(8.0) * t15940 * t53814 + F::new(8.0) / F::new(3.0) * t15940 * t53817 + F::new(4.0) / F::new(9.0) * t5134 * t9257 - F::new(16.0) * t514 * t16 + t54347);
    (t54324, t54326, t54349)
}
