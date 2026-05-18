//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 910/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk910<F: Float>(t10294: F, t2403: F, t909: F, t2827: F, t699: F, t2830: F, t2833: F, t241: F, t2978: F, t2955: F, t969: F, t2967: F, t964: F) -> (F, F, F, F, F, F, F, F) {
    let t10295 = F::new(20.0) / F::new(27.0) * t10294;
    let t10296 = t2403 * t909;
    let t10298 = t699 * t2827;
    let t10300 = t699 * t2830;
    let t10302 = t699 * t2833;
    let t10304 = t241 * t2978;
    let t10331 = t2955 * t969;
    let t10333 = t964 * t2967;
    (t10295, t10296, t10298, t10300, t10302, t10304, t10331, t10333)
}
