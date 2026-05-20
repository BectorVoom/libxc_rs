//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta336 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1415;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1416;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta336<F: Float>(t3749: F, t9577: F, t3726: F, t3745: F, t1314: F, t2566: F, t3741: F, t3732: F, t792: F, t118: F, t3734: F, t794: F, t3719: F, t3739: F, t782: F, t3736: F, t1365: F, t154: F, t205: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12196, t12197, t12199, t12200, t12202, t12204) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1415::<F>(t3749, t9577, t3726, t3745, t1314, t2566, t3741, t3732, t792, t118, t3734, t794);
        let (t12205, t12209, t12211, t12212, t12214, t12215) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1416::<F>(t12202, t12204, t118, t3719, t794, t3739, t3732, t782, t3736, t1365, t154, t205);
    (t12196, t12197, t12199, t12200, t12205, t12209, t12211, t12212, t12214, t12215)
}
