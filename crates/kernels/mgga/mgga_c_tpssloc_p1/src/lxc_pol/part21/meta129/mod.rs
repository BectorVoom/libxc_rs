//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta129 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk867;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk868;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta129<F: Float>(t3040: F, t3131: F, t1021: F, t248: F, t135: F, t999: F, t973: F, t2250: F, t998: F, t974: F, t2770: F, t2978: F, t2244: F) -> (F, F, F, F, F, F, F, F) {
        let t3132 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk867::<F>(t3040, t3131);
        let (t3134, t3139, t3140, t3142, t3143, t3146, t3147) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk868::<F>(t1021, t248, t3132, t135, t999, t973, t2250, t998, t974, t2770, t2978, t2244);
    (t3132, t3134, t3139, t3140, t3142, t3143, t3146, t3147)
}
