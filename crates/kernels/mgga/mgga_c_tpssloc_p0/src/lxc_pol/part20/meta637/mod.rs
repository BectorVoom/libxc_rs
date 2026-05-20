//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2342;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2343;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2344;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta637<F: Float>(t1597: F, t43052: F, t2986: F, t2990: F, t10255: F, t13847: F, t10190: F, t13861: F, t13559: F, t13779: F, t10189: F, t4540: F, t42771: F, t4514: F, t43057: F, t13913: F, t2960: F, t4542: F, t698: F, t973: F, t10186: F, t13788: F, t13799: F, t13858: F, t13862: F, t13865: F, t13868: F, t13877: F) -> (F, F, F, F, F, F) {
        let (t48022, t48024, t48030, t48044, t48046) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2342::<F>(t1597, t43052, t2986, t2990, t10255, t13847, t10190, t13861, t13559, t13779, t10189, t4540);
        let (t48048, t48052, t48061, t48063, t48067, t48068) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2343::<F>(t2986, t2990, t48046, t42771, t4514, t43057, t13913, t2960, t4542, t698, t973, t10186, t13788);
        let t48076 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2344::<F>(t10186, t13799, t13858, t13862, t13865, t13868, t13877, t48052, t48061, t48063, t48067, t48068);
    (t48022, t48024, t48030, t48044, t48048, t48076)
}
