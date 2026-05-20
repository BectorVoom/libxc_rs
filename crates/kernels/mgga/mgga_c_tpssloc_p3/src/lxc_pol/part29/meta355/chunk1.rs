//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1440/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1440<F: Float>(t52: F, t12606: F, t12874: F, t12877: F, t2244: F, t2250: F, t4087: F, t607: F, t76: F, t12873: F, t157: F, t182: F, t145: F, zeta_threshold: F) -> (F, F) {
    let t150 = t52 <= zeta_threshold;
    let t12885 = piecewise3::<F>(t150, F::new(0.0), F::new(8.0) / F::new(27.0) * t12874 * t2244 + F::new(8.0) / F::new(9.0) * t12877 * t607 + F::new(4.0) / F::new(9.0) * t4087 * t2250 - F::new(4.0) / F::new(3.0) * t76 * t12606);
    let t12886 = t12873 + t12885;
    let t12887 = t12886 * t157;
    let t12889 = F::cast_from(0.19751673498613801407e-1_f64) * t12887 * t182;
    let t12890 = t145 * t12886;
    (t12889, t12890)
}
