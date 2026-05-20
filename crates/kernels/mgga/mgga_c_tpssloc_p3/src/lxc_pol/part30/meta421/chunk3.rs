//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1612/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1612<F: Float>(t11668: F, t19015: F, t18232: F, t3440: F, t1017: F, t6163: F, t1210: F, t1207: F, t11665: F, t11678: F, t1174: F, t11834: F, t1218: F, t15569: F, t15717: F, t15719: F, t15722: F, t15740: F, t18997: F, t19002: F, t19005: F, t19010: F, t3577: F, t4889: F, t4950: F, t4954: F, t4969: F, t5046: F, t6192: F) -> F {
    let t19016 = t11668 * t19015;
    let t19019 = t3440 * t18232;
    let t19024 = t6163 * t1017;
    let t19025 = t1210 * t19024;
    let t19026 = t1207 * t19025;
    let t19029 = t15569 * t4950 / F::new(432.0) - t11665 * t6192 / F::new(2304.0) + t4889 * t5046 / F::new(54.0) - t1174 * t18997 / F::new(288.0) - t11678 * t19002 / F::new(1152.0) + t11834 - t1174 * t19005 / F::new(48.0) + t4889 * t4969 / F::new(27.0) - t1174 * t19010 / F::new(144.0) - t15740 * t4954 / F::new(2304.0) + F::new(5.0) / F::new(6912.0) * t3577 * t19016 + t1174 * t19019 / F::new(216.0) + t15717 / F::new(1296.0) - t15719 / F::new(6912.0) - t15722 + F::new(19.0) / F::new(1728.0) * t19026 * t1218;
    t19029
}
