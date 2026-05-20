//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1587/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1587<F: Float>(t1329: F, t22797: F, t3770: F, t6916: F, t2230: F, t6924: F, t213: F, t6928: F, t1998: F, t236: F, t3719: F, t6926: F) -> (F, F, F, F, F, F, F, F) {
    let t22798 = t22797 * t1329;
    let t22799 = F::new(7.0) / F::new(72.0) * t22798;
    let t22800 = t6916 * t3770;
    let t22803 = t2230 * t6924;
    let t22804 = t22803 * t213;
    let t22805 = t22804 * t6928;
    let t22808 = t1998 * t236 * t3719;
    let t22809 = t6926 * t22808;
    (t22798, t22799, t22800, t22803, t22804, t22805, t22808, t22809)
}
