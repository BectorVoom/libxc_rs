//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta130 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk851;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta130<F: Float>(t135: F, t999: F, t973: F, t2250: F, t998: F, t974: F, t2770: F, t2978: F, t2244: F, t2775: F, t976: F, t1005: F, t1036: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t3139, t3140, t3142, t3143, t3147, t3148, t3152, t3153, t3156) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk851::<F>(t135, t999, t973, t2250, t998, t974, t2770, t2978, t2244, t2775, t976, t1005, t1036);
    (t3139, t3140, t3142, t3143, t3147, t3148, t3152, t3153, t3156)
}
