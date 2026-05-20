//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta700 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2528;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2529;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta700<F: Float>(t1597: F, t341: F, t10245: F, t13847: F, t2986: F, t13931: F, t2987: F, t135: F, t13933: F, t973: F, t13532: F, t13784: F, t10213: F, t134: F, t344: F, t13537: F, t4509: F, t4540: F, t13797: F, t10186: F, t13848: F, t13780: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48184, t48189, t48191, t48207, t48210) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2528::<F>(t1597, t341, t10245, t13847, t2986, t13931, t2987, t135, t13933, t973, t13532, t13784);
        let (t48213, t48215, t48217, t48221, t48233, t48242) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2529::<F>(t10213, t134, t344, t13537, t2986, t4509, t4540, t13797, t1597, t10186, t13848, t13780);
    (t48184, t48189, t48191, t48207, t48210, t48213, t48215, t48217, t48221, t48233, t48242)
}
