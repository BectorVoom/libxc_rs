//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1098/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1098<F: Float>(t218: F, t32849: F, t1528: F, t1912: F, t25188: F, t25348: F, t259: F, t30655: F, t30662: F, t30741: F, t30748: F, t32865: F, t32869: F, t32877: F, t32878: F, t4147: F, t6627: F, t7538: F, t8363: F) -> (F, F) {
    let t32880 = t218 * t32849;
    let t32884 = -t1528 * t30741 - F::new(2.0) * t1912 * t25188 - F::new(2.0) * t1912 * t25348 + t259 * t32878 + t259 * t32880 - t4147 * t8363 - F::new(2.0) * t6627 * t7538 - t30655 + t30662 + t30748 + t32865 - t32869 - t32877;
    (t32880, t32884)
}
