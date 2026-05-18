//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1032/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1032<F: Float>(t25742: F, t3174: F, t1054: F, t1634: F, t884: F, t23329: F, t225: F, t7594: F, t254: F, t382: F, t10164: F, t1955: F) -> (F, F, F, F, F, F) {
    let t25743 = t3174 * t25742;
    let t25749 = t1054 * t1634;
    let t25750 = t25749 * t884;
    let t25751 = t23329 * t25750;
    let t25755 = t7594 * t225;
    let t25757 = t382 * t254;
    let t25758 = t10164 * t1955;
    (t25743, t25749, t25751, t25755, t25757, t25758)
}
