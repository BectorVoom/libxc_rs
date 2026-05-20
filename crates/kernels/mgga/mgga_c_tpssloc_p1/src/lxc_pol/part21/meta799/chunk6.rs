//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2785/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2785<F: Float>(t39483: F, t40741: F, t40743: F, t40748: F, t40760: F, t57993: F, t57996: F, t58005: F, t58008: F, t58020: F, t58022: F, t58023: F, t58025: F, t58026: F, t58027: F, t58028: F, t58030: F, t58032: F, t58033: F, t58034: F) -> F {
    let t58967 = t57993 + t57996 + t58005 + t58008 + t39483 + t58020 - t58022 + t58023 - t58025 - t40741 - t40743 + t58026 + t58027 + t58028 + t40748 + t58030 + t58032 + t58033 + t40760 - t58034;
    t58967
}
