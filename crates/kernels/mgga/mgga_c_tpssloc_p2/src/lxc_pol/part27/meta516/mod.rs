//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta516<F: Float>(t2775: F, t381: F, t3961: F, t25510: F, t1625: F, t362: F, t884: F, t6784: F, t6743: F, t7577: F, t6801: F, t1058: F, t23327: F, t23601: F, t23642: F, t23670: F, t25487: F, t25493: F, t25497: F, t25500: F, t25503: F, t25508: F, t3180: F, t6687: F, t6797: F, t7611: F, t7620: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25511, t25512, t25513, t25516, t25517, t25518, t25523, t25524, t25527) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1921::<F>(t2775, t381, t3961, t25510, t1625, t362, t884, t6784, t6743, t7577, t6801, t1058, t23327, t23601, t23642, t23670, t25487, t25493, t25497, t25500, t25503, t25508, t3180, t6687, t6797, t7611, t7620);
    (t25511, t25512, t25513, t25516, t25517, t25518, t25523, t25524, t25527)
}
