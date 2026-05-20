//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 820/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk820<F: Float>(t1052: F, t1920: F, t1956: F, t388: F, t6687: F, t6771: F, t8377: F, t8381: F, t8392: F, t8397: F, t8407: F, t1958: F) -> (F, F) {
    let t8409 = F::cast_from(0.16449340668482264365e-1_f64) * t1920 * t8377 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t8381 + t8392 * t388 - F::new(2.0) * t6771 * t1956 + F::new(2.0) * t1052 * t8397 - t1052 * t8407;
    let t8413 = t1958 * t1958;
    (t8409, t8413)
}
