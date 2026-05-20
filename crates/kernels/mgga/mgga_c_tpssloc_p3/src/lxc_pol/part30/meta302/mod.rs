//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1321;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta302<F: Float>(t4509: F, t984: F, t2770: F, t343: F, t2775: F, t2769: F, t40: F, t698: F, t986: F, t973: F, t241: F, t625: F) -> (F, F, F, F, F, F) {
        let (t10235, t10236, t10254, t10277, t10287, t10292) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1321::<F>(t4509, t984, t2770, t343, t2775, t2769, t40, t698, t986, t973, t241, t625);
    (t10235, t10236, t10254, t10277, t10287, t10292)
}
