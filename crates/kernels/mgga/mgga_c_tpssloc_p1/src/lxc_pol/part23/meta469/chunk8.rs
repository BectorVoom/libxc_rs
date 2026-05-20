//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1394/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1394<F: Float>(t1615: F, t3188: F, t5872: F, t5914: F, t381: F, t76740: F, t11046: F, t11048: F, t11065: F, t1610: F, t1632: F, t21481: F, t21615: F, t21622: F, t21627: F, t21634: F, t21647: F, t3131: F, t3186: F, t3200: F, t3201: F, t43553: F, t43554: F, t4669: F, t47841: F, t5936: F, t77782: F, t77794: F) -> (F, F, F) {
    let t77806 = t3188 * t1615;
    let t77819 = t5914 * t5872;
    let t77826 = t381 * t76740;
    let t77835 = -F::new(36.0) * t11065 * t3131 * t5936 * t77794 + F::new(4.0) * t11046 * t11048 * t77782 - F::new(4.0) * t21622 * t21634 * t3200 + F::new(8.0) * t21634 * t3186 * t77806 + F::new(12.0) * t3186 * t3188 * t77819 - F::new(6.0) * t3200 * t3201 * t77819 - F::new(36.0) * t43553 * t43554 * t77826 + F::new(4.0) * t1610 * t21615 + F::new(4.0) * t1632 * t21481 + F::new(12.0) * t21627 * t4669 + F::new(24.0) * t21647 * t47841;
    (t77806, t77826, t77835)
}
