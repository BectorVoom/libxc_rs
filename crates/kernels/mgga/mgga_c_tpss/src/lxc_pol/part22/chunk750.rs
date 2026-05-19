//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 750/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk750<F: Float>(t3904: F, t912: F, t1448: F, t2618: F, t2621: F, t903: F, t140: F, t1460: F, t925: F, t2697: F, t926: F, t3749: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3906 = F::cast_from(0.5848223622634646207e0_f64) * t912 * t3904;
    let t3907 = t2618 * t1448;
    let t3908 = t2621 * t903;
    let t3909 = t3907 * t3908;
    let t3911 = F::cast_from(0.17315859105681463759e2_f64) * t912 * t3909;
    let t3916 = t140 * t1460;
    let t3917 = t925 * t3916;
    let t3919 = t926 * t2697;
    let t3920 = t3919 * t3749;
    (t3906, t3907, t3908, t3909, t3911, t3916, t3917, t3919, t3920)
}
