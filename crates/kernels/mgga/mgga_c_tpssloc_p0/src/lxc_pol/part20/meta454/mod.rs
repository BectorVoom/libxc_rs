//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1909;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1910;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta454<F: Float>(t3403: F, t4857: F, t1155: F, t3395: F, t4861: F, t11285: F, t1694: F, t3377: F, t1683: F, t3333: F, t11303: F, t11310: F, t11415: F, t15050: F, t15053: F, t15056: F, t15059: F, t15063: F, t15066: F, t15070: F, t3357: F, t3401: F, t4802: F, t4824: F, t15139: F, t15162: F, t15213: F, t300: F, t3411: F, t4875: F, t14958: F, t14963: F, t14969: F, t14971: F, t15038: F, t15040: F, t15043: F, t15046: F, t15048: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15218, t15219, t15222, t15225, t15226, t15229, t15232) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1909::<F>(t3403, t4857, t1155, t3395, t4861, t11285, t1694, t3377, t1683, t3333, t11303, t11310, t11415, t15050, t15053, t15056, t15059, t15063, t15066, t15070, t3357, t3401, t4802, t4824);
        let (t15235, t15237, t15238) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1910::<F>(t15139, t15162, t15213, t15232, t300, t3411, t4875, t14958, t14963, t14969, t14971, t15038, t15040, t15043, t15046, t15048, t15050, t15053, t15056, t15059, t15063, t15066, t15070);
    (t15218, t15219, t15222, t15225, t15226, t15229, t15235, t15237, t15238)
}
