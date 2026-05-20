//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1784/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1784<F: Float>(t81598: F, t81735: F, t81742: F, t81849: F, t81852: F, t81920: F, t81954: F, t2627: F, t7084: F, t81688: F, t81716: F, t82046: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t84851 = F::cast_from(0.3244175520728446583e0_f64) * t81598;
    let t84857 = F::cast_from(0.13958506597733353653e-1_f64) * t81735;
    let t84859 = F::cast_from(0.87474304870637513515e-3_f64) * t81742;
    let t84896 = F::cast_from(0.2034786907144675699e0_f64) * t81849;
    let t84897 = F::new(455.0) / F::new(648.0) * t81852;
    let t84921 = F::new(595.0) / F::new(2592.0) * t81920;
    let t84932 = F::cast_from(0.67287926823567318088e-4_f64) * t81954;
    let t84962 = t2627 * t7084;
    let t84995 = F::cast_from(0.27415567780803773942e-2_f64) * t81688;
    let t85003 = F::cast_from(0.19739208802178717238e0_f64) * t81716;
    let t85027 = F::cast_from(0.55440370401180965083e0_f64) * t82046;
    (t84851, t84857, t84859, t84896, t84897, t84921, t84932, t84962, t84995, t85003, t85027)
}
