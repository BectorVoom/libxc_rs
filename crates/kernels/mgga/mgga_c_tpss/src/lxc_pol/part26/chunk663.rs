//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 663/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk663<F: Float>(t3894: F, t914: F, t1457: F, t2629: F, t1448: F, t2593: F, t905: F, t912: F, t3882: F, t895: F, t904: F, t2618: F, t2621: F, t903: F, t140: F, t1460: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3896 = 0.5848223622634646207e0 * t3894 * t914;
    let t3898 = 0.5848223622634646207e0 * t2629 * t1457;
    let t3899 = t2593 * t1448;
    let t3900 = t3899 * t905;
    let t3902 = 0.11696447245269292414e1 * t912 * t3900;
    let t3904 = t895 * t3882 * t904;
    let t3906 = 0.5848223622634646207e0 * t912 * t3904;
    let t3907 = t2618 * t1448;
    let t3908 = t2621 * t903;
    let t3909 = t3907 * t3908;
    let t3911 = 0.17315859105681463759e2 * t912 * t3909;
    let t3916 = t140 * t1460;
    (t3896, t3898, t3899, t3900, t3902, t3904, t3906, t3907, t3908, t3909, t3911, t3916)
}
