//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1794;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta466<F: Float>(t23204: F, t6555: F, t23164: F, t6572: F, t6562: F, t6624: F, t798: F, t1911: F, t2719: F, t10110: F, t2742: F, t6571: F) -> (F, F, F, F, F, F, F, F) {
        let (t23205, t23206, t23207, t23208, t23209, t23211, t23215, t23218) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1794::<F>(t23204, t6555, t23164, t6572, t6562, t6624, t798, t1911, t2719, t10110, t2742, t6571);
    (t23205, t23206, t23207, t23208, t23209, t23211, t23215, t23218)
}
