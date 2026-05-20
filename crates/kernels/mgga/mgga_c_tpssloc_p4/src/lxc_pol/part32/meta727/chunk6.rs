//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2359/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2359<F: Float>(t1458: F, t20127: F, t2314: F, t27858: F, t27863: F, t29501: F, t29848: F, t4034: F, t4077: F, t652: F, t671: F, t7266: F, t97820: F, t97829: F, t97831: F, t97833: F, t97835: F, t97836: F, t97839: F, t97842: F, t97844: F, t97846: F, t97848: F, t97850: F, t97854: F) -> F {
    let t105062 = -F::new(4.0) * t1458 * t27858 * t652 - F::new(2.0) * t29848 * t652 * t671 - F::new(2.0) * t20127 * t7266 - F::new(4.0) * t2314 * t29501 - F::new(4.0) * t27863 * t4077 - F::new(4.0) * t29501 * t4034 + t97820 - t97829 - t97831 - t97833 + t97835 - t97836 + t97839 + t97842 - t97844 - t97846 - t97848 - t97850 - t97854;
    t105062
}
