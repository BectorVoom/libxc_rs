//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 806/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk806<F: Float>(t761: F, t9892: F, t152: F, t31: F, t2368: F, t2505: F, t745: F) -> (F, F, F) {
    let t9894 = F::cast_from(0.51947577317044391277e2_f64) * t761 * t9892;
    let t9897 = t31 * t152;
    let t9905 = t2368 * t745 * t2505;
    (t9894, t9897, t9905)
}
