//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1112/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1112<F: Float>(t28: F, t265: F, t504: F, t33043: F, t1409: F, t33073: F, t52: F, t8435: F, t33049: F, t15899: F, t8493: F, t1983: F, t1458: F, t1868: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t33074 = piecewise3::<f64>(t505, F::new(0.0), t33043);
    let t33079 = piecewise3::<f64>(t401, t33073, -t8435 * t1409 / F::new(2.0) + t33074 * t52 / F::new(2.0));
    let t33080 = t33049 + t33079;
    let t33082 = t8493 * t15899;
    let t33084 = F::new(2.0) * t1983 * t33082;
    let t33085 = t1868 * t1458;
    (t33074, t33080, t33082, t33084, t33085)
}
