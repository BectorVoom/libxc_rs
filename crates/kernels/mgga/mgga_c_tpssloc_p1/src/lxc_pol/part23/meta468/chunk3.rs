//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1377/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1377<F: Float>(t42213: F, t47787: F, t76587: F, t76595: F, t76610: F, t76618: F, t76626: F, t76899: F, t76903: F, t76906: F, t76912: F, t77102: F, t77105: F, t77107: F) -> F {
    let t77301 = t42213 - F::cast_from(0.27785333333333333334e0_f64) * t76899 + F::cast_from(0.83356e0_f64) * t76903 - F::cast_from(0.13892666666666666667e0_f64) * t76906 - F::cast_from(0.375102e1_f64) * t76912 + F::cast_from(0.3529725e1_f64) * t77102 + F::cast_from(0.21424148148148148148e1_f64) * t47787 - F::cast_from(0.52945875e1_f64) * t77105 + F::cast_from(0.2366859375e0_f64) * t77107 - F::cast_from(0.34431666666666666667e1_f64) * t76587 + F::cast_from(0.123954e2_f64) * t76595 - F::cast_from(0.13772666666666666667e1_f64) * t76610 - F::cast_from(0.185931e2_f64) * t76618 + F::cast_from(0.41318e1_f64) * t76626;
    t77301
}
