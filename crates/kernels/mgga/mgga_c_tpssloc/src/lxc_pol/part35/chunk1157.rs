//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1157/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1157<F: Float>(t10469: F, t461: F, t11715: F, t11721: F, t3032: F, t3502: F, t3508: F, t11553: F, t2121: F, t2148: F, t27561: F, t7327: F, t1209: F, t475: F, t210: F, t24810: F) -> (F, F, F, F, F, F, F, F, F) {
    let t85964 = t461 * t10469;
    let t85965 = t85964 * t11715;
    let t85966 = t3032 * t11721;
    let t85971 = t85964 * t3502;
    let t85972 = t3032 * t3508;
    let t86000 = 0.30461741978670859935e-2 * t2121 * t11553 * t2148;
    let t86015 = t7327 * t27561;
    let t86022 = t85964 * t1209;
    let t86023 = t3032 * t475;
    let t86036 = t24810 * t210;
    (t85965, t85966, t85971, t85972, t86000, t86015, t86022, t86023, t86036)
}
