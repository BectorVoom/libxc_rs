//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1619/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1619<F: Float>(t11668: F, t19015: F, t18232: F, t3440: F, t1017: F, t6163: F, t1210: F, t1207: F, t11665: F, t11678: F, t1174: F, t11834: F, t1218: F, t15569: F, t15717: F, t15719: F, t15722: F, t15740: F, t18997: F, t19002: F, t19005: F, t19010: F, t3577: F, t4889: F, t4950: F, t4954: F, t4969: F, t5046: F, t6192: F) -> (F, F, F) {
    let t19016 = t11668 * t19015;
    let t19019 = t3440 * t18232;
    let t19024 = t6163 * t1017;
    let t19025 = t1210 * t19024;
    let t19026 = t1207 * t19025;
    let t19029 = t15569 * t4950 / F::cast_from(432.0_f64) - t11665 * t6192 / F::cast_from(2304.0_f64) + t4889 * t5046 / F::cast_from(54.0_f64) - t1174 * t18997 / F::cast_from(288.0_f64) - t11678 * t19002 / F::cast_from(1152.0_f64) + t11834 - t1174 * t19005 / F::cast_from(48.0_f64) + t4889 * t4969 / F::cast_from(27.0_f64) - t1174 * t19010 / F::cast_from(144.0_f64) - t15740 * t4954 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t3577 * t19016 + t1174 * t19019 / F::cast_from(216.0_f64) + t15717 / F::cast_from(1296.0_f64) - t15719 / F::cast_from(6912.0_f64) - t15722 + F::cast_from(19.0_f64) / F::cast_from(1728.0_f64) * t19026 * t1218;
    (t19016, t19024, t19029)
}
