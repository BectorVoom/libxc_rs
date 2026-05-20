//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1961/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1961<F: Float>(t26959: F, t6486: F, t1860: F, t26024: F, t7031: F, t2032: F, t23963: F, t26016: F, t84180: F, t84216: F, t84242: F, t84248: F, t84270: F, t84280: F, t84283: F, t84285: F, t90072: F, t90121: F, t90141: F) -> F {
    let t92031 = F::new(16.0) / F::new(9.0) * t6486 * t26959;
    let t92034 = F::new(16.0) / F::new(9.0) * t1860 * t7031 * t26024;
    let t92039 = F::new(20.0) / F::new(3.0) * t26016 * t84180 + t90121 * t2032 / F::new(3.0) - F::new(880.0) / F::new(27.0) * t84242 - F::new(352.0) / F::new(27.0) * t84248 - F::new(70.0) * t84216 * t90141 - F::new(8.0) / F::new(9.0) * t84270 - t84280 - t92031 - t92034 + F::new(16.0) / F::new(9.0) * t84283 + F::new(176.0) / F::new(27.0) * t84285 + F::new(20.0) * t23963 * t90072;
    t92039
}
