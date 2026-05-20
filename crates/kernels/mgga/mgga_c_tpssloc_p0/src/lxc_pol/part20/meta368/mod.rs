//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1712;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta368<F: Float>(t12813: F, t510: F, t1458: F, t3652: F, t4098: F, t751: F, t2752: F, t4303: F, t172: F, t4095: F, t763: F, t1472: F, t2517: F, t40: F, t1409: F, t9427: F, t2433: F, t3966: F, t12606: F, t2244: F, t2250: F, t4080: F, t607: F, t73: F, t9438: F, t2440: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12835, t12841, t12850, t12854, t12858, t12860, t12861) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1712::<F>(t12813, t510, t1458, t3652, t4098, t751, t2752, t4303, t172, t4095, t763, t1472, t2517);
        let (t12862, t12865, t12873, t12874, t12877) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1713::<F>(t40, t1409, t9427, t2433, t3966, t12606, t2244, t2250, t4080, t607, t73, t9438, t2440, zeta_threshold);
    (t12835, t12841, t12850, t12854, t12858, t12860, t12861, t12862, t12865, t12873, t12874, t12877)
}
