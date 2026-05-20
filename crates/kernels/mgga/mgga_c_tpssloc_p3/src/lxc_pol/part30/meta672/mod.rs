//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta672 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2101;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta672<F: Float>(t87140: F, t87155: F, t87177: F, t87243: F, t87304: F, t87345: F, t87403: F, t87405: F, t87432: F, t87653: F, t87666: F, t87718: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t92513, t92516, t92543, t92597, t92633, t92652, t92676, t92677, t92689, t92781, t92794, t92817) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2101::<F>(t87140, t87155, t87177, t87243, t87304, t87345, t87403, t87405, t87432, t87653, t87666, t87718);
    (t92513, t92516, t92543, t92597, t92633, t92652, t92676, t92677, t92689, t92781, t92794, t92817)
}
