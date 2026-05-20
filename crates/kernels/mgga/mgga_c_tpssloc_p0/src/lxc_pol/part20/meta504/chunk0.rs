//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2013/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2013<F: Float>(t9108: F, t94: F, t102: F, t9174: F, t12512: F, t580: F, t1404: F, t3931: F, t1395: F, t3946: F, t12537: F, t576: F) -> (F, F, F, F, F, F) {
    let t35577 = t94 * t9108;
    let t35761 = t102 * t9174;
    let t39022 = t12512 * t580;
    let t39024 = t3931 * t1404;
    let t39026 = t1395 * t3946;
    let t39028 = t576 * t12537;
    (t35577, t35761, t39022, t39024, t39026, t39028)
}
