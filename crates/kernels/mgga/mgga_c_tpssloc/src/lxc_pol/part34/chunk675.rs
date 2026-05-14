//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 675/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk675<F: Float>(t9490: F, t9493: F, t172: F, t9720: F, t2512: F, t746: F, t9711: F, t9689: F, t9692: F, t9695: F, t9698: F, t9702: F, t9704: F, t9706: F, t9709: F, t702: F) -> (F, F, F, F, F) {
    let t9759 = t9490 * t9493;
    let t9762 = t172 * t9720;
    let t9763 = t9490 * t2512;
    let t9766 = t9711 * t746;
    let t9777 = -0.25319e1 * t9689 + 0.16879333333333333333e1 * t9692 - 0.19692555555555555555e1 * t9695 - 0.93011851851851851854e0 * t9698 + 0.13651666666666666667e0 * t9702 - 0.27303333333333333333e0 * t9704 - 0.3185388888888888889e0 * t9706 - 0.36514074074074074075e0 * t9709;
    let t9778 = t9777 * t702;
    (t9759, t9762, t9763, t9766, t9778)
}
