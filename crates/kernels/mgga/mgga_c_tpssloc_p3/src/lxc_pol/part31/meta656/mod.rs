//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta656 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1938;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1939;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta656<F: Float>(t23133: F, t5628: F, t23041: F, t5614: F, t1512: F, t87261: F, t16944: F, t25119: F, t841: F, t23083: F, t28372: F, t28395: F, t81782: F, t81783: F, t5587: F, t81803: F, t87295: F, t23097: F, t232: F, t67793: F, t815: F, t2628: F, t5585: F, t776: F, t13228: F, t4233: F, t6605: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t98733, t98736, t98738, t98744, t98746, t98750) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1938::<F>(t23133, t5628, t23041, t5614, t1512, t87261, t16944, t25119, t841, t23083, t28372, t28395, t81782, t81783);
        let (t98752, t98754, t98758, t98762, t98766) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1939::<F>(t5587, t81803, t1512, t87295, t23097, t232, t67793, t815, t2628, t5585, t776, t13228, t4233, t6605);
    (t98733, t98736, t98738, t98744, t98746, t98750, t98752, t98754, t98758, t98762, t98766)
}
