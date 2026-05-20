//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta606 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta606<F: Float>(t6802: F, t82713: F, t3158: F, t6796: F, t23665: F, t23674: F, t23600: F, t995: F, t23680: F, t23606: F, t225: F, t23494: F) -> (F, F, F, F, F, F, F, F) {
        let (t82714, t82716, t82717, t82734, t82736, t82737, t82739, t82750) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2078::<F>(t6802, t82713, t3158, t6796, t23665, t23674, t23600, t995, t23680, t23606, t225, t23494);
    (t82714, t82716, t82717, t82734, t82736, t82737, t82739, t82750)
}
