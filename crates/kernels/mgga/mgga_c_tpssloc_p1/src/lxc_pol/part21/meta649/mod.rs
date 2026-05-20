//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta649 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2444;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta649<F: Float>(t10224: F, t2995: F, t973: F, t10225: F, t2960: F, t10213: F, t135: F, t41961: F, t697: F, t976: F, t984: F, t2986: F, t2990: F) -> (F, F, F, F, F, F, F) {
        let (t42962, t42968, t42972, t43002, t43052, t43053, t43055) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2444::<F>(t10224, t2995, t973, t10225, t2960, t10213, t135, t41961, t697, t976, t984, t2986, t2990);
    (t42962, t42968, t42972, t43002, t43052, t43053, t43055)
}
