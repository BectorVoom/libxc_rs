//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1998;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1999;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta605<F: Float>(t22715: F, t6887: F, t6970: F, t12225: F, t22641: F, t22690: F, t6969: F, t268: F, t547: F, t6559: F, t22724: F, t22927: F, t22642: F, t22643: F, t6907: F, t22644: F, t81152: F, t6891: F, t1372: F, t212: F, t6890: F, t1988: F, t81071: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t81186, t81187, t81195, t81197, t81228) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1998::<F>(t22715, t6887, t6970, t12225, t22641, t22690, t6969, t268, t547, t6559);
        let (t81264, t81267, t81282, t81284, t81311) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1999::<F>(t22724, t22927, t22642, t22643, t6907, t22644, t81152, t6891, t81195, t1372, t212, t6890);
        let (t81318, t81326) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2000::<F>(t1988, t81071, t225, t22643);
    (t81186, t81187, t81195, t81197, t81228, t81264, t81267, t81282, t81284, t81311, t81318, t81326)
}
