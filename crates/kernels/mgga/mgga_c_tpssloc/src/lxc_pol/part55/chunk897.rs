//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 897/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk897<F: Float>(t1235: F, t7284: F, t1240: F, t1251: F, t2122: F, t1170: F, t7295: F, t2121: F, t461: F, t6729: F, t7324: F, t2131: F, t23508: F) -> (F, F, F, F, F, F) {
    let t24633 = t7284 * t1235;
    let t24637 = t1240 * t1251;
    let t24638 = t2122 * t24637;
    let t24645 = t1170 * t7295;
    let t24646 = t2121 * t24645;
    let t24649 = t6729 * t461;
    let t24650 = t7324 * t24649;
    let t24658 = t2131 * t23508;
    (t24633, t24637, t24638, t24646, t24650, t24658)
}
