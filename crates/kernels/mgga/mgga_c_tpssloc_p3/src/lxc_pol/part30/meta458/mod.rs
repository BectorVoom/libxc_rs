//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1732;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1733;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta458<F: Float>(t23122: F, t23124: F, t2617: F, t6620: F, t6619: F, t835: F, t812: F, t849: F, t1891: F, t9223: F, t213: F, t1895: F, t1887: F, t206: F, t22715: F, t242: F, t6612: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23125, t23127, t23132, t23133) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1732::<F>(t23122, t23124, t2617, t6620, t6619, t835, t812);
        let (t23134, t23138, t23141, t23143, t23144, t23145, t23146) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1733::<F>(t23133, t849, t1891, t9223, t213, t1895, t1887, t206, t22715, t242, t6612, t812);
    (t23125, t23127, t23132, t23133, t23134, t23138, t23141, t23143, t23144, t23145, t23146)
}
