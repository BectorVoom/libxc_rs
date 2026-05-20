//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1296/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1296<F: Float>(t1193: F, t27506: F, t24660: F, t8034: F, t24667: F, t24847: F, t64825: F, t974: F, t8067: F, t85660: F, t8070: F, t210: F, t24848: F, t27505: F) -> (F, F, F, F, F, F, F) {
    let t94909 = t27506 * t1193;
    let t94932 = t8034 * t24660;
    let t94936 = t8034 * t24667;
    let t94963 = t24847 * t974 * t64825;
    let t94966 = t85660 * t8067;
    let t95033 = t85660 * t8070;
    let t95092 = t27505 * t210 * t24848;
    (t94909, t94932, t94936, t94963, t94966, t95033, t95092)
}
