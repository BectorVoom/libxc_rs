//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2642/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2642<F: Float>(t25: F, t53796: F, t5154: F, t9919: F, t39305: F, t3665: F, t584: F, t2249: F, t606: F, t16: F, t5173: F, t591: F, t11987: F, t11988: F, t1298: F, t1408: F, t15989: F, t15992: F, t2: F, t3704: F, t39861: F, t5170: F, t9257: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t53797 = F::cast_from(0.35089341735807877242e1_f64) * t53796;
    let t53798 = t5154 * t9919;
    let t53799 = F::cast_from(0.35089341735807877242e1_f64) * t53798;
    let t53800 = F::cast_from(0.31168546390226634765e3_f64) * t39305;
    let t53805 = t584 * t3665;
    let t53808 = t606 * t2249;
    let t53814 = t16 * t606;
    let t53817 = t584 * t2249;
    let t53825 = F::new(16.0) * t5173 * t591;
    let t53827 = piecewise3::<F>(t26, F::new(0.0), -F::new(56.0) / F::new(81.0) * t39861 * t1408 * t11988 + F::new(16.0) / F::new(9.0) * t11987 * t2 * t53805 + F::new(8.0) / F::new(9.0) * t15989 * t53808 - F::new(4.0) / F::new(3.0) * t3704 * t584 * t606 + F::new(4.0) * t15992 * t53814 - F::new(4.0) / F::new(3.0) * t15992 * t53817 - F::new(2.0) / F::new(9.0) * t5170 * t9257 - F::new(8.0) * t1298 * t16 + t53825);
    (t53797, t53799, t53800, t53805, t53808, t53814, t53817, t53827)
}
