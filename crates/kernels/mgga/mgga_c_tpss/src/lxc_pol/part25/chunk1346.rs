//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1346/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1346<F: Float>(t1265: F, t1657: F, t18483: F, t18490: F, t18496: F, t18967: F, t19507: F, t19509: F, t19521: F, t19535: F, t19540: F, t20157: F, t20174: F, t20179: F, t20187: F, t20190: F, t20191: F, t20202: F, t20214: F, t21061: F, t21804: F, t21820: F, t4494: F, t5432: F, t5739: F, t5740: F, t5918: F, t5933: F, t6260: F, t6433: F, t65871: F, t66970: F, t67083: F, t69654: F, t69676: F, t69691: F, t69704: F, t69708: F) -> F {
    let t71662 = F::new(4.0) * t19509 * t20179 + F::new(2.0) * t5739 * t5740 * t21804 * t1265 - t21061 * t5933 - F::new(6.0) * t18483 * t21820 - F::new(6.0) * t5739 * t18490 * t5918 * t5432 - F::new(2.0) * t6260 * t20214 - F::new(2.0) * t19507 * t6433 - F::new(4.0) * t18496 * t18967 * t69691 - F::new(4.0) * t65871 * t20174 - F::new(4.0) * t65871 * t20187 + F::new(2.0) * t69654 * t20202 + F::new(4.0) * t18496 * t20190 * t69704 - F::new(4.0) * t19540 * t20190 * t69708 - F::new(4.0) * t18496 * t66970 * t19535 - F::new(4.0) * t69654 * t20191 - F::new(4.0) * t18496 * t66970 * t19521 - F::new(4.0) * t18496 * t18967 * t69676 - F::new(2.0) * t67083 * t1657 + F::new(4.0) * t20157 * t4494;
    t71662
}
