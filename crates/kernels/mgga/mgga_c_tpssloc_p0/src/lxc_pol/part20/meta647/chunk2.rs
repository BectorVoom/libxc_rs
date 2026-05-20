//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2377/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2377<F: Float>(t10756: F, t1580: F, t2930: F, t10717: F, t10720: F, t10744: F, t14271: F, t42671: F, t47798: F, t47802: F, t48725: F, t48730: F, t48732: F, t48734: F, t48736: F, t48738: F, t48741: F, t48744: F, t48771: F, t48776: F, t933: F, t950: F) -> F {
    let t48779 = t10756 * t1580;
    let t48783 = t2930 * t1580;
    let t48786 = F::new(3.0) * t48771 * t933 + F::new(6.0) * t14271 * t10744 - F::cast_from(0.57895126195293126243e3_f64) * t48776 * t10717 + F::cast_from(0.30762056574649219974e4_f64) * t48779 * t42671 * t950 + F::cast_from(0.10526802520742363173e2_f64) * t48783 * t10720 - t47798 - t47802 + t48725 + t48730 + t48732 + t48734 - t48736 - t48738 + t48741 + t48744;
    t48786
}
