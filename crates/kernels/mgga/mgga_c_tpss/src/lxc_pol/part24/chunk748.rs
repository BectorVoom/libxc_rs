//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 748/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk748<F: Float>(t294: F, t4946: F, t4919: F, t1457: F, t3894: F, t2593: F, t4923: F, t904: F, t912: F, t4939: F, t895: F, t2618: F, t2621: F, t2698: F, t4573: F, t926: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4947 = t294 * t4946;
    let t4949 = 0.19751673498613801407e-1 * t294 * t4919;
    let t4951 = 0.11696447245269292414e1 * t3894 * t1457;
    let t4953 = t2593 * t4923 * t904;
    let t4955 = 0.11696447245269292414e1 * t912 * t4953;
    let t4957 = t895 * t4939 * t904;
    let t4959 = 0.5848223622634646207e0 * t912 * t4957;
    let t4960 = t2618 * t4923;
    let t4961 = t4960 * t2621;
    let t4963 = 0.17315859105681463759e2 * t912 * t4961;
    let t4965 = t2698 * t4573;
    let t4966 = t926 * t4965;
    (t4947, t4949, t4951, t4953, t4955, t4957, t4959, t4960, t4961, t4963, t4965, t4966)
}
