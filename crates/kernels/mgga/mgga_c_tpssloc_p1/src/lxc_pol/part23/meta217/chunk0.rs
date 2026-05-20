//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 863/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk863<F: Float>(t3069: F, t4669: F, t1612: F, t3082: F, t1606: F, t698: F, t973: F, t1043: F, t2770: F, t10277: F, t3061: F, t10216: F, t10969: F) -> (F, F, F, F, F, F) {
    let t13995 = t4669 * t3069;
    let t14117 = t1612 * t3082;
    let t14159 = t698 * t1606;
    let t14160 = t973 * t14159;
    let t14164 = t1043 * t2770;
    let t14172 = t3061 * t10277;
    let t14187 = t10969 * t10216;
    (t13995, t14117, t14160, t14164, t14172, t14187)
}
