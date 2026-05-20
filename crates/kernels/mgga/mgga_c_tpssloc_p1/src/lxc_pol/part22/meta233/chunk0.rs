//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1303/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1303<F: Float>(t731: F, t9751: F, t746: F, t9490: F, t172: F, t9489: F, t9493: F, t9720: F, t2512: F, t9711: F, t9689: F, t9692: F, t9695: F, t9698: F, t9702: F, t9704: F, t9706: F, t9709: F) -> (F, F, F, F, F, F, F, F) {
    let t9752 = t9751 * t731;
    let t9755 = t9490 * t746;
    let t9758 = t172 * t9489;
    let t9759 = t9490 * t9493;
    let t9762 = t172 * t9720;
    let t9763 = t9490 * t2512;
    let t9766 = t9711 * t746;
    let t9777 = -F::new(0.25319e1) * t9689 + F::cast_from(0.16879333333333333333e1_f64) * t9692 - F::cast_from(0.19692555555555555555e1_f64) * t9695 - F::cast_from(0.93011851851851851854e0_f64) * t9698 + F::cast_from(0.13651666666666666667e0_f64) * t9702 - F::cast_from(0.27303333333333333333e0_f64) * t9704 - F::cast_from(0.3185388888888888889e0_f64) * t9706 - F::cast_from(0.36514074074074074075e0_f64) * t9709;
    (t9752, t9755, t9758, t9759, t9762, t9763, t9766, t9777)
}
