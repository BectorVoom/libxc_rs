//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1045/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1045<F: Float>(t1210: F, t19024: F, t1207: F, t11665: F, t11678: F, t1174: F, t11834: F, t1218: F, t15569: F, t15717: F, t15719: F, t15722: F, t15740: F, t18997: F, t19002: F, t19005: F, t19010: F, t19016: F, t19019: F, t3577: F, t4889: F, t4950: F, t4954: F, t4969: F, t5046: F, t6192: F) -> (F,) {
    let t19025 = t1210 * t19024;
    let t19026 = t1207 * t19025;
    let t19029 = t15569 * t4950 / 432.0 - t11665 * t6192 / 2304.0 + t4889 * t5046 / 54.0 - t1174 * t18997 / 288.0 - t11678 * t19002 / 1152.0 + t11834 - t1174 * t19005 / 48.0 + t4889 * t4969 / 27.0 - t1174 * t19010 / 144.0 - t15740 * t4954 / 2304.0 + 5.0 / 6912.0 * t3577 * t19016 + t1174 * t19019 / 216.0 + t15717 / 1296.0 - t15719 / 6912.0 - t15722 + 19.0 / 1728.0 * t19026 * t1218;
    (t19029,)
}
