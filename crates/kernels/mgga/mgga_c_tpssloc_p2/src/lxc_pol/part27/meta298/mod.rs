//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1357;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta298<F: Float>(t2970: F, t2995: F, t973: F, t2769: F, t40: F, t698: F, t986: F, t135: F, t3010: F, t241: F, t625: F, t281: F, t283: F) -> (F, F, F, F, F, F) {
        let (t10274, t10277, t10287, t10290, t10292, t10294) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1357::<F>(t2970, t2995, t973, t2769, t40, t698, t986, t135, t3010, t241, t625, t281, t283);
    (t10274, t10277, t10287, t10290, t10292, t10294)
}
