//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2600/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2600<F: Float>(t3577: F, t44951: F, t4953: F, t11677: F, t15245: F, t1174: F, t14753: F, t3431: F, t14744: F, t11651: F, t15438: F, t1227: F, t13969: F, t15540: F) -> (F, F, F, F, F, F) {
    let t52758 = t3577 * t44951 * t4953;
    let t52766 = t15245 * t11677;
    let t52773 = t1174 * t3431 * t14753;
    let t52776 = t1174 * t3431 * t14744;
    let t52781 = t15438 * t11651;
    let t52792 = t1227 * t13969 * t15540;
    (t52758, t52766, t52773, t52776, t52781, t52792)
}
