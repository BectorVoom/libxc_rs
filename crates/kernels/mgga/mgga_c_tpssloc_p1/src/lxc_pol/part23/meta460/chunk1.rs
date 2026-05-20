//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1347/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1347<F: Float>(t10702: F, t2844: F, t76644: F, t1557: F, t68924: F, t17195: F, t5727: F, t5730: F, t59959: F, t21300: F, t4354: F, t1637: F, t4700: F, t68711: F, t76634: F, t76636: F, t76641: F, t76643: F, t76647: F, t76652: F, t76654: F) -> (F, F, F, F, F, F) {
    let t76657 = F::cast_from(0.57895126195293126241e3_f64) * t10702 * t76644 * t2844;
    let t76659 = F::new(4.0) * t68924 * t1557;
    let t76661 = F::new(6.0) * t17195 * t5727;
    let t76663 = F::cast_from(0.96491876992155210402e2_f64) * t59959 * t5730;
    let t76665 = F::new(4.0) * t4354 * t21300;
    let t76666 = -F::new(4.0) * t1637 * t4700 * t68711 + t76634 - t76636 - t76641 + t76643 + t76647 - t76652 - t76654 + t76657 + t76659 + t76661 + t76663 + t76665;
    (t76657, t76659, t76661, t76663, t76665, t76666)
}
