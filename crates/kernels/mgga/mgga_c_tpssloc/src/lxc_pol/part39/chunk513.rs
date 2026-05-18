//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 513/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk513<F: Float>(t1851: F, t3: F, t1401: F, t1458: F, t577: F, t193: F, t202: F, t154: F, t204: F, t119: F, t210: F, t201: F, t243: F) -> (F, F, F, F, F, F) {
    let t1852 = t3 * t1851;
    let t1858 = F::new(0.45e1) * t1851 * t577 + F::new(0.135e2) * t1401 * t1458;
    let t1877 = t193 * t202;
    let t1878 = t204 * t154;
    let t1887 = t210 * t119;
    let t1891 = F::new(1.0) / t243 / t201;
    (t1852, t1858, t1877, t1878, t1887, t1891)
}
