//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 719/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk719<F: Float>(t25: F, t265: F, t394: F, t2165: F, t671: F, t6834: F, t2116: F, t40: F, t607: F, t6678: F, t1170: F, t2123: F, t2121: F, t2127: F, t6686: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t7271 = t2165 * t671;
    let t7274 = piecewise3::<F>(t395, F::new(0.0), t6834);
    let t7279 = piecewise3::<F>(t115, t6678, t2116 * t607 / F::new(2.0) + t7274 * t40 / F::new(2.0));
    let t7280 = t1170 * t2123;
    let t7282 = F::cast_from(0.27415567780803773942e-2_f64) * t2121 * t7280;
    let t7283 = t2127 * t6686;
    (t7271, t7274, t7279, t7280, t7282, t7283)
}
