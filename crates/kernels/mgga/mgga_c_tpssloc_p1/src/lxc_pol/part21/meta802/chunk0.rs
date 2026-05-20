//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2789/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2789<F: Float>(t12940: F, t58994: F, t12606: F, t4194: F, t4195: F, t12908: F, t16713: F, t12939: F, t5392: F, t607: F, t750: F, t157: F, t4196: F, t46447: F) -> (F, F, F, F, F) {
    let t58996 = F::new(48.0) * t58994 * t12940;
    let t58999 = F::new(24.0) * t4194 * t4195 * t12606;
    let t59001 = F::new(48.0) * t12908 * t16713;
    let t59004 = t12939 * t750 * t5392 * t607;
    let t59005 = F::new(48.0) * t59004;
    let t59008 = F::new(48.0) * t46447 * t157 * t4196;
    (t58996, t58999, t59001, t59005, t59008)
}
