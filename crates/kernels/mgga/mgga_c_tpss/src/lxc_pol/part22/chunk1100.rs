//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1100/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1100<F: Float>(t1062: F, t12066: F, t1072: F, t4155: F, t1535: F, t2998: F, t1054: F, t1082: F, t11970: F, t11973: F, t11975: F, t11978: F, t11980: F, t11982: F, t1531: F, t2925: F, t2994: F, t3002: F, t4143: F, t4158: F, t9414: F) -> F {
    let t12067 = t12066 * t1062;
    let t12070 = t4155 * t1072;
    let t12075 = t1535 * t2998;
    let t12078 = -t11970 - t11973 - t11975 - t11978 - t11980 - t11982 + F::new(1.0) * t9414 * t1531 + F::new(2.0) * t2925 * t4143 + F::new(1.0) * t1054 * t12067 + F::cast_from(0.11696447245269292414e1_f64) * t12070 * t1082 + F::cast_from(0.5848223622634646207e0_f64) * t4158 * t2994 + F::cast_from(0.17315859105681463759e2_f64) * t12075 * t3002;
    t12078
}
