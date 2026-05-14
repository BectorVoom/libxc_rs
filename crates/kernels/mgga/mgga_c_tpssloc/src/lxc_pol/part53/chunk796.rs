//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 796/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk796<F: Float>(t25: F, t265: F, t394: F, t1877: F, t193: F, t202: F, t2522: F, t32029: F, t32034: F, t32047: F, t7109: F, t7114: F, t776: F, t868: F, t870: F, t8744: F, t8748: F, t32030: F, t32044: F, t40: F, t606: F, t607: F, t6542: F, t6671: F, t8760: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t32071 = t193 * t202 * t32029 * t870 - t1877 * t32034 * t868 + 2.0 * t1877 * t32047 * t868 - 2.0 * t1877 * t7109 * t7114 + 3.0 * t2522 * t776 * t8744 - 3.0 * t2522 * t776 * t8748;
    let t32072 = piecewise3(t395, 0.0, t32071);
    let t32077 = piecewise3(t115, 3.0 / 2.0 * t2522 * t8744 * t6542 + t1877 * t32030 * t25 / 2.0 - t1877 * t32034 * t6671 / 2.0 + t1877 * t8744 * t606 / 2.0 - 3.0 / 2.0 * t2522 * t8748 * t6542 - t1877 * t7114 * t32044 + t1877 * t32047 * t6671 - t1877 * t8748 * t606 / 2.0, t32072 * t40 / 2.0 + t8760 * t607 / 2.0);
    (t32071, t32072, t32077)
}
