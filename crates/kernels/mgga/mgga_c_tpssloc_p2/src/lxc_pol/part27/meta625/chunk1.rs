//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2108/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2108<F: Float>(t16524: F, t23896: F, t12813: F, t1458: F, t7010: F, t84004: F, t86582: F, t86606: F, t86610: F, t86612: F, t86614: F, t86616: F, t86619: F, t86622: F, t86625: F, t86629: F, t86631: F, t86633: F, t86635: F, t86637: F) -> F {
    let t86639 = F::new(27.0) * t16524 * t23896;
    let t86640 = t86582 + t86606 + F::new(0.135e2) * t84004 * t1458 + t86610 + t86612 + t86614 + t86616 + t86619 + t86622 + t86625 + F::new(0.135e2) * t7010 * t12813 + t86629 + t86631 + t86633 + t86635 + t86637 + t86639;
    t86640
}
