//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 906/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk906<F: Float>(t40: F, t52: F, t12606: F, t12862: F, t12865: F, t2244: F, t2250: F, t4080: F, t607: F, t73: F, t1409: F, t9438: F, t2440: F, t3966: F, t4087: F, t76: F, t157: F, t182: F, zeta_threshold: F) -> (F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t12873 = piecewise3(t146, 0.0, -8.0 / 27.0 * t12862 * t2244 + 8.0 / 9.0 * t12865 * t607 + 4.0 / 9.0 * t4080 * t2250 + 4.0 / 3.0 * t73 * t12606);
    let t12874 = t9438 * t1409;
    let t12877 = t2440 * t3966;
    let t12885 = piecewise3(t150, 0.0, 8.0 / 27.0 * t12874 * t2244 + 8.0 / 9.0 * t12877 * t607 + 4.0 / 9.0 * t4087 * t2250 - 4.0 / 3.0 * t76 * t12606);
    let t12886 = t12873 + t12885;
    let t12887 = t12886 * t157;
    let t12889 = 0.19751673498613801407e-1 * t12887 * t182;
    (t12886, t12889)
}
