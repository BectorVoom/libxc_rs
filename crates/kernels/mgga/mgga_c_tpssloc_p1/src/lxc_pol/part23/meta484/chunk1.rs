//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1473/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1473<F: Float>(t1174: F, t1177: F, t1196: F, t1227: F, t1735: F, t18321: F, t21758: F, t22129: F, t22133: F, t22137: F, t22197: F, t22258: F, t3560: F, t3577: F, t45128: F, t4582: F, t4889: F, t4987: F, t5024: F, t6184: F, t6188: F, t73076: F, t75847: F, t75912: F, t77621: F, t78043: F, t78047: F, t974: F) -> F {
    let t79320 = -F::cast_from(5.0_f64) / F::cast_from(216.0_f64) * t5024 * t22197 + t5024 * t22258 / F::cast_from(36.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t1227 * t4582 * t4987 * t77621 - t1174 * t1177 * t78047 / F::cast_from(36.0_f64) - t1174 * t1177 * t78043 / F::cast_from(8.0_f64) - F::cast_from(11.0_f64) / F::cast_from(54.0_f64) * t18321 * t6184 - F::cast_from(11.0_f64) / F::cast_from(27.0_f64) * t18321 * t6188 + t1174 * t974 * t3560 * t75847 / F::cast_from(72.0_f64) - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t4889 * t22137 + t4889 * t22129 / F::cast_from(27.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4889 * t22133 - t1174 * t974 * t1196 * t75912 / F::cast_from(288.0_f64) - F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t73076 - F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t3577 * t45128 * t1735 * t21758;
    t79320
}
