//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2059;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta525<F: Float>(t12189: F, t3770: F, t12313: F, t3726: F, t2559: F, t3732: F, t3766: F, t12214: F, t782: F, t12320: F, t154: F, t1995: F) -> (F, F, F, F, F, F, F) {
        let (t40008, t40012, t40018, t40019, t40021, t40022, t40024) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2059::<F>(t12189, t3770, t12313, t3726, t2559, t3732, t3766, t12214, t782, t12320, t154, t1995);
    (t40008, t40012, t40018, t40019, t40021, t40022, t40024)
}
