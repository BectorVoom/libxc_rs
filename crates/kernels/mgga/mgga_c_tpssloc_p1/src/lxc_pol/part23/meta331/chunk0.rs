//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1099/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1099<F: Float>(t20: F, t60: F, t9108: F, t94: F, t102: F, t9174: F, t16: F, t2: F, t591: F, t21: F, t9: F, t587: F, t598: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32253 = F::new(1.0) / t60 / t20;
    let t35577 = t94 * t9108;
    let t35761 = t102 * t9174;
    let t39030 = F::new(0.7464e2) * t16;
    let t39031 = t2 * t591;
    let t39032 = F::new(0.35904e3) * t39031;
    let t39033 = t9 * t21;
    let t39034 = F::new(1638.0) * t39033;
    let t39035 = t587 * t598;
    (t32253, t35577, t35761, t39030, t39031, t39032, t39033, t39034, t39035)
}
