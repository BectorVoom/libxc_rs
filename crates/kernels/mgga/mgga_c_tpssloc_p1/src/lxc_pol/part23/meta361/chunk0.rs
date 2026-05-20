//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1160/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1160<F: Float>(t2402: F, t976: F, t10213: F, t135: F, t344: F, t41687: F, t41961: F, t697: F, t10216: F, t343: F, t10868: F, t820: F) -> (F, F, F, F, F, F, F) {
    let t42891 = t2402 * t976;
    let t42972 = t135 * t10213;
    let t42976 = t344 * t41687;
    let t43002 = F::new(220.0) / F::new(81.0) * t41961;
    let t43052 = t697 * t976;
    let t43070 = t343 * t10216;
    let t43198 = t820 * t10868;
    (t42891, t42972, t42976, t43002, t43052, t43070, t43198)
}
