//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 746/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk746<F: Float>(t294: F, t4946: F, t4919: F, t1457: F, t3894: F, t2593: F, t4923: F, t904: F, t912: F, t4939: F, t895: F, t2618: F) -> (F, F, F, F, F, F, F, F) {
    let t4947 = t294 * t4946;
    let t4949 = F::cast_from(0.19751673498613801407e-1_f64) * t294 * t4919;
    let t4951 = F::cast_from(0.11696447245269292414e1_f64) * t3894 * t1457;
    let t4953 = t2593 * t4923 * t904;
    let t4955 = F::cast_from(0.11696447245269292414e1_f64) * t912 * t4953;
    let t4957 = t895 * t4939 * t904;
    let t4959 = F::cast_from(0.5848223622634646207e0_f64) * t912 * t4957;
    let t4960 = t2618 * t4923;
    (t4947, t4949, t4951, t4953, t4955, t4957, t4959, t4960)
}
