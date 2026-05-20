//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta561<F: Float>(t14536: F, t225: F, t14532: F, t14562: F, t14527: F, t14534: F, t16465: F, t12250: F, t1824: F, t1799: F, t3791: F, t3850: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t50625, t50632, t50653, t50690, t50703, t53866, t54014, t54068, t54153) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2004::<F>(t14536, t225, t14532, t14562, t14527, t14534, t16465, t12250, t1824, t1799, t3791, t3850);
    (t50625, t50632, t50653, t50690, t50703, t53866, t54014, t54068, t54153)
}
