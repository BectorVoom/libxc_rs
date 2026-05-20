//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1088/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1088<F: Float>(t25: F, t265: F, t394: F, t202: F, t7109: F, t1877: F, t193: F, t2057: F, t2522: F, t7114: F, t776: F, t868: F, t870: F, t2064: F, t40: F, t606: F, t607: F, t6542: F, t6671: F, t7110: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t7125 = t202 * t7109;
    let t7130 = -t1877 * t7114 * t868 + t193 * t7125 * t870 + F::new(3.0) * t2057 * t2522 * t776;
    let t7131 = piecewise3::<F>(t395, F::new(0.0), t7130);
    let t7136 = piecewise3::<F>(t115, F::new(3.0) / F::new(2.0) * t2522 * t2057 * t6542 + t1877 * t7110 * t25 / F::new(2.0) - t1877 * t7114 * t6671 / F::new(2.0) + t1877 * t2057 * t606 / F::new(2.0), t2064 * t607 / F::new(2.0) + t7131 * t40 / F::new(2.0));
    (t7130, t7131, t7136)
}
