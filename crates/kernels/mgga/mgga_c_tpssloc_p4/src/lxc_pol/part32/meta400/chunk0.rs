//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1513/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1513<F: Float>(t17375: F, t17449: F, t17516: F, t17558: F, t300: F, t2940: F, t5808: F, t10629: F, t5774: F, t10632: F, t950: F, t959: F) -> (F, F, F) {
    let t17561 = t300 * (t17375 + t17449 + t17516 + t17558);
    let t17563 = F::cast_from(0.5848223622634646207e0_f64) * t2940 * t5808;
    let t17564 = t10629 * t5774;
    let t17565 = t10632 * t950;
    let t17566 = t17564 * t17565;
    let t17568 = F::cast_from(0.10254018858216406658e4_f64) * t959 * t17566;
    (t17561, t17563, t17568)
}
