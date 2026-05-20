//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta651 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2192;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2193;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta651<F: Float>(t4194: F, t5398: F, t607: F, t750: F, t32: F, t5519: F, t2517: F, t707: F, t16616: F, t2535: F, t16701: F, t2427: F, t13133: F, t4101: F, t2371: F, t17083: F, t225: F, t16805: F, t68: F, t16752: F, t252: F, t13396: F, t1499: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t57965, t57973, t57992, t58021, t58047) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2192::<F>(t4194, t5398, t607, t750, t32, t5519, t2517, t707, t16616, t2535, t16701, t2427);
        let (t58052, t58057, t58143, t58181, t58262, t58313) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2193::<F>(t13133, t4101, t16616, t2371, t17083, t225, t16805, t68, t16752, t252, t13396, t1499);
    (t57965, t57973, t57992, t58021, t58047, t58052, t58057, t58143, t58181, t58262, t58313)
}
