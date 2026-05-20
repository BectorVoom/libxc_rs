//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2315/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2315<F: Float>(t27598: F, t3535: F, t1218: F, t14731: F, t14736: F, t14740: F, t15663: F, t15750: F, t2121: F, t24736: F, t24741: F, t4899: F, t4989: F, t7331: F, t8040: F, t86204: F, t86324: F, t95410: F, t95415: F, t95424: F, t95435: F) -> F {
    let t95440 = t3535 * t27598;
    let t95443 = -t95410 - F::cast_from(0.10093189023535097714e-3_f64) * t86204 * t8040 + F::cast_from(0.20186378047070195428e-3_f64) * t95415 * t7331 + F::new(5.0) / F::new(3456.0) * t24736 * t4989 - t95424 + t2121 * t4899 * t14736 / F::new(108.0) + t2121 * t4899 * t14740 / F::new(216.0) + t2121 * t4899 * t14731 / F::new(36.0) - t95435 - t86324 * t15663 / F::new(576.0) + F::new(5.0) / F::new(3456.0) * t24741 * t15750 - t95440 * t1218 / F::new(144.0);
    t95443
}
