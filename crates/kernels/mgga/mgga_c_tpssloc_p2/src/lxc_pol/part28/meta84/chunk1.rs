//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 530/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk530<F: Float>(t1714: F, t457: F, t460: F, t974: F, t1173: F, t1174: F, t1706: F, t1710: F, t463: F) -> (F, F) {
    let t1715 = t457 * t1714;
    let t1716 = t1715 * t460;
    let t1717 = t974 * t1716;
    let t1720 = -F::cast_from(0.22222222222222222222e-2_f64) * t1706 * t463 + t1173 - F::cast_from(0.27777777777777777777e-3_f64) * t1174 * t1710 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t1717;
    (t1716, t1720)
}
