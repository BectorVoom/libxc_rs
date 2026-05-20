//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1721;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1722;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta417<F: Float>(t4034: F, t6535: F, t107: F, t240: F, t625: F, t656: F, t666: F, t2331: F, t63: F, t2332: F, t2358: F, t6530: F, t109: F) -> (F, F, F, F, F, F) {
        let (t22467, t22469, t22470, t22471, t22472, t22473, t22474, t22476) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1721::<F>(t4034, t6535, t107, t240, t625, t656, t666, t2331, t63, t2332, t2358, t6530);
        let t22479 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1722::<F>(t109, t22469, t22472, t22474, t22476);
    (t22467, t22469, t22470, t22471, t22473, t22479)
}
