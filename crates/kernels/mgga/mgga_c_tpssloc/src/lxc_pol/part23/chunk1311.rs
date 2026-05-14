//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1311/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1311<F: Float>(t11516: F, t11547: F, t1174: F, t1177: F, t1178: F, t1717: F, t18321: F, t29614: F, t3440: F, t457: F, t460: F, t4934: F, t52281: F, t6138: F, t6141: F, t6147: F, t73113: F, t73523: F, t73535: F, t73541: F, t75836: F, t75912: F, t78596: F, t78607: F, t974: F) -> (F,) {
    let t78634 = -0.50699588477366255142e-1 * t73523 - 0.41152263374485596707e-3 * t52281 + 0.15209876543209876543e0 * t73113 * t1717 - 0.48888888888888888888e-1 * t18321 * t6141 - 0.83333333333333333332e-3 * t1174 * t974 * t457 * (t78596 + t78607) * t460 + 0.13333333333333333332e-1 * t1174 * t3440 * t11547 * t75836 - 0.66666666666666666664e-2 * t1174 * t1177 * t11516 * t75836 - 0.49999999999999999999e-2 * t1174 * t4934 * t29614 * t6138 - 0.27777777777777777777e-3 * t1174 * t1177 * t1178 * t75912 + 0.11111111111111111111e-2 * t73535 - 0.22222222222222222221e-2 * t73541 - 0.48888888888888888888e-1 * t18321 * t6147;
    (t78634,)
}
