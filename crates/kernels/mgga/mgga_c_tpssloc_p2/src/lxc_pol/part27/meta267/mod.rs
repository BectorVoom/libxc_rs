//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1280;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1281;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1282;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1283;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1284;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta267<F: Float>(t1603: F, t1945: F, t1409: F, t3: F, t1933: F, t1597: F, t343: F, t6734: F, t1615: F, t68: F, t360: F, t6744: F, t1611: F, t1941: F, t1607: F, t1618: F, t1622: F, t1935: F, t1937: F, t378: F, t6716: F, t6717: F, t6728: F, t6742: F, t6755: F, t6763: F, t6765: F, t349: F, t1634: F, t1955: F, t3174: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7569, t7573, t7574, t7577) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1280::<F>(t1603, t1945, t1409, t3, t1933, t1597, t343);
        let t7578 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1281::<F>(t6734, t7577);
        let (t7581, t7582, t7583) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1282::<F>(t1615, t68, t360, t6744);
        let (t7586, t7593) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1283::<F>(t1611, t1941, t1607, t1618, t1622, t1935, t1937, t378, t6716, t6717, t6728, t6742, t6755, t6763, t6765, t7574, t7578, t7583);
        let (t7594, t7600) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1284::<F>(t349, t7593, t1634, t1955, t3174);
    (t7569, t7573, t7574, t7577, t7578, t7581, t7582, t7583, t7586, t7593, t7594, t7600)
}
