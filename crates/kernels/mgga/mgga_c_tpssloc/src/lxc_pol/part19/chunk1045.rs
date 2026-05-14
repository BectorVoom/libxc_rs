//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1045/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1045<F: Float>(t12088: F, t2535: F, t2504: F, t2368: F, t746: F, t1294: F, t268: F, t676: F, t9478: F, t9482: F) -> (F, F, F, F, F) {
    let t39387 = t12088 * t2535;
    let t39388 = 0.35089341735807877242e1 * t39387;
    let t39389 = t2504 * t2504;
    let t39391 = t2368 * t39389 * t746;
    let t39393 = 0.35089341735807877242e1 * t1294 * t39391;
    let t39397 = 0.3684616320282908548e2 * t268 * t676 * t9478 * t9482;
    (t39388, t39389, t39391, t39393, t39397)
}
