//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1198/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1198<F: Float>(t2531: F, t9722: F, t2379: F, t39483: F, t40727: F, t40730: F, t40732: F, t40734: F, t40737: F, t40739: F, t40741: F, t40743: F, t40746: F, t40748: F, t40750: F, t4314: F, t9470: F) -> (F, F) {
    let t40754 = t2531 * t9722;
    let t40755 = F::cast_from(0.4155806185363551302e3_f64) * t40754;
    let t40756 = -F::new(36.0) * t2379 * t4314 * t9470 + t39483 + t40727 + t40730 - t40732 - t40734 + t40737 - t40739 - t40741 - t40743 + t40746 + t40748 + t40750 + t40755;
    (t40755, t40756)
}
