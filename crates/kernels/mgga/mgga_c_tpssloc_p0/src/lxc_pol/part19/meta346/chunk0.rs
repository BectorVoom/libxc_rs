//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1246/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1246<F: Float>(t2784: F, t2841: F, t2845: F, t10697: F, t2787: F, t10696: F, t2842: F, t2844: F, t912: F, t10702: F, t10704: F, t2793: F, t2836: F) -> (F, F, F, F) {
    let t41623 = t2784 * t2841;
    let t41625 = F::cast_from(0.96491876992155210402e2_f64) * t41623 * t2845;
    let t41627 = F::new(4.0) * t2787 * t10697;
    let t41635 = F::cast_from(0.64327917994770140268e2_f64) * t2842 * t10696 * t2844 * t912;
    let t41639 = F::cast_from(0.3103560775156404018e4_f64) * t10702 * t2793 * t10704 * t2836;
    (t41625, t41627, t41635, t41639)
}
