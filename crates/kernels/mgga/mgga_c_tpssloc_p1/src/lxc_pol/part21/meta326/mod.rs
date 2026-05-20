//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1695;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1696;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta326<F: Float>(t12199: F, t3741: F, t3732: F, t792: F, t118: F, t3734: F, t794: F, t3719: F, t3739: F, t782: F, t3736: F, t1365: F, t154: F, t205: F, t116: F, t547: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12200, t12202, t12204, t12205, t12208, t12209, t12211) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1695::<F>(t12199, t3741, t3732, t792, t118, t3734, t794, t3719, t3739, t782);
        let (t12212, t12214, t12215, t12225) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1696::<F>(t12211, t3736, t1365, t154, t205, t116, t547);
    (t12200, t12202, t12204, t12205, t12208, t12209, t12211, t12212, t12214, t12215, t12225)
}
