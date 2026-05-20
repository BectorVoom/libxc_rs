//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1729;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1730;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta376<F: Float>(t2427: F, t4202: F, t9869: F, t2655: F, t4205: F, t12914: F, t12922: F, t12926: F, t12927: F, t12928: F, t12934: F, t12942: F, t12944: F, t12947: F, t9724: F, t9780: F, t9789: F, t9863: F, t1462: F, t9912: F, t9871: F, t4101: F, t9880: F, t2528: F, t4199: F, t2663: F, t4211: F, t9793: F, t9797: F, t9820: F, t9824: F, t9876: F, t9884: F, t9887: F, t9890: F, t9894: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13095, t13096, t13098, t13099) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1729::<F>(t2427, t4202, t9869, t2655, t4205, t12914, t12922, t12926, t12927, t12928, t12934, t12942, t12944, t12947, t9724, t9780, t9789, t9863);
        let (t13102, t13103, t13105, t13106, t13108, t13110, t13111) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1730::<F>(t1462, t9912, t9871, t2427, t4101, t9880, t2528, t4199, t2663, t4211, t9793, t9797, t9820, t9824, t9876, t9884, t9887, t9890, t9894);
    (t13095, t13096, t13098, t13099, t13102, t13103, t13105, t13106, t13108, t13110, t13111)
}
