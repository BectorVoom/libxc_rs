//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1589/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1589<F: Float>(t1155: F, t6085: F, t3403: F, t6084: F, t4857: F, t4861: F, t11285: F, t6068: F, t11310: F, t11365: F, t15126: F, t15136: F, t15146: F, t15207: F, t18247: F, t18603: F, t18606: F, t18609: F, t3376: F, t3401: F, t4802: F, t4824: F, t4840: F, t4862: F) -> F {
    let t18612 = t6085 * t1155;
    let t18615 = t6084 * t3403;
    let t18616 = t18615 * t1155;
    let t18619 = t4861 * t4857;
    let t18622 = t6068 * t11285;
    let t18623 = t18622 * t1155;
    let t18630 = -F::cast_from(0.23392894490538584828e1_f64) * t15136 * t4840 + F::cast_from(0.34631718211362927517e2_f64) * t15126 * t4862 + F::cast_from(0.35089341735807877242e1_f64) * t3401 * t18603 - F::cast_from(0.23392894490538584828e1_f64) * t3376 * t18606 - F::cast_from(0.10389515463408878255e3_f64) * t11365 * t18609 - F::cast_from(0.11696447245269292414e1_f64) * t3376 * t18612 + F::cast_from(0.17315859105681463759e2_f64) * t3401 * t18616 + F::cast_from(0.34631718211362927518e2_f64) * t3401 * t18619 + F::cast_from(0.10254018858216406658e4_f64) * t11310 * t18623 + t18247 - F::cast_from(4.0_f64) * t15207 * t4802 + F::cast_from(0.64327917994770140268e2_f64) * t15146 * t4824;
    t18630
}
