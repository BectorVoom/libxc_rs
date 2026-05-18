//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1326/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1326<F: Float>(t109: F, t110517: F, t110549: F, t110580: F, t110623: F, t111: F, t8240: F, t110240: F, t110253: F, t12521: F, t12524: F, t12813: F, t1401: F, t1458: F, t16524: F, t16535: F, t20173: F, t2180: F, t2319: F, t29934: F, t30009: F, t30180: F, t30250: F, t30253: F, t3938: F, t3941: F, t4072: F, t45560: F, t5371: F, t5376: F, t55405: F, t66940: F, t8143: F, t8161: F, t8230: F, t8251: F) -> (F, F) {
    let t110 = F::new(1.0) < t109;
    let t110626 = piecewise3::<f64>(t110, F::new(0.0), t110517 + t110549 + t110580 + t110623);
    let t110631 = t8240 * t111;
    let t110655 = F::new(0.135e2) * t5371 * t29934 + F::new(27.0) * t16535 * t8230 + F::new(27.0) * t45560 * t8251 + F::new(27.0) * t3938 * t30180 + F::new(0.135e2) * t1401 * t110626 + F::new(0.135e2) * t110253 * t1458 + F::new(27.0) * t110631 * t2319 + F::new(54.0) * t110240 * t5376 + F::new(54.0) * t20173 * t30250 + F::new(54.0) * t20173 * t30253 + F::new(54.0) * t16524 * t30009 + F::new(54.0) * t12524 * t30250 + F::new(27.0) * t55405 * t2180 + F::new(0.135e2) * t12521 * t8230 + F::new(0.135e2) * t8161 * t12813 + F::new(54.0) * t66940 * t8251 + F::new(54.0) * t3941 * t8143 * t4072;
    (t110626, t110655)
}
