//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1068/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1068<F: Float>(t118858: F, t1880: F, t25329: F, t6553: F, t6571: F, t112660: F, t7488: F, t112899: F, t22986: F, t25054: F, t118837: F, t118838: F, t118841: F, t118847: F, t118850: F, t118851: F, t1911: F, t259: F, t2597: F, t2718: F, t30647: F, t30651: F, t32800: F, t32849: F, t4268: F, t4300: F, t798: F, t8362: F, t855: F) -> (F,) {
    let t118859 = 0.38381794893125283518e-1 * t118858;
    let t118871 = 0.16449340668482264365e-1 * t1880 * t6553 * t6571 * t25329;
    let t118874 = 0.16449340668482264365e-1 * t1880 * t112660 * t7488;
    let t118877 = 0.3289868133696452873e-1 * t22986 * t112899 * t25054;
    let t118878 = 4.0 * t1911 * t25329 * t2718 * t855 + 2.0 * t2718 * t4300 * t8362 * t855 + t259 * t32849 * t798 + 4.0 * t2597 * t32800 + 2.0 * t30647 * t4268 - 6.0 * t30651 * t4268 - t118837 - t118838 - t118841 + t118847 - t118850 - t118851 + t118859 - t118871 - t118874 + t118877;
    (t118878,)
}
