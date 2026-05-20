//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta296 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1458;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta296<F: Float>(t13847: F, t2990: F, t2986: F, t2987: F, t4540: F, t2989: F, t3966: F, t2960: F, t4506: F, t10224: F, t1592: F, t973: F) -> (F, F, F, F, F, F) {
        let (t13850, t13851, t13861, t13893, t13895, t13896) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1458::<F>(t13847, t2990, t2986, t2987, t4540, t2989, t3966, t2960, t4506, t10224, t1592, t973);
    (t13850, t13851, t13861, t13893, t13895, t13896)
}
