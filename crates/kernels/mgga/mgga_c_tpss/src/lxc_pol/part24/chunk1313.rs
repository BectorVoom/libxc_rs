//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1313/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1313<F: Float>(t19766: F, t6134: F, t45241: F, t782: F, t4716: F, t226: F, t44994: F, t4764: F, t818: F, t4799: F, t10584: F, t3664: F, t1396: F, t14363: F, t1707: F, t1708: F, t18000: F, t18006: F, t18007: F, t19734: F, t19736: F, t19763: F, t19767: F, t19768: F, t19769: F, t19770: F, t19779: F, t19782: F, t19786: F, t19791: F, t21299: F, t2157: F, t228: F, t4783: F, t5562: F, t5565: F, t5571: F, t5583: F, t6130: F, t6146: F, t64016: F, t64060: F, t64163: F, t70006: F) -> (F,) {
    let t70039 = t6134 * t19766;
    let t70042 = t45241 * t782;
    let t70046 = t4716 * t782;
    let t70060 = t44994 * t226;
    let t70063 = t4764 * t818;
    let t70070 = t4799 * t782 * t226;
    let t70074 = t4716 * t818;
    let t70094 = t10584 * t3664;
    let t70098 = -4.0 * t70039 * t19770 + 6.0 * t19767 * t64163 * t70042 - 6.0 * t19767 * t19768 * t70046 - 4.0 * t19767 * t2157 * t6130 * t19769 - 2.0 * t19734 * t6146 + 4.0 * t19736 * t19786 - 4.0 * t64060 * t19763 + t19767 * t18007 * t70060 - 2.0 * t18006 * t18007 * t70063 + 2.0 * t70039 * t19782 - 2.0 * t18006 * t18007 * t70070 + 4.0 * t18006 * t19768 * t70074 - t1707 * t1708 * t228 * t70006 - 6.0 * t5571 * t18000 * t5562 * t4783 - t21299 * t5583 - 2.0 * t64016 * t1396 - 6.0 * t5565 * t14363 + 2.0 * t19736 * t19791 + 2.0 * t19736 * t19779 - 4.0 * t19767 * t19768 * t70094;
    (t70098,)
}
