//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2066;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2067;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2068;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta600<F: Float>(t23197: F, t6547: F, t23257: F, t6562: F, t794: F, t23012: F, t6568: F, t225: F, t23211: F, t23205: F, t82038: F, t23242: F, t81979: F, t1914: F, t40772: F, t23547: F, t381: F, t23310: F, t23384: F, t23460: F, t6686: F, t23396: F, t23326: F, t6712: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t82230, t82236, t82259, t82287, t82294, t82296) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2066::<F>(t23197, t6547, t23257, t6562, t794, t23012, t6568, t225, t23211, t23205, t82038, t23242, t81979);
        let (t82312, t82357, t82380, t82382) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2067::<F>(t1914, t40772, t23547, t381, t23310, t23384, t23460, t6686);
        let (t82400, t82402) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2068::<F>(t23384, t23396, t23326, t6712);
    (t82230, t82236, t82259, t82287, t82294, t82296, t82312, t82357, t82380, t82382, t82400, t82402)
}
