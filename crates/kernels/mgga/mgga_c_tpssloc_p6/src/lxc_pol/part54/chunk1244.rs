//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1244/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1244<F: Float>(t2165: F, t7801: F, t1459: F, t1774: F, t1849: F, t32350: F, t32676: F, t32679: F, t33554: F, t33555: F, t33556: F, t34137: F, t34146: F, t510: F, t574: F, t652: F, t7042: F, t7943: F, t7989: F, t8329: F, t8690: F, t8829: F, t8840: F) -> (F, F) {
    let t34150 = t2165 * t7801;
    let t34157 = -F::cast_from(2.0_f64) * t1459 * t32350 - t1774 * t8829 + t1849 * t8840 - t34137 * t510 + t34146 * t574 - F::cast_from(2.0_f64) * t34150 * t652 - F::cast_from(2.0_f64) * t7042 * t7989 - t7943 * t8690 - t32676 - t32679 - t33554 - t33555 - t33556 - t8329;
    (t34150, t34157)
}
