//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1458/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1458<F: Float>(t671: F, t8828: F, t104977: F, t117533: F, t120876: F, t120877: F, t120878: F, t120881: F, t120885: F, t1459: F, t19456: F, t2040: F, t27145: F, t27863: F, t32318: F, t32350: F, t4028: F, t4037: F, t7050: F, t7408: F, t7787: F, t8690: F, t8835: F) -> (F, F) {
    let t124715 = t8828 * t671;
    let t124726 = -F::new(2.0) * t104977 * t2040 - F::new(2.0) * t117533 * t1459 - F::new(2.0) * t124715 * t1459 - F::new(2.0) * t19456 * t8835 + t27145 * t8690 - F::new(2.0) * t27863 * t7050 - F::new(2.0) * t32318 * t4028 - F::new(2.0) * t32350 * t4037 - t7408 * t7787 + t120876 - t120877 - t120878 - t120881 + t120885;
    (t124715, t124726)
}
