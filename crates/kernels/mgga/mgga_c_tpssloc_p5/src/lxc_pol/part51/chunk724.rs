//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 724/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk724<F: Float>(t25: F, t265: F, t394: F, t1068: F, t1070: F, t193: F, t336: F, t4700: F, t6818: F, t6822: F, t6834: F, t1965: F, t40: F, t607: F, t6678: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t6835 = piecewise3::<F>(t395, t1070 * t193 * t336 * t6818 - t1068 * t4700 * t6822, t6834);
    let t6840 = piecewise3::<F>(t115, t6678, t1965 * t607 / F::cast_from(2.0_f64) + t6835 * t40 / F::cast_from(2.0_f64));
    (t6835, t6840)
}
