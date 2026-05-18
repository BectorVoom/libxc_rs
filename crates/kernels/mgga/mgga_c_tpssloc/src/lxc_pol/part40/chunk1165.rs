//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1165/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1165<F: Float>(t18215: F, t4900: F, t11570: F, t5392: F, t11569: F, t1180: F, t15284: F, t15287: F, t15300: F, t15307: F, t18321: F, t18443: F, t18447: F, t18452: F, t18455: F, t18458: F, t18460: F, t3447: F, t4889: F, t4937: F) -> F {
    let t18466 = t4900 * t18215;
    let t18469 = t11570 * t5392;
    let t18470 = t11569 * t18469;
    let t18473 = -t15284 - t15287 - F::new(0.86419753086419753084e-3) * t3447 * t18443 + F::new(0.18518518518518518518e-3) * t18447 + F::new(0.44444444444444444444e-2) * t4889 * t4937 - F::new(0.18518518518518518518e-3) * t18452 - F::new(0.9259259259259259259e-4) * t18455 + F::new(0.12345679012345679012e-3) * t18458 + F::new(0.49382716049382716047e-3) * t18460 - F::new(0.27160493827160493827e-2) * t18321 * t1180 + F::new(0.12345679012345679012e-3) * t15300 + F::new(0.49382716049382716047e-3) * t15307 + F::new(0.74074074074074074072e-3) * t3447 * t18466 - F::new(0.37037037037037037036e-3) * t3447 * t18470;
    t18473
}
