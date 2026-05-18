//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1059/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1059<F: Float>(t12512: F, t3: F, t112: F, t3931: F, t111: F, t1395: F, t2319: F, t671: F, t2363: F, t1401: F, t3938: F, t3941: F, t576: F, t577: F, t9416: F) -> (F, F, F, F, F, F) {
    let t12513 = t3 * t12512;
    let t12521 = t3931 * t112;
    let t12524 = t1395 * t111;
    let t12529 = t2319 * t671;
    let t12532 = t671 * t2363;
    let t12537 = F::new(0.45e1) * t12512 * t577 + F::new(0.405e2) * t12521 * t671 + F::new(81.0) * t12524 * t2319 + F::new(0.405e2) * t3938 * t2363 + F::new(27.0) * t576 * t12529 + F::new(81.0) * t3941 * t12532 + F::new(0.135e2) * t1401 * t9416;
    (t12513, t12521, t12524, t12529, t12532, t12537)
}
