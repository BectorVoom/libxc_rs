//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2403/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2403<F: Float>(t324: F, t68736: F, t68756: F, t300: F, t1557: F, t59979: F, t17195: F, t4396: F, t1068: F, t25845: F, t4700: F, t60874: F, t68441: F, t68706: F, t68708: F, t68710: F, t68711: F, t68715: F, t68717: F) -> (F, F, F, F, F) {
    let t68758 = (t68736 + t68756) * t324;
    let t68760 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t68758;
    let t68762 = F::new(3.0) * t59979 * t1557;
    let t68764 = F::new(3.0) * t17195 * t4396;
    let t68765 = -t1068 * t4700 * t68711 + F::new(6.0) * t25845 * t4700 * t60874 - t68441 - t68706 + t68708 - t68710 - t68715 - t68717 + t68760 + t68762 + t68764;
    (t68758, t68760, t68762, t68764, t68765)
}
