//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta601 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2035;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2036;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta601<F: Float>(t23083: F, t23086: F, t23138: F, t6604: F, t6606: F, t22690: F, t2627: F, t236: F, t2631: F, t23109: F, t2632: F, t10024: F, t1899: F, t23110: F, t232: F, t23116: F, t838: F, t2693: F, t6609: F, t213: F, t6589: F, t9223: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t81909, t81911, t81912, t81914, t81915, t81918, t81920) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2035::<F>(t23083, t23086, t23138, t6604, t6606, t22690, t2627, t236, t2631, t23109, t2632, t10024, t1899);
        let (t81921, t81924, t81926, t81928, t81933) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2036::<F>(t81920, t23109, t23110, t232, t81915, t23116, t838, t2693, t6609, t213, t6589, t9223);
    (t81909, t81911, t81912, t81914, t81918, t81921, t81924, t81926, t81928, t81933)
}
