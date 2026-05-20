//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2006;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta563<F: Float>(t16463: F, t225: F, t16448: F, t12020: F, t1842: F, t16468: F, t16458: F, t16486: F, t3701: F, t112: F, t16506: F, t111: F, t5363: F) -> (F, F, F, F, F, F, F, F) {
        let (t55069, t55093, t55118, t55134, t55150, t55169, t55341, t55353) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2006::<F>(t16463, t225, t16448, t12020, t1842, t16468, t16458, t16486, t3701, t112, t16506, t111, t5363);
    (t55069, t55093, t55118, t55134, t55150, t55169, t55341, t55353)
}
