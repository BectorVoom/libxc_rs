//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1185/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1185<F: Float>(t3787: F, t3879: F, t12248: F, t1372: F, t12169: F, t12171: F, t12178: F, t12244: F, t12255: F, t12259: F, t12260: F, t12435: F, t1332: F, t1336: F, t1352: F, t22694: F, t3773: F, t3777: F, t3851: F, t3856: F, t3901: F, t3909: F, t40453: F, t40475: F, t5344: F, t544: F, t553: F) -> (F, F, F) {
    let t40486 = t3787 * t3879;
    let t40492 = t12248 * t1372;
    let t40524 = F::new(24.0) * t12171 * t12255 * t1336 - F::new(4.0) * t12178 * t1336 * t3901 - F::new(6.0) * t12259 * t1336 * t3856 - F::new(4.0) * t1352 * t40475 * t5344 - F::new(12.0) * t22694 * t3851 * t5344 + t40453 * t544 * t553 - F::new(4.0) * t12169 * t3777 - F::new(12.0) * t12244 * t3777 - F::new(12.0) * t12260 * t3777 + F::new(4.0) * t12435 * t1332 + F::new(6.0) * t3773 * t3909;
    (t40486, t40492, t40524)
}
