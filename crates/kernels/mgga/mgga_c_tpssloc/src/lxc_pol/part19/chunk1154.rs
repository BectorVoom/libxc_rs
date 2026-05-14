//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1154/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1154<F: Float>(t40672: F, t40705: F, t40724: F, t40756: F, t40791: F, t40819: F, t41591: F, t41603: F, t10647: F, t892: F, t914: F, t10650: F, t2837: F, t2784: F, t2841: F, t2845: F) -> (F, F, F, F) {
    let t41606 = t40672 + t40705 + t40724 + t40756 + t40791 + t40819 + t41591 + t41603;
    let t41618 = t10647 * t892;
    let t41620 = 4.0 * t41618 * t914;
    let t41622 = 6.0 * t10650 * t2837;
    let t41623 = t2784 * t2841;
    let t41625 = 0.96491876992155210402e2 * t41623 * t2845;
    (t41606, t41620, t41622, t41625)
}
