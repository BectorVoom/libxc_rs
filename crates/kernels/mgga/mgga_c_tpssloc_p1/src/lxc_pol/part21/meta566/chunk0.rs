//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2274/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2274<F: Float>(t19153: F, t6252: F, t11889: F, t1215: F, t5079: F, t6260: F, t11888: F, t11904: F, t11907: F, t11914: F, t1244: F, t15027: F, t15032: F, t15245: F, t1756: F, t19123: F, t19129: F, t19131: F, t19139: F, t19142: F, t19146: F, t3604: F, t3610: F, t3624: F, t5064: F, t5069: F, t5080: F, t5084: F, t6253: F, t6261: F, t6263: F) -> (F, F, F, F, F) {
    let t19154 = t6252 * t19153;
    let t19156 = t11889 * t1215;
    let t19157 = t6252 * t19156;
    let t19160 = t6260 * t5079;
    let t19164 = -F::cast_from(6.0_f64) * t11888 * t19157 + F::cast_from(2.0_f64) * t11904 * t6253 - t11907 * t6263 + t11914 * t19154 + t1244 * t19129 + F::cast_from(4.0_f64) * t15027 * t5069 + F::cast_from(2.0_f64) * t15032 * t1756 - F::cast_from(2.0_f64) * t15245 * t5080 + F::cast_from(2.0_f64) * t19123 * t3610 - F::cast_from(2.0_f64) * t19131 * t3624 - F::cast_from(2.0_f64) * t19139 * t3624 + F::cast_from(4.0_f64) * t19142 * t3610 - t19146 * t3624 - t19160 * t3624 + t3604 * t6261 + F::cast_from(2.0_f64) * t5064 * t5084;
    (t19154, t19156, t19157, t19160, t19164)
}
