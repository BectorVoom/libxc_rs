//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1307/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1307<F: Float>(t29978: F, t3: F, t112: F, t8153: F, t111: F, t2186: F, t671: F, t8143: F, t2180: F, t2363: F, t12521: F, t12524: F, t1401: F, t16535: F, t2319: F, t29934: F, t3938: F, t3941: F, t577: F, t8161: F, t8166: F) -> (F, F, F, F, F, F) {
    let t29979 = t3 * t29978;
    let t29993 = t8153 * t112;
    let t29996 = t2186 * t111;
    let t30009 = t8143 * t671;
    let t30012 = t2180 * t2363;
    let t30017 = F::cast_from(0.45e1_f64) * t29978 * t577 + F::cast_from(27.0_f64) * t29993 * t671 + F::cast_from(27.0_f64) * t29996 * t2319 + F::cast_from(0.135e2_f64) * t8161 * t2363 + F::cast_from(0.135e2_f64) * t12521 * t2180 + F::cast_from(54.0_f64) * t12524 * t8166 + F::cast_from(27.0_f64) * t3938 * t8143 + F::cast_from(27.0_f64) * t16535 * t2180 + F::cast_from(54.0_f64) * t3941 * t30009 + F::cast_from(27.0_f64) * t3941 * t30012 + F::cast_from(0.135e2_f64) * t1401 * t29934;
    (t29979, t29993, t29996, t30009, t30012, t30017)
}
