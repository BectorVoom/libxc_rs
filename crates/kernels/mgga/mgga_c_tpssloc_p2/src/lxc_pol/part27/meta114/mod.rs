//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta114 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk692;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk693;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta114<F: Float>(t207: F, t215: F, t2559: F, t782: F, t786: F, t789: F, t59: F, t591: F, t795: F, t154: F, t244: F) -> (F, F, F, F, F, F) {
        let (t2562, t2563) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk692::<F>(t207, t215, t2559, t782, t786);
        let (t2564, t2566, t2569, t2570) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk693::<F>(t2563, t789, t59, t591, t207, t795, t154, t244);
    (t2562, t2563, t2564, t2566, t2569, t2570)
}
