//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1185/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1185<F: Float>(t11285: F, t3377: F, t14853: F, t1164: F, t300: F, t4832: F, t1166: F, t3419: F, t4869: F, t11180: F, t1671: F, t3259: F, t4782: F) -> (F, F, F, F, F) {
    let t14854 = t11285 * t3377;
    let t14855 = t14853 * t14854;
    let t14857 = F::new(0.10254018858216406658e4) * t1164 * t14855;
    let t14858 = t300 * t4832;
    let t14860 = F::new(0.11696447245269292414e1) * t14858 * t1166;
    let t14862 = F::new(0.5848223622634646207e0) * t4869 * t3419;
    let t14864 = F::new(1.0) * t11180 * t1671;
    let t14866 = F::new(2.0) * t3259 * t4782;
    (t14857, t14860, t14862, t14864, t14866)
}
