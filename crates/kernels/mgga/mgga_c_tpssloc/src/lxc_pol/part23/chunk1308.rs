//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1308/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1308<F: Float>(t1174: F, t1177: F, t1714: F, t18321: F, t22032: F, t22047: F, t22052: F, t22082: F, t3440: F, t3441: F, t3455: F, t44487: F, t44621: F, t44622: F, t460: F, t4889: F, t4934: F, t6120: F, t65002: F, t65023: F, t73491: F, t75836: F, t75847: F) -> (F,) {
    let t78545 = -t44487 - 0.19753086419753086419e-2 * t65002 + 0.92181069958847736624e-2 * t4889 * t22082 + 0.28806584362139917695e-2 * t1174 * t44621 * t44622 * t75836 - 0.59259259259259259257e-2 * t65023 + 0.14814814814814814815e-2 * t73491 - 0.33333333333333333332e-2 * t1174 * t4934 * t22032 * t1714 * t460 + 0.11111111111111111111e-2 * t1174 * t3440 * t3441 * t75847 - 0.16666666666666666666e-2 * t1174 * t1177 * t3455 * t75847 + 0.21728395061728395061e-1 * t18321 * t6120 + 0.26666666666666666666e-1 * t4889 * t22052 + 0.29629629629629629628e-2 * t4889 * t22047;
    (t78545,)
}
