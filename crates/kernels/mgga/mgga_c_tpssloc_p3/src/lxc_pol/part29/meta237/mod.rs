//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta237 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1112;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1113;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta237<F: Float>(t1385: F, t1842: F, t3887: F, t3787: F, t68: F, t544: F, t1824: F, t562: F, t5250: F, t1825: F, t3901: F, t1380: F, t5287: F, t1338: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5325, t5326, t5333, t5334) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1112::<F>(t1385, t1842, t3887, t3787, t68, t544);
        let t5335 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1113::<F>(t1824, t562);
        let (t5336, t5339, t5341, t5343, t5344) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1114::<F>(t5250, t5335, t1825, t3901, t1380, t5287, t1338, t68, t544);
    (t5325, t5326, t5333, t5334, t5335, t5336, t5339, t5341, t5343, t5344)
}
