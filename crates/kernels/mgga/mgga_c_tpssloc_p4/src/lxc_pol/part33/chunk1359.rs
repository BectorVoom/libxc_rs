//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1359/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1359<F: Float>(t1052: F, t1635: F, t18074: F, t1955: F, t21662: F, t21676: F, t21691: F, t25406: F, t25757: F, t25758: F, t25778: F, t28495: F, t28713: F, t3174: F, t43604: F, t4557: F, t5920: F, t5943: F, t5944: F, t6687: F, t6704: F, t6705: F, t7600: F, t7624: F, t89617: F, t99221: F, t99877: F) -> F {
    let t106492 = -F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t6704 * t6705 * t21662 + F::cast_from(0.82246703342411321826e-2_f64) * t99877 - F::new(3.0) * t25778 * t5944 - F::new(3.0) * t99221 * t1635 + F::new(6.0) * t18074 * t7600 + F::new(6.0) * t1052 * t3174 * t7624 * t5943 + F::new(6.0) * t4557 * t28713 + F::new(6.0) * t25778 * t5920 + F::cast_from(0.49348022005446793095e-1_f64) * t6687 * t25406 * t28495 - F::cast_from(0.54831135561607547884e-2_f64) * t89617 + F::new(24.0) * t1052 * t43604 * t1955 * t21676 - F::new(18.0) * t25757 * t25758 * t21691;
    t106492
}
