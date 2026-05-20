//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 823/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk823<F: Float>(t25: F, t265: F, t394: F, t29148: F, t1409: F, t2064: F, t29124: F, t40: F, t5398: F, t7865: F, t2057: F, t28764: F, t1649: F, t1877: F, t24191: F, t24344: F, t2522: F, t26744: F, t28: F, t28771: F, t28774: F, t28778: F, t28789: F, t28792: F, t28795: F, t29106: F, t4314: F, t5966: F, t7114: F, t7649: F, t7656: F, t7845: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t29149 = piecewise3::<F>(t395, F::new(0.0), t29148);
    let t29156 = piecewise3::<F>(t115, t29124, t29149 * t40 / F::new(2.0) + t7865 * t1409 + t2064 * t5398 / F::new(2.0));
    let t29157 = t2057 * t28764;
    let t29188 = F::new(3.0) * t4314 * t29157 + F::new(3.0) * t2522 * t7845 * t7649 - F::new(3.0) * t24191 * t28771 + F::new(3.0) * t2522 * t2057 * t28774 + F::new(3.0) / F::new(2.0) * t2522 * t2057 * t28778 + t1877 * t29106 * t28 / F::new(2.0) - t1877 * t26744 * t7656 + t1877 * t7845 * t1649 + t1877 * t24344 * t28789 - t1877 * t7114 * t28792 - t1877 * t7114 * t28795 / F::new(2.0) + t1877 * t2057 * t5966 / F::new(2.0);
    (t29156, t29188)
}
