//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1649;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta453<F: Float>(t24098: F, t24164: F, t533: F, t1390: F, t2095: F, t23857: F, t532: F, t7216: F, t6879: F, t193: F, t201: F, t2056: F, t2047: F, t2591: F, t23042: F, t23044: F, t23049: F, t23051: F, t23054: F, t23057: F, t23059: F, t23063: F, t23067: F, t23070: F, t23073: F, t23081: F, t23084: F, t23087: F, t23090: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t24165, t24166, t24167, t24169, t24175, t24176, t24191) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1649::<F>(t24098, t24164, t533, t1390, t2095, t23857, t532, t7216, t6879, t193, t201, t2056);
        let (t24200, t24217) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1650::<F>(t2047, t2591, t23042, t23044, t23049, t23051, t23054, t23057, t23059, t23063, t23067, t23070, t23073, t23081, t23084, t23087, t23090);
    (t24165, t24166, t24167, t24169, t24175, t24176, t24191, t24200, t24217)
}
