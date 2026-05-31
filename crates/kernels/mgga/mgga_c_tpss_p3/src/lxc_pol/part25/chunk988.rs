//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 988/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk988<F: Float>(t9883: F, t9887: F, t9890: F, t4397: F, t4533: F, t9907: F, t12744: F, t12749: F, t9957: F, t12742: F, t12754: F, t4532: F, t7954: F, t7960: F, t7972: F, t7975: F, t9886: F, t9900: F, t9903: F, t9906: F, t9954: F, t9956: F) -> (F, F, F, F, F, F, F, F) {
    let t13615 = F::cast_from(8.0_f64) * t9883;
    let t13616 = F::cast_from(0.17315859105681463759e2_f64) * t9887;
    let t13617 = F::cast_from(0.11696447245269292414e1_f64) * t9890;
    let t13618 = t4533 * t4397;
    let t13621 = F::cast_from(8.0_f64) * t9907;
    let t13622 = F::cast_from(0.21687162600603479684e-1_f64) * t12744;
    let t13623 = F::cast_from(40.0_f64) * t12749;
    let t13624 = F::cast_from(0.24415263074675393405e-3_f64) * t9957;
    let t13625 = F::cast_from(12.0_f64) * t13618 * t4532 + t12742 - t12754 - t13615 - t13616 + t13617 - t13621 + t13622 + t13623 + t13624 - t7954 - t7960 + t7972 + t7975 + t9886 + t9900 + t9903 - t9906 - t9954 + t9956;
    (t13615, t13616, t13617, t13621, t13622, t13623, t13624, t13625)
}
