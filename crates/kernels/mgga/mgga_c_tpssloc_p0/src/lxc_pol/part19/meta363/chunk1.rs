//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1324/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1324<F: Float>(t10526: F, t2940: F, t10623: F, t2948: F, t10709: F, t2944: F, t10632: F, t2924: F, t10629: F, t2906: F, t959: F, t10523: F, t10723: F) -> (F, F, F, F, F, F) {
    let t42663 = F::cast_from(0.4155806185363551302e3_f64) * t2940 * t10526;
    let t42665 = F::cast_from(0.35089341735807877242e1_f64) * t10623 * t2948;
    let t42667 = F::cast_from(0.14035736694323150897e2_f64) * t2940 * t10709;
    let t42669 = F::cast_from(0.70178683471615754484e1_f64) * t10623 * t2944;
    let t42671 = t10632 * t2924;
    let t42674 = F::cast_from(0.61524113149298439947e4_f64) * t959 * t10629 * t2906 * t42671;
    let t42678 = F::cast_from(0.62337092780453269531e3_f64) * t959 * t10523 * t2906 * t10723;
    (t42663, t42665, t42667, t42669, t42674, t42678)
}
