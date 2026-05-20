//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1799;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta402<F: Float>(t13779: F, t4343: F, t2986: F, t134: F, t2978: F, t344: F) -> (F, F, F, F) {
        let (t13780, t13782, t13783, t13784) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1799::<F>(t13779, t4343, t2986, t134, t2978, t344);
    (t13780, t13782, t13783, t13784)
}
