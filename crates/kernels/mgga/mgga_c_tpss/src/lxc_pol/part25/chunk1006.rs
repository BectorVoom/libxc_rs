//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1006/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1006<F: Float>(t14731: F, t14800: F, t14841: F, t14882: F, t294: F, t2618: F, t4939: F, t3908: F, t912: F, t4918: F, t914: F, t11222: F, t1457: F, t242: F, t2675: F, t4994: F) -> (F, F, F, F, F) {
    let t14885 = t294 * (t14731 + t14800 + t14841 + t14882);
    let t14886 = t2618 * t4939;
    let t14887 = t14886 * t3908;
    let t14889 = 0.17315859105681463759e2 * t912 * t14887;
    let t14890 = t294 * t4918;
    let t14892 = 0.5848223622634646207e0 * t14890 * t914;
    let t14894 = 0.11696447245269292414e1 * t11222 * t1457;
    let t14901 = t242 * t2675 * t4994;
    (t14885, t14889, t14892, t14894, t14901)
}
