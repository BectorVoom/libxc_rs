//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1143/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1143<F: Float>(t1156: F, t18785: F, t11297: F, t1148: F, t18676: F, t18679: F, t18682: F, t18685: F, t18688: F, t18690: F, t18692: F, t18694: F, t18696: F, t18711: F, t3371: F, t6069: F, t6085: F) -> F {
    let t18786 = t18785 * t1156;
    let t18789 = -t18676 - t18679 + t18682 + t18685 - t18688 - t18690 - t18692 + t18694 - t18696 - F::new(0.19751673498613801407e-1) * t18711 - F::new(0.11696447245269292414e1) * t11297 * t6069 + F::new(0.5848223622634646207e0) * t3371 * t6085 + F::new(0.5848223622634646207e0) * t1148 * t18786;
    t18789
}
