//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1199/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1199<F: Float>(t118837: F, t118838: F, t118841: F, t118847: F, t118850: F, t118851: F, t118859: F, t118871: F, t118874: F, t118877: F, t1911: F, t25329: F, t259: F, t2597: F, t2718: F, t30647: F, t30651: F, t32800: F, t32849: F, t4268: F, t4300: F, t798: F, t8362: F, t855: F) -> F {
    let t118878 = F::cast_from(4.0_f64) * t1911 * t25329 * t2718 * t855 + F::cast_from(2.0_f64) * t2718 * t4300 * t8362 * t855 + t259 * t32849 * t798 + F::cast_from(4.0_f64) * t2597 * t32800 + F::cast_from(2.0_f64) * t30647 * t4268 - F::cast_from(6.0_f64) * t30651 * t4268 - t118837 - t118838 - t118841 + t118847 - t118850 - t118851 + t118859 - t118871 - t118874 + t118877;
    t118878
}
