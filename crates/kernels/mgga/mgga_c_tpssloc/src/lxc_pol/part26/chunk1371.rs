//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1371/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1371<F: Float>(t10401: F, t24739: F, t3610: F, t3624: F, t24740: F, t3604: F, t11838: F, t7310: F, t11841: F, t11496: F, t11670: F, t11674: F, t11680: F, t11684: F, t11688: F, t11694: F, t11845: F, t2134: F, t24741: F, t3580: F, t460: F, t7320: F) -> F {
    let t86323 = t24739 * t10401;
    let t86324 = t3610 * t86323;
    let t86327 = t3624 * t86323;
    let t86330 = t3604 * t24740;
    let t86341 = t7310 * t11838;
    let t86343 = t7310 * t11841;
    let t86347 = F::new(5.0) / F::new(2304.0) * t24741 * t11670 - t24741 * t11684 / F::new(768.0) - t86324 * t11680 / F::new(384.0) + t86327 * t11694 / F::new(768.0) - t86330 * t3580 / F::new(384.0) - t24741 * t11674 / F::new(768.0) - t24741 * t11688 / F::new(384.0) - F::new(0.10093189023535097714e-3) * t2134 * t11496 * t460 * t7320 - t86341 / F::new(288.0) - t86343 / F::new(144.0) - t7310 * t11845 / F::new(288.0);
    t86347
}
