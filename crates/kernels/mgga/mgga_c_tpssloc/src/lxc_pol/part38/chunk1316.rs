//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1316/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1316<F: Float>(t12813: F, t88: F, t1458: F, t2311: F, t89: F, t1395: F, t8171: F, t29978: F, t580: F, t1404: F, t8153: F, t2193: F, t3931: F) -> (F, F, F, F, F, F, F) {
    let t90375 = t88 * t12813;
    let t90381 = t2311 * t1458;
    let t91753 = t89 * t12813;
    let t110014 = t1395 * t8171;
    let t110018 = t29978 * t580;
    let t110020 = t8153 * t1404;
    let t110024 = t3931 * t2193;
    (t90375, t90381, t91753, t110014, t110018, t110020, t110024)
}
