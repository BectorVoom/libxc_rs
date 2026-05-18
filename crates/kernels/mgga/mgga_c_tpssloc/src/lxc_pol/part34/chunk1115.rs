//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1115/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1115<F: Float>(t12248: F, t2085: F, t81317: F, t81398: F, t2056: F, t40772: F, t82069: F, t81598: F, t81735: F, t81742: F, t81849: F, t81852: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t84627 = t12248 * t2085;
    let t84659 = F::new(0.55440370401180965083e0) * t81317;
    let t84705 = F::new(0.27415567780803773942e-2) * t81398;
    let t84766 = t2056 * t40772;
    let t84820 = F::new(0.19739208802178717238e0) * t82069;
    let t84851 = F::new(0.3244175520728446583e0) * t81598;
    let t84857 = F::new(0.13958506597733353653e-1) * t81735;
    let t84859 = F::new(0.87474304870637513515e-3) * t81742;
    let t84896 = F::new(0.2034786907144675699e0) * t81849;
    let t84897 = F::new(455.0) / F::new(648.0) * t81852;
    (t84627, t84659, t84705, t84766, t84820, t84851, t84857, t84859, t84896, t84897)
}
