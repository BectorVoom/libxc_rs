//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1298/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1298<F: Float>(t26504: F, t8607: F, t120172: F, t120719: F, t120721: F, t120728: F, t120730: F, t120735: F, t122706: F, t122708: F, t122710: F, t122713: F, t122732: F, t122734: F, t122735: F, t122736: F, t122737: F, t122738: F, t122739: F, t122740: F, t122754: F, t1774: F, t2039: F, t24999: F, t26875: F, t31700: F, t5107: F, t574: F, t7056: F, t8519: F, t90400: F, t96361: F) -> (F,) {
    let t122758 = t8607 * t26504;
    let t122761 = 6.0 * t120172 * t26875 - t120719 - t120721 - t120728 - t120730 - t120735 - t122706 - t122708 - t122710 - t122713 + (2.0 * t2039 * t90400 + 2.0 * t2039 * t96361 + 2.0 * t24999 * t7056 + 2.0 * t122732 + 2.0 * t122734 + 2.0 * t122735 + 2.0 * t122736 + 2.0 * t122737 + 2.0 * t122738 + 2.0 * t122739 + 2.0 * t122740 + t122754) * t574 + t122758 - t31700 * t1774 - t8519 * t5107;
    (t122761,)
}
